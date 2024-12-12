// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DlcAnnouncement {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(message, optional, tag = "2")]
    pub oracle_event: ::core::option::Option<DlcEvent>,
    #[prost(string, tag = "3")]
    pub signature: ::prost::alloc::string::String,
    #[prost(enumeration = "AnnouncementStatus", tag = "4")]
    pub status: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DlcAttestation {
    #[prost(uint64, tag = "1")]
    pub announcement_id: u64,
    #[prost(message, optional, tag = "2")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "3")]
    pub outcome: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub signature: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DlcEvent {
    #[prost(uint32, tag = "1")]
    pub maturity_epoch: u32,
    #[prost(string, tag = "2")]
    pub nonce: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub index: u64,
    #[prost(string, tag = "4")]
    pub descriptor: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub pubkey: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AnnouncementStatus {
    AnnouncementUnspecified = 0,
    AnnouncementPending = 1,
    AnnouncementReady = 2,
}
impl AnnouncementStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AnnouncementStatus::AnnouncementUnspecified => "Announcement_Unspecified",
            AnnouncementStatus::AnnouncementPending => "Announcement_Pending",
            AnnouncementStatus::AnnouncementReady => "Announcement_Ready",
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceInterval {
    #[prost(string, tag = "1")]
    pub price_pair: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub interval: i32,
}
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(message, repeated, tag = "1")]
    pub price_interval: ::prost::alloc::vec::Vec<PriceInterval>,
}
/// GenesisState defines the dlc module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub announcements: ::prost::alloc::vec::Vec<DlcAnnouncement>,
    #[prost(message, repeated, tag = "3")]
    pub attestations: ::prost::alloc::vec::Vec<DlcAttestation>,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryAnnouncementsRequest is request type for the Query/Announcements RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAnnouncementsRequest {
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryAnnouncementsResponse is response type for the Query/Announcements RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAnnouncementsResponse {
    #[prost(message, repeated, tag = "1")]
    pub announcements: ::prost::alloc::vec::Vec<DlcAnnouncement>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryPriceRequest is request type for the Query/Price RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPriceRequest {
    #[prost(string, tag = "1")]
    pub symbol: ::prost::alloc::string::String,
}
/// QueryPriceResponse is response type for the Query/Price RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPriceResponse {
    #[prost(uint32, tag = "1")]
    pub price: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitAnnouncementNonce {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub announcement_id: u64,
    #[prost(string, tag = "3")]
    pub nonce: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub signature: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitAnnouncementNonceResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitAttestation {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub announcement_id: u64,
    #[prost(string, tag = "3")]
    pub signature: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitAttestationResponse {}
include!("side.dlc.tonic.rs");
// @@protoc_insertion_point(module)
