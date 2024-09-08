// This file is @generated by prost-build.
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
    #[prost(uint64, optional, tag = "13")]
    pub cancel_at: ::core::option::Option<u64>,
}
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
    #[prost(uint64, optional, tag = "12")]
    pub cancel_at: ::core::option::Option<u64>,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PutMediaSubscriptionResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMediaSubscriptionRequest {
    #[prost(string, optional, tag = "1")]
    pub media_subscription_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub offer_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMediaSubscriptionResponse {
    #[prost(message, optional, tag = "1")]
    pub media_subscription: ::core::option::Option<MediaSubscriptionResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMediaSubscriptionsRequest {
    #[prost(string, optional, tag = "1")]
    pub shop_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::types::v1::PaginationRequest>,
    #[prost(bool, optional, tag = "3")]
    pub is_accessible: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMediaSubscriptionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub media_subscriptions: ::prost::alloc::vec::Vec<MediaSubscriptionResponse>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::types::v1::PaginationResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelMediaSubscriptionRequest {
    #[prost(string, tag = "1")]
    pub media_subscription_id: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CancelMediaSubscriptionResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResumeMediaSubscriptionRequest {
    #[prost(string, tag = "1")]
    pub media_subscription_id: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ResumeMediaSubscriptionResponse {}
