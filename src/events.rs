use actix_web::HttpResponse;
use chrono::{DateTime, Utc};
use deadpool_postgres::Pool;
use stripe::{
    CheckoutSession, Event, EventObject, Invoice,
    Subscription as StripeSubscription,
};

use crate::api::sited_io::media::v1::MediaSubscriptionResponse;
use crate::model::Subscription;
use crate::{DbError, HttpError, Publisher};

#[derive(Debug, Clone)]
pub struct EventService {
    pool: Pool,
    publisher: Publisher,
}

impl EventService {
    const METADATA_KEY_USER_ID: &'static str = "user_id";
    const METADATA_KEY_OFFER_ID: &'static str = "offer_id";
    const METADATA_KEY_SHOP_ID: &'static str = "shop_id";

    pub fn new(pool: Pool, publisher: Publisher) -> Self {
        Self { pool, publisher }
    }

    fn unexpected_object(event: &Event) -> HttpError {
        tracing::error!("Event: {:?}", event);
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
            stripe_subscription_id,
            buyer_user_id,
            offer_id,
            shop_id,
            current_period_start,
            current_period_end,
            subscription_status,
            payed_at,
            payed_until,
            created_at,
            updated_at,
            canceled_at,
            cancel_at,
            event_timestamp,
        } = subscription;

        // These fields are destructured here, in order to get an compiler error,
        // when we add new fields to the Subscription struct and do not handle them.
        #[allow(unused_variables, clippy::no_effect)]
        (created_at, updated_at, event_timestamp);

        if let (
            Some(buyer_user_id),
            Some(offer_id),
            Some(shop_id),
            Some(current_period_start),
            Some(current_period_end),
            Some(subscription_status),
            Some(payed_at),
            Some(payed_until),
        ) = (
            buyer_user_id,
            offer_id,
            shop_id,
            current_period_start,
            current_period_end,
            subscription_status,
            payed_at,
            payed_until,
        ) {
            self.publisher
                .publish_subscription_upsert(&MediaSubscriptionResponse {
                    media_subscription_id: subscription_id.to_string(),
                    buyer_user_id: buyer_user_id.clone(),
                    shop_id: shop_id.to_string(),
                    offer_id: offer_id.to_string(),
                    current_period_start: current_period_start
                        .timestamp()
                        .try_into()
                        .unwrap(),
                    current_period_end: current_period_end
                        .timestamp()
                        .try_into()
                        .unwrap(),
                    subscription_status: subscription_status.clone(),
                    payed_at: payed_at.timestamp().try_into().unwrap(),
                    payed_until: payed_until.timestamp().try_into().unwrap(),
                    stripe_subscription_id: Some(
                        stripe_subscription_id.clone(),
                    ),
                    canceled_at: canceled_at
                        .map(|t| t.timestamp().try_into().unwrap()),
                    cancel_at: cancel_at
                        .map(|t| t.timestamp().try_into().unwrap()),
                })
                .await;

            tracing::info!("[EventService.send_updated_subscription] Sucessfully sent subscription to media api");
        }

