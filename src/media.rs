use tonic::transport::Channel;
use tonic::{Request, Status};

use crate::api::peoplesmarkets::media::v1::media_subscription_service_client::MediaSubscriptionServiceClient;
use crate::api::peoplesmarkets::media::v1::PutMediaSubscriptionRequest;
use crate::CredentialsService;

#[derive(Debug, Clone)]
pub struct MediaService {
    media_subscription_client: MediaSubscriptionServiceClient<Channel>,
    credentials_service: CredentialsService,
}

impl MediaService {
    pub async fn init(
        url: String,
        credentials_service: CredentialsService,
    ) -> Result<Self, tonic::transport::Error> {
        Ok(Self {
            media_subscription_client: MediaSubscriptionServiceClient::connect(
                url.clone(),
            )
            .await?,
            credentials_service,
        })
    }

    pub async fn put_media_subscription(
        &self,
        media_subscription_id: String,
        buyer_user_id: String,
        offer_id: String,
        current_period_start: u64,
        current_period_end: u64,
        subscription_status: String,
        payed_at: u64,
    ) -> Result<(), Status> {
        let mut client = self.media_subscription_client.clone();

        let mut request = Request::new(PutMediaSubscriptionRequest {
            media_subscription_id,
            buyer_user_id,
            offer_id,
            current_period_start,
            current_period_end,
            subscription_status,
            payed_at,
        });

        self.credentials_service
            .with_auth_header(&mut request)
            .await;

        client.put_media_subscription(request).await?;

        Ok(())
    }
}
