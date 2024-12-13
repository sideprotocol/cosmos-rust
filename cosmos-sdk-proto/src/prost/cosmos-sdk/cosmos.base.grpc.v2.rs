// @generated
/// QueryRequest is the request for the Query method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRequest {
    #[prost(message, optional, tag = "1")]
    pub request: ::core::option::Option<::prost_types::Any>,
}
/// QueryResponse is the response for the Query method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResponse {
    #[prost(message, optional, tag = "1")]
    pub response: ::core::option::Option<::prost_types::Any>,
}
/// ListQueryHandlersRequest is the request for the ListQueryHandlers method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListQueryHandlersRequest {}
/// ListQueryHandlersResponse is the response for the ListQueryHandlers method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListQueryHandlersResponse {
    #[prost(message, repeated, tag = "1")]
    pub handlers: ::prost::alloc::vec::Vec<Handler>,
}
/// Handler defines a query handler
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Handler {
    #[prost(string, tag = "1")]
    pub request_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub response_name: ::prost::alloc::string::String,
}
include!("cosmos.base.grpc.v2.tonic.rs");
// @@protoc_insertion_point(module)
