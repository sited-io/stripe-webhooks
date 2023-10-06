#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaSubscriptionResponse {
    #[prost(string, tag = "1")]
    pub media_subscription_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub buyer_user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub shop_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub offer_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "6")]
    pub current_period_start: u64,
    #[prost(uint64, tag = "7")]
    pub current_period_end: u64,
    #[prost(string, tag = "8")]
    pub subscription_status: ::prost::alloc::string::String,
    #[prost(uint64, tag = "9")]
    pub payed_at: u64,
    #[prost(uint64, tag = "10")]
    pub payed_until: u64,
    #[prost(string, optional, tag = "11")]
    pub stripe_subscription_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "12")]
    pub canceled_at: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutMediaSubscriptionRequest {
    #[prost(string, tag = "1")]
    pub media_subscription_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub buyer_user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub offer_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub current_period_start: u64,
    #[prost(uint64, tag = "5")]
    pub current_period_end: u64,
    #[prost(string, tag = "6")]
    pub subscription_status: ::prost::alloc::string::String,
    #[prost(uint64, tag = "7")]
    pub payed_at: u64,
    #[prost(uint64, tag = "8")]
    pub payed_until: u64,
    #[prost(string, tag = "9")]
    pub shop_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "10")]
    pub stripe_subscription_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "11")]
    pub canceled_at: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutMediaSubscriptionResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMediaSubscriptionRequest {
    #[prost(string, optional, tag = "1")]
    pub media_subscription_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub offer_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMediaSubscriptionResponse {
    #[prost(message, optional, tag = "1")]
    pub media_subscription: ::core::option::Option<MediaSubscriptionResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMediaSubscriptionsRequest {
    #[prost(string, optional, tag = "1")]
    pub shop_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::pagination::v1::Pagination>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMediaSubscriptionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub media_subscriptions: ::prost::alloc::vec::Vec<MediaSubscriptionResponse>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::pagination::v1::Pagination>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelMediaSubscriptionRequest {
    #[prost(string, tag = "1")]
    pub media_subscription_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelMediaSubscriptionResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResumeMediaSubscriptionRequest {
    #[prost(string, tag = "1")]
    pub media_subscription_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResumeMediaSubscriptionResponse {}
/// Generated client implementations.
pub mod media_subscription_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct MediaSubscriptionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MediaSubscriptionServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MediaSubscriptionServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MediaSubscriptionServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MediaSubscriptionServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn put_media_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::PutMediaSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PutMediaSubscriptionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.media.v1.MediaSubscriptionService/PutMediaSubscription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.media.v1.MediaSubscriptionService",
                        "PutMediaSubscription",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_media_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMediaSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMediaSubscriptionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.media.v1.MediaSubscriptionService/GetMediaSubscription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.media.v1.MediaSubscriptionService",
                        "GetMediaSubscription",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_media_subscriptions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMediaSubscriptionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMediaSubscriptionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.media.v1.MediaSubscriptionService/ListMediaSubscriptions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.media.v1.MediaSubscriptionService",
                        "ListMediaSubscriptions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn cancel_media_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelMediaSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelMediaSubscriptionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.media.v1.MediaSubscriptionService/CancelMediaSubscription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.media.v1.MediaSubscriptionService",
                        "CancelMediaSubscription",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn resume_media_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::ResumeMediaSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResumeMediaSubscriptionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/peoplesmarkets.media.v1.MediaSubscriptionService/ResumeMediaSubscription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "peoplesmarkets.media.v1.MediaSubscriptionService",
                        "ResumeMediaSubscription",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
