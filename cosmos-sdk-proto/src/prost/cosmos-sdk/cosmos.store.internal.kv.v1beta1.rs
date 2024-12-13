// @generated
// This is duplicated from the base kv directory to avoid a circular dependency
// with the cosmos-sdk

/// Pairs defines a repeated slice of Pair objects.
///
/// Deprecated: Store v1 is deprecated as of v0.50.x, please use Store v2 types
/// instead.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pairs {
    #[prost(message, repeated, tag = "1")]
    pub pairs: ::prost::alloc::vec::Vec<Pair>,
}
/// Pair defines a key/value bytes tuple.
///
/// Deprecated: Store v1 is deprecated as of v0.50.x, please use Store v2 types
/// instead.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pair {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
// @@protoc_insertion_point(module)
