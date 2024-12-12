// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bid {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub auction_id: u64,
    #[prost(string, tag = "3")]
    pub bidder: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub bid_price: i64,
    #[prost(message, optional, tag = "5")]
    pub bid_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(enumeration = "BidStatus", tag = "6")]
    pub status: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Auction {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(message, optional, tag = "2")]
    pub deposited_asset: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "3")]
    pub borrower: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub liquidated_price: i64,
    #[prost(message, optional, tag = "5")]
    pub liquidated_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(int64, tag = "6")]
    pub expected_value: i64,
    #[prost(int64, tag = "7")]
    pub bidded_value: i64,
    #[prost(string, tag = "8")]
    pub payment_tx_id: ::prost::alloc::string::String,
    #[prost(enumeration = "AuctionStatus", tag = "9")]
    pub status: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AssetType {
    Bitcoin = 0,
}
impl AssetType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AssetType::Bitcoin => "Bitcoin",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AuctionStatus {
    AuctionOpen = 0,
    AuctionClose = 1,
}
impl AuctionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AuctionStatus::AuctionOpen => "AuctionOpen",
            AuctionStatus::AuctionClose => "AuctionClose",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BidStatus {
    Bidding = 0,
    Accepted = 1,
    Rejected = 2,
}
impl BidStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BidStatus::Bidding => "Bidding",
            BidStatus::Accepted => "Accepted",
            BidStatus::Rejected => "Rejected",
        }
    }
}
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(message, optional, tag = "1")]
    pub price_drop_period: ::core::option::Option<::prost_types::Duration>,
    #[prost(uint32, tag = "2")]
    pub initial_discount: u32,
    #[prost(uint32, tag = "3")]
    pub fee_rate: u32,
    #[prost(uint64, tag = "4")]
    pub min_bid_amount: u64,
}
/// GenesisState defines the auctioin module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub auctions: ::prost::alloc::vec::Vec<Auction>,
    #[prost(message, repeated, tag = "3")]
    pub bids: ::prost::alloc::vec::Vec<Bid>,
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
/// QueryAuctionsRequest is request type for the Query/Auctions RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuctionsRequest {
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryAuctionsResponse is response type for the Query/Auctions RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuctionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub auctions: ::prost::alloc::vec::Vec<Auction>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryBidsRequest is request type for the Query/Bids RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBidsRequest {
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryBidsResponse is response type for the Query/Bids RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBidsResponse {
    #[prost(message, repeated, tag = "1")]
    pub bids: ::prost::alloc::vec::Vec<Bid>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// MsgBid defines the Msg/Bid request type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBid {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub auction_id: u64,
    #[prost(int64, tag = "3")]
    pub price: i64,
    #[prost(message, optional, tag = "4")]
    pub amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgBidResponse defines the Msg/Bid response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBidResponse {}
include!("side.auction.tonic.rs");
// @@protoc_insertion_point(module)
