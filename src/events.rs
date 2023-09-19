use actix_web::HttpResponse;
use chrono::{DateTime, Utc};
use deadpool_postgres::Pool;
use stripe::{
    CheckoutSession, Event, EventObject, Expandable, Invoice,
    Subscription as StripeSubscription,
};

use crate::model::Subscription;
use crate::{DbError, HttpError, MediaService};

#[derive(Debug, Clone)]
pub struct EventService {
    pool: Pool,
    media_service: MediaService,
}

impl EventService {
    const METADATA_KEY_USER_ID: &str = "user_id";
    const METADATA_KEY_OFFER_ID: &str = "offer_id";

    pub fn new(pool: Pool, media_service: MediaService) -> Self {
        Self {
            pool,
            media_service,
        }
    }

    fn unexpected_object(event: &Event) -> HttpError {
        tracing::log::error!("Event: {:?}", event);
        HttpError::bad_request(format!(
            "Got {} event with unexpected object. Event: {:?}",
            event.type_, event
        ))
    }

    async fn send_updated_subscription(
        &self,
        subscription: Subscription,
    ) -> Result<(), HttpError> {
        let Subscription {
            subscription_id,
            buyer_user_id,
            offer_id,
            current_period_start,
            current_period_end,
            subscription_status,
            payed_at,
            ..
        } = subscription;

        if let (
            Some(buyer_user_id),
            Some(offer_id),
            Some(current_period_start),
            Some(current_period_end),
            Some(subscription_status),
            Some(payed_at),
        ) = (
            buyer_user_id,
            offer_id,
            current_period_start,
            current_period_end,
            subscription_status,
            payed_at,
        ) {
            self.media_service
                .put_media_subscription(
                    subscription_id.to_string(),
                    buyer_user_id,
                    offer_id.to_string(),
                    current_period_start.timestamp().try_into().unwrap(),
                    current_period_end.timestamp().try_into().unwrap(),
                    subscription_status,
                    payed_at.timestamp().try_into().unwrap(),
                )
                .await
                .map_err(|err| {
                    tracing::log::error!(
                        "[EventService.send_updated_subscription] {err}"
                    );
                    HttpError::internal()
                })?;
        }

        Ok(())
    }

    async fn handle_checkout_session(
        &self,
        checkout_session: CheckoutSession,
    ) -> Result<HttpResponse, HttpError> {
        if let Some(stripe_subscription) = checkout_session.subscription {
            let buyer_user_id = checkout_session
                .metadata
                .get(Self::METADATA_KEY_USER_ID)
                .cloned();

            let offer_id = checkout_session
                .metadata
                .get(Self::METADATA_KEY_OFFER_ID)
                .and_then(|id| id.parse().ok());

            let mut current_period_start = None;
            let mut current_period_end = None;
            let mut subscription_status = None;

            let stripe_subscription_id = match stripe_subscription {
                Expandable::Id(id) => id.to_string(),
                Expandable::Object(s) => {
                    current_period_start = DateTime::<Utc>::from_timestamp(
                        s.current_period_start,
                        0,
                    );
                    current_period_end = DateTime::<Utc>::from_timestamp(
                        s.current_period_end,
                        0,
                    );
                    subscription_status = Some(s.status.to_string());

                    s.id.to_string()
                }
            };

            let mut conn = self.pool.get().await.map_err(DbError::from)?;
            let transaction =
                conn.transaction().await.map_err(DbError::from)?;

            let found_subscription =
                Subscription::get(&transaction, &stripe_subscription_id)
                    .await?;

            let updated_subscription = if found_subscription.is_some() {
                Subscription::update(
                    &transaction,
                    stripe_subscription_id,
                    buyer_user_id,
                    offer_id,
                    current_period_start,
                    current_period_end,
                    subscription_status,
                )
                .await?
            } else {
                Subscription::create(
                    &transaction,
                    stripe_subscription_id,
                    buyer_user_id,
                    offer_id,
                    current_period_start,
                    current_period_end,
                    subscription_status,
                    None,
                )
                .await?
            };

            transaction.commit().await.map_err(DbError::from)?;

            self.send_updated_subscription(updated_subscription).await?;
        }

        Ok(HttpResponse::Ok().finish())
    }