        Ok(())
    }

    async fn handle_checkout_session(
        &self,
        checkout_session: CheckoutSession,
    ) -> Result<HttpResponse, HttpError> {
        if let (Some(stripe_subscription), Some(metadata)) =
            (checkout_session.subscription, checkout_session.metadata)
        {
            if let (Some(buyer_user_id), Some(offer_id), Some(shop_id)) = (
                metadata.get(Self::METADATA_KEY_USER_ID),
                metadata
                    .get(Self::METADATA_KEY_OFFER_ID)
                    .and_then(|id| id.parse().ok()),
                metadata
                    .get(Self::METADATA_KEY_SHOP_ID)
                    .and_then(|id| id.parse().ok()),
            ) {
                let stripe_subscription_id =
                    stripe_subscription.id().to_string();

                let updated_subscription = Subscription::put_checkout_session(
                    &self.pool,
                    &stripe_subscription_id,
                    buyer_user_id,
                    &offer_id,
                    &shop_id,
                    checkout_session.created,
                )
                .await?;

                self.send_updated_subscription(updated_subscription).await?;
            }
        }

        Ok(HttpResponse::Ok().finish())
    }

    async fn handle_subscription(
        &self,
        subscription: StripeSubscription,
        event_timestamp: i64,
    ) -> Result<HttpResponse, HttpError> {
        let stripe_subscription_id = subscription.id.to_string();

        let mut conn = self.pool.get().await.map_err(DbError::from)?;
        let transaction = conn.transaction().await.map_err(DbError::from)?;

        let found_subscription =
            Subscription::get(&transaction, &stripe_subscription_id).await?;

        let mut update = false;

        match found_subscription {
            Some(found_subscription) => {
                if found_subscription.event_timestamp < event_timestamp {
                    update = true;
                }

                if let Some(metadata_user_id) =
                    subscription.metadata.get(Self::METADATA_KEY_USER_ID)
                {
                    if found_subscription
                        .buyer_user_id
                        .is_some_and(|user_id| user_id != *metadata_user_id)
                    {
                        Subscription::update_buyer_user_id(
                            &transaction,
                            &stripe_subscription_id,
                            metadata_user_id,
                        )
                        .await?;
                    }
                }
            }
            None => {
                update = true;
            }
        }

        if update {
            let current_period_start = DateTime::<Utc>::from_timestamp(
                subscription.current_period_start,
                0,
            )
            .unwrap();

            let current_period_end = DateTime::<Utc>::from_timestamp(
                subscription.current_period_end,
                0,
            )
            .unwrap();

            let canceled_at = subscription
                .canceled_at
                .and_then(|c| DateTime::<Utc>::from_timestamp(c, 0));

            let cancel_at = subscription
                .cancel_at
                .and_then(|c| DateTime::<Utc>::from_timestamp(c, 0));

            let updated_subscription = Subscription::put_subscription(
                &transaction,
                &stripe_subscription_id,
                &current_period_start,
                &current_period_end,
                &subscription.status.to_string(),
                canceled_at,
                cancel_at,
                subscription.created,
            )
            .await?;

            self.send_updated_subscription(updated_subscription).await?;
        }

        transaction.commit().await.map_err(DbError::from)?;

        Ok(HttpResponse::Ok().finish())
    }

    async fn handle_invoice(
        &self,
        invoice: Invoice,
    ) -> Result<HttpResponse, HttpError> {
        if let Some(lines) = invoice.lines {
            for line in lines.data {
                if let Some(stripe_subscription) = line.subscription {
                    if let (Some(payed_at), Some(payed_until)) = (
                        line.period.as_ref().and_then(|p| p.start).and_then(
                            |p| DateTime::<Utc>::from_timestamp(p, 0),
                        ),
                        line.period.and_then(|p| p.end).and_then(|p| {
                            DateTime::<Utc>::from_timestamp(p, 0)
                        }),
                    ) {
                        let stripe_subscription_id =
                            stripe_subscription.id().to_string();

                        let updated_subscription = Subscription::put_invoice(
                            &self.pool,
                            &stripe_subscription_id,
                            &payed_at,
                            &payed_until,
                            invoice.created.unwrap_or(0),
                        )
                        .await?;

                        self.send_updated_subscription(updated_subscription)
                            .await?;
                    }
                }
            }
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
            CustomerSubscriptionResumed
            | CustomerSubscriptionPendingUpdateExpired
            | CustomerSubscriptionPendingUpdateApplied
            | CustomerSubscriptionPaused
            | CustomerSubscriptionDeleted
            | CustomerSubscriptionTrialWillEnd
            | CustomerSubscriptionCreated
            | CustomerSubscriptionUpdated => {
                if let EventObject::Subscription(subscription) =
                    event.data.object
                {
                    self.handle_subscription(subscription, event.created).await
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
                tracing::error!(
                    "Unexpected event, will respond OK to stripe. Event: {:?}",
                    event
                );
                Ok(HttpResponse::Ok().finish())
            }
        }
    }
}
