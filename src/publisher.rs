use prost::Message;

use crate::api::sited_io::media::v1::MediaSubscriptionResponse;

#[derive(Debug, Clone)]
pub struct Publisher {
    client: async_nats::Client,
}

impl Publisher {
    const SUBSCRIPTION_UPSERT_SUBJECT: &'static str =
        "stripe-webhooks.subscription.upsert";
    const SUBSCRIPTION_DELETE_SUBJECT: &'static str =
        "stripe-webhooks.subscription.delete";

    pub fn new(client: async_nats::Client) -> Self {
        Self { client }
    }

    pub async fn flush(
        &self,
    ) -> Result<(), async_nats::error::Error<async_nats::client::FlushErrorKind>>
    {
        self.client.flush().await
    }

    pub async fn publish_subscription_upsert(
        &self,
        subscription: &MediaSubscriptionResponse,
    ) {
        if let Err(err) = self
            .client
            .publish(
                Self::SUBSCRIPTION_UPSERT_SUBJECT,
                subscription.encode_to_vec().into(),
            )
            .await
        {
            tracing::error!("[Publisher.publish_subscription_upsert]: {err}");
        }
    }

    pub async fn publish_subscription_delete(
        &self,
        subscription: &MediaSubscriptionResponse,
    ) {
        if let Err(err) = self
            .client
            .publish(
                Self::SUBSCRIPTION_DELETE_SUBJECT,
                subscription.encode_to_vec().into(),
            )
            .await
        {
            tracing::error!("[Publisher.publish_subscription_delete]: {err}");
        }
    }
}