    async fn handle_subscription(
        &self,
        subscription: StripeSubscription,
    ) -> Result<HttpResponse, HttpError> {
        let stripe_subscription_id = subscription.id.to_string();
        let current_period_start = DateTime::<Utc>::from_timestamp(
            subscription.current_period_start,
            0,
        );
        let current_period_end =
            DateTime::<Utc>::from_timestamp(subscription.current_period_end, 0);
        let subscription_status = Some(subscription.status.to_string());

        let mut conn = self.pool.get().await.map_err(DbError::from)?;
        let transaction = conn.transaction().await.map_err(DbError::from)?;

        let found_subscription =
            Subscription::get(&transaction, &stripe_subscription_id).await?;

        let updated_subscription = if found_subscription.is_some() {
            Subscription::update(
                &transaction,
                stripe_subscription_id,
                None,
                None,
                current_period_start,
                current_period_end,
                subscription_status,
            )
            .await?
        } else {
            Subscription::create(
                &transaction,
                stripe_subscription_id,
                None,
                None,
                current_period_start,
                current_period_end,
                subscription_status,
                None,
            )
            .await?
        };

        transaction.commit().await.map_err(DbError::from)?;

        self.send_updated_subscription(updated_subscription).await?;

        Ok(HttpResponse::Ok().finish())
    }

    async fn handle_invoice(
        &self,
        invoice: Invoice,
    ) -> Result<HttpResponse, HttpError> {
        if let Some(stripe_subscription) = invoice.subscription {
            let stripe_subscription_id = match stripe_subscription {
                Expandable::Id(id) => id.to_string(),
                Expandable::Object(s) => s.id.to_string(),
            };

            let payed_at = match invoice.status_transitions.and_then(|s| {
                s.paid_at
                    .and_then(|p| DateTime::<Utc>::from_timestamp(p, 0))
            }) {
                Some(p) => p,
                None => return Ok(HttpResponse::Ok().finish()),
            };

            let mut conn = self.pool.get().await.map_err(DbError::from)?;
            let transaction =
                conn.transaction().await.map_err(DbError::from)?;

            let found_subscription =
                Subscription::get(&transaction, &stripe_subscription_id)
                    .await?;

            let updated_subscription =
                if let Some(found_subscription) = found_subscription {
                    // if saved payed_at is further in time than timestamp from event, return early
                    if matches!(
                        found_subscription.payed_at,
                        Some(old_payed_at)
                        if old_payed_at > payed_at
                    ) {
                        return Ok(HttpResponse::Ok().finish());
                    }

                    Subscription::update_payed_at(
                        &transaction,
                        &stripe_subscription_id,
                        payed_at,
                    )
                    .await?
                } else {
                    Subscription::create(
                        &transaction,
                        stripe_subscription_id,
                        None,
                        None,
                        None,
                        None,
                        None,
                        Some(payed_at),
                    )
                    .await?
                };

            transaction.commit().await.map_err(DbError::from)?;

            self.send_updated_subscription(updated_subscription).await?;
        }
        Ok(HttpResponse::Ok().finish())
    }

    pub async fn handle_event(
        &self,
        event: Event,
    ) -> Result<HttpResponse, HttpError> {
        use stripe::EventType::*;

        match event.type_ {
            CheckoutSessionCompleted => {
                if let EventObject::CheckoutSession(checkout_session) =
                    event.data.object
                {
                    self.handle_checkout_session(checkout_session).await
                } else {
                    Err(Self::unexpected_object(&event))
                }
            }
            CustomerSubscriptionCreated | CustomerSubscriptionUpdated => {
                if let EventObject::Subscription(subscription) =
                    event.data.object
                {
                    self.handle_subscription(subscription).await
                } else {
                    Err(Self::unexpected_object(&event))
                }
            }
            InvoicePaid => {
                if let EventObject::Invoice(invoice) = event.data.object {
                    self.handle_invoice(invoice).await
                } else {
                    Err(Self::unexpected_object(&event))
                }
            }
            _ => {
                tracing::log::error!(
                    "Unexpected event, will respond OK to stripe. Event: {:?}",
                    event
                );
                Ok(HttpResponse::Ok().finish())
            }
        }
    }
}
