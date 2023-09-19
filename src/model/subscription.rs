use chrono::{DateTime, Utc};
use deadpool_postgres::tokio_postgres::Row;
use deadpool_postgres::Transaction;
use sea_query::{Asterisk, Expr, Iden, PostgresQueryBuilder, Query};
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Subscription {
    pub async fn create<'a>(
        transaction: &Transaction<'a>,
        stripe_subscription_id: String,
        buyer_user_id: Option<String>,
        offer_id: Option<Uuid>,
        current_period_start: Option<DateTime<Utc>>,
        current_period_end: Option<DateTime<Utc>>,
        subscription_status: Option<String>,
        payed_at: Option<DateTime<Utc>>,
    ) -> Result<Self, DbError> {
        let (sql, values) = Query::insert()
            .into_table(SubscriptionIden::Table)
            .columns([
                SubscriptionIden::StripeSubscriptionId,
                SubscriptionIden::BuyerUserId,
                SubscriptionIden::OfferId,
                SubscriptionIden::CurrentPeriodStart,
                SubscriptionIden::CurrentPeriodEnd,
                SubscriptionIden::SubscriptionStatus,
                SubscriptionIden::PayedAt,
            ])
            .values([
                stripe_subscription_id.into(),
                buyer_user_id.into(),
                offer_id.into(),
                current_period_start.into(),
                current_period_end.into(),
                subscription_status.into(),
                payed_at.into(),
            ])?
            .returning_all()
            .build_postgres(PostgresQueryBuilder);

        let row = transaction
            .query_one(sql.as_str(), &values.as_params())
            .await?;

        Ok(Self::from(row))
    }

    pub async fn get<'a>(
        transaction: &Transaction<'a>,
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

        let row = transaction
            .query_opt(sql.as_str(), &values.as_params())
            .await?;

        Ok(row.map(Self::from))
    }

    pub async fn update<'a>(
        transaction: &Transaction<'a>,
        stripe_subscription_id: String,
        buyer_user_id: Option<String>,
        offer_id: Option<Uuid>,
        current_period_start: Option<DateTime<Utc>>,
        current_period_end: Option<DateTime<Utc>>,
        subscription_status: Option<String>,
    ) -> Result<Self, DbError> {
        let (sql, values) = {
            let mut query = Query::update();

            query.table(SubscriptionIden::Table);

            if buyer_user_id.is_some() {
                query.value(SubscriptionIden::BuyerUserId, buyer_user_id);
            }

            if offer_id.is_some() {
                query.value(SubscriptionIden::OfferId, offer_id);
            }

            if current_period_start.is_some() {
                query.value(
                    SubscriptionIden::CurrentPeriodStart,
                    current_period_start,
                );
            }

            if current_period_end.is_some() {
                query.value(
                    SubscriptionIden::CurrentPeriodEnd,
                    current_period_end,
                );
            }

            if subscription_status.is_some() {
                query.value(
                    SubscriptionIden::SubscriptionStatus,
                    subscription_status,
                );
            }

            query
                .and_where(
                    Expr::col(SubscriptionIden::StripeSubscriptionId)
                        .eq(stripe_subscription_id),
                )
                .returning_all()
                .build_postgres(PostgresQueryBuilder)
        };

        let row = transaction
            .query_one(sql.as_str(), values.as_params().as_ref())
            .await?;

        Ok(Self::from(row))
    }

    pub async fn update_payed_at<'a>(
        transaction: &Transaction<'a>,
        stripe_subscription_id: &String,
        payed_at: DateTime<Utc>,
    ) -> Result<Self, DbError> {
        let (sql, values) = Query::update()
            .table(SubscriptionIden::Table)
            .value(SubscriptionIden::PayedAt, Some(payed_at))
            .and_where(
                Expr::col(SubscriptionIden::StripeSubscriptionId)
                    .eq(stripe_subscription_id),
            )
            .returning_all()
            .build_postgres(PostgresQueryBuilder);

        let row = transaction
            .query_one(sql.as_str(), values.as_params().as_ref())
            .await?;

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
            created_at: row
                .get(SubscriptionIden::CreatedAt.to_string().as_str()),
            updated_at: row
                .get(SubscriptionIden::UpdatedAt.to_string().as_str()),
        }
    }
}
