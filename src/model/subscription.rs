use chrono::{DateTime, Utc};
use deadpool_postgres::tokio_postgres::Row;
use deadpool_postgres::Pool;
use sea_query::{Iden, OnConflict, PostgresQueryBuilder, Query};
use sea_query_postgres::PostgresBinder;
use uuid::Uuid;

use crate::DbError;

#[derive(Debug, Clone, Iden)]
#[iden(rename = "subscriptions")]
enum SubscriptionIden {
    Table,
    SubscriptionId,
    StripeSubscriptionId,
    BuyerUserId,
    OfferId,
    CurrentPeriodStart,
    CurrentPeriodEnd,
    SubscriptionStatus,
    PayedAt,
    PayedUntil,
    CreatedAt,
    UpdatedAt,
}

#[derive(Debug, Clone)]
pub struct Subscription {
    pub subscription_id: Uuid,
    pub stripe_subscription_id: String,
    pub buyer_user_id: Option<String>,
    pub offer_id: Option<Uuid>,
    pub current_period_start: Option<DateTime<Utc>>,
    pub current_period_end: Option<DateTime<Utc>>,
    pub subscription_status: Option<String>,
    pub payed_at: Option<DateTime<Utc>>,
    pub payed_until: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Subscription {
    const PUT_CHECKOUT_SESSION_COLUMNS: [SubscriptionIden; 3] = [
        SubscriptionIden::StripeSubscriptionId,
        SubscriptionIden::BuyerUserId,
        SubscriptionIden::OfferId,
    ];

    const PUT_SUBSCRIPTION_COLUMNS: [SubscriptionIden; 4] = [
        SubscriptionIden::StripeSubscriptionId,
        SubscriptionIden::CurrentPeriodStart,
        SubscriptionIden::CurrentPeriodEnd,
        SubscriptionIden::SubscriptionStatus,
    ];

    const PUT_INVOICE_COLUMNS: [SubscriptionIden; 3] = [
        SubscriptionIden::StripeSubscriptionId,
        SubscriptionIden::PayedAt,
        SubscriptionIden::PayedUntil,
    ];

    pub async fn put_checkout_session(
        pool: &Pool,
        stripe_subscription_id: &String,
        buyer_user_id: &String,
        offer_id: &Uuid,
    ) -> Result<Self, DbError> {
        let conn = pool.get().await?;

        let (sql, values) = Query::insert()
            .into_table(SubscriptionIden::Table)
            .columns(Self::PUT_CHECKOUT_SESSION_COLUMNS)
            .values([
                stripe_subscription_id.into(),
                buyer_user_id.into(),
                (*offer_id).into(),
            ])?
            .on_conflict(
                OnConflict::column(SubscriptionIden::StripeSubscriptionId)
                    .update_columns(Self::PUT_CHECKOUT_SESSION_COLUMNS)
                    .to_owned(),
            )
            .returning_all()
            .build_postgres(PostgresQueryBuilder);

        let row = conn.query_one(sql.as_str(), &values.as_params()).await?;

        Ok(Self::from(row))
    }

    pub async fn put_subscription(
        pool: &Pool,
        stripe_subscription_id: &String,
        current_period_start: &DateTime<Utc>,
        current_period_end: &DateTime<Utc>,
        subscription_status: &String,
    ) -> Result<Self, DbError> {
        let conn = pool.get().await?;

        let (sql, values) = Query::insert()
            .into_table(SubscriptionIden::Table)
            .columns(Self::PUT_SUBSCRIPTION_COLUMNS)
            .values([
                stripe_subscription_id.into(),
                (*current_period_start).into(),
                (*current_period_end).into(),
                subscription_status.into(),
            ])?
            .on_conflict(
                OnConflict::column(SubscriptionIden::StripeSubscriptionId)
                    .update_columns(Self::PUT_SUBSCRIPTION_COLUMNS)
                    .to_owned(),
            )
            .returning_all()
            .build_postgres(PostgresQueryBuilder);

        let row = conn.query_one(sql.as_str(), &values.as_params()).await?;

        Ok(Self::from(row))
    }

    pub async fn put_invoice(
        pool: &Pool,
        stripe_subscription_id: &String,
        payed_at: &DateTime<Utc>,
        payed_until: &DateTime<Utc>,
    ) -> Result<Self, DbError> {
        let conn = pool.get().await?;

        let (sql, values) = Query::insert()
            .into_table(SubscriptionIden::Table)
            .columns(Self::PUT_INVOICE_COLUMNS)
            .values([
                stripe_subscription_id.into(),
                (*payed_at).into(),
                (*payed_until).into(),
            ])?
            .on_conflict(
                OnConflict::column(SubscriptionIden::StripeSubscriptionId)
                    .update_columns(Self::PUT_INVOICE_COLUMNS)
                    .to_owned(),
            )
            .returning_all()
            .build_postgres(PostgresQueryBuilder);

        let row = conn.query_one(sql.as_str(), &values.as_params()).await?;

        Ok(Self::from(row))
    }
}

impl From<Row> for Subscription {
    fn from(row: Row) -> Self {
        Self {
            subscription_id: row
                .get(SubscriptionIden::SubscriptionId.to_string().as_str()),
            stripe_subscription_id: row.get(
                SubscriptionIden::StripeSubscriptionId.to_string().as_str(),
            ),
            buyer_user_id: row
                .get(SubscriptionIden::BuyerUserId.to_string().as_str()),
            offer_id: row.get(SubscriptionIden::OfferId.to_string().as_str()),
            current_period_start: row
                .get(SubscriptionIden::CurrentPeriodStart.to_string().as_str()),
            current_period_end: row
                .get(SubscriptionIden::CurrentPeriodEnd.to_string().as_str()),
            subscription_status: row
                .get(SubscriptionIden::SubscriptionStatus.to_string().as_str()),
            payed_at: row.get(SubscriptionIden::PayedAt.to_string().as_str()),
            payed_until: row
                .get(SubscriptionIden::PayedUntil.to_string().as_str()),
            created_at: row
                .get(SubscriptionIden::CreatedAt.to_string().as_str()),
            updated_at: row
                .get(SubscriptionIden::UpdatedAt.to_string().as_str()),
        }
    }
}
