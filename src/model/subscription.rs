use chrono::{DateTime, Utc};
use deadpool_postgres::tokio_postgres::Row;
use deadpool_postgres::{GenericClient, Pool, Transaction};
use sea_query::{
    Asterisk, Expr, Iden, OnConflict, PostgresQueryBuilder, Query,
};
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
    ShopId,
    CurrentPeriodStart,
    CurrentPeriodEnd,
    SubscriptionStatus,
    PayedAt,
    PayedUntil,
    CreatedAt,
    UpdatedAt,
    CanceledAt,
    CancelAt,
    EventTimestamp,
}

#[derive(Debug, Clone)]
pub struct Subscription {
    pub subscription_id: Uuid,
    pub stripe_subscription_id: String,
    pub buyer_user_id: Option<String>,
    pub offer_id: Option<Uuid>,
    pub shop_id: Option<Uuid>,
    pub current_period_start: Option<DateTime<Utc>>,
    pub current_period_end: Option<DateTime<Utc>>,
    pub subscription_status: Option<String>,
    pub payed_at: Option<DateTime<Utc>>,
    pub payed_until: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub canceled_at: Option<DateTime<Utc>>,
    pub cancel_at: Option<DateTime<Utc>>,
    pub event_timestamp: i64,
}

impl Subscription {
    const PUT_CHECKOUT_SESSION_COLUMNS: [SubscriptionIden; 5] = [
        SubscriptionIden::StripeSubscriptionId,
        SubscriptionIden::BuyerUserId,
        SubscriptionIden::OfferId,
        SubscriptionIden::ShopId,
        SubscriptionIden::EventTimestamp,
    ];

    const PUT_SUBSCRIPTION_COLUMNS: [SubscriptionIden; 7] = [
        SubscriptionIden::StripeSubscriptionId,
        SubscriptionIden::CurrentPeriodStart,
        SubscriptionIden::CurrentPeriodEnd,
        SubscriptionIden::SubscriptionStatus,
        SubscriptionIden::CanceledAt,
        SubscriptionIden::CancelAt,
        SubscriptionIden::EventTimestamp,
    ];

    const PUT_INVOICE_COLUMNS: [SubscriptionIden; 4] = [
        SubscriptionIden::StripeSubscriptionId,
        SubscriptionIden::PayedAt,
        SubscriptionIden::PayedUntil,
        SubscriptionIden::EventTimestamp,
    ];

    pub async fn get<'a>(
        conn: &Transaction<'a>,
        stripe_subscription_id: &String,
    ) -> Result<Option<Self>, DbError> {
        let (sql, values) = Query::select()
            .column(Asterisk)
            .from(SubscriptionIden::Table)
            .and_where(
                Expr::col(SubscriptionIden::StripeSubscriptionId)
                    .eq(stripe_subscription_id),
            )
            .build_postgres(PostgresQueryBuilder);

        let row = conn.query_opt(sql.as_str(), &values.as_params()).await?;

        Ok(row.map(Self::from))
    }

    pub async fn put_checkout_session(
        pool: &Pool,
        stripe_subscription_id: &String,
        buyer_user_id: &String,
        offer_id: &Uuid,
        shop_id: &Uuid,
        event_timestamp: i64,
    ) -> Result<Self, DbError> {
        let conn = pool.get().await?;

        let (sql, values) = Query::insert()
            .into_table(SubscriptionIden::Table)
            .columns(Self::PUT_CHECKOUT_SESSION_COLUMNS)
            .values([
                stripe_subscription_id.into(),
                buyer_user_id.into(),
                (*offer_id).into(),
                (*shop_id).into(),
                event_timestamp.into(),
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

    #[allow(clippy::too_many_arguments)]
    pub async fn put_subscription<'a>(
        conn: &Transaction<'a>,
        stripe_subscription_id: &String,
        current_period_start: &DateTime<Utc>,
        current_period_end: &DateTime<Utc>,
        subscription_status: &String,
        canceled_at: Option<DateTime<Utc>>,
        cancel_at: Option<DateTime<Utc>>,
        event_timestamp: i64,
    ) -> Result<Self, DbError> {
        let (sql, values) = Query::insert()
            .into_table(SubscriptionIden::Table)
            .columns(Self::PUT_SUBSCRIPTION_COLUMNS)
            .values([
                stripe_subscription_id.into(),
                (*current_period_start).into(),
                (*current_period_end).into(),
                subscription_status.into(),
                canceled_at.into(),
                cancel_at.into(),
                event_timestamp.into(),
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

    pub async fn put_invoice<'a>(
        pool: &Pool,
        stripe_subscription_id: &String,
        payed_at: &DateTime<Utc>,
        payed_until: &DateTime<Utc>,
        event_timestamp: i64,
    ) -> Result<Self, DbError> {
        let conn = pool.get().await?;

        let (sql, values) = Query::insert()
            .into_table(SubscriptionIden::Table)
            .columns(Self::PUT_INVOICE_COLUMNS)
            .values([
                stripe_subscription_id.into(),
                (*payed_at).into(),
                (*payed_until).into(),
                event_timestamp.into(),
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
            shop_id: row.get(SubscriptionIden::ShopId.to_string().as_str()),
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
            canceled_at: row
                .get(SubscriptionIden::CanceledAt.to_string().as_str()),
            cancel_at: row.get(SubscriptionIden::CancelAt.to_string().as_str()),
            event_timestamp: row
                .get(SubscriptionIden::EventTimestamp.to_string().as_str()),
        }
    }
}
