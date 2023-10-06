#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pagination {
    #[prost(uint64, tag = "1")]
    pub page: u64,
    #[prost(uint64, tag = "2")]
    pub size: u64,
}
