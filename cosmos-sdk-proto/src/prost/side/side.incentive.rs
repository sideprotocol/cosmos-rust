// @generated
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// Indicates if the incentive mechanism is enabled
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    /// Reward per deposit tx via btc bridge
    #[prost(message, optional, tag = "2")]
    pub reward_per_deposit: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    /// Reward per withdrawal tx via btc bridge
    #[prost(message, optional, tag = "3")]
    pub reward_per_withdraw: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
/// GenesisState defines the incentive module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// Rewards
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rewards {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub deposit_count: u64,
    #[prost(uint64, tag = "3")]
    pub withdraw_count: u64,
    #[prost(message, optional, tag = "4")]
    pub deposit_reward: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "5")]
    pub withdraw_reward: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "6")]
    pub total_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
/// Reward Statistics
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardStats {
    #[prost(uint64, tag = "1")]
    pub address_count: u64,
    #[prost(uint64, tag = "2")]
    pub tx_count: u64,
    #[prost(message, optional, tag = "3")]
    pub total_reward_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryRewardsRequest is request type for the Query/Rewards RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardsRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryRewardsResponse is response type for the Query/Rewards RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardsResponse {
    #[prost(message, optional, tag = "1")]
    pub rewards: ::core::option::Option<Rewards>,
}
/// QueryRewardStatsRequest is request type for the Query/RewardStats RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardStatsRequest {}
/// QueryRewardStatsResponse is response type for the Query/RewardStats RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardStatsResponse {
    #[prost(message, optional, tag = "1")]
    pub reward_stats: ::core::option::Option<RewardStats>,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParams is the Msg/UpdateParams request type.
///
/// Since: cosmos-sdk 0.47
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/incentive parameters to be updated.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the Msg/UpdateParams response type.
///
/// Since: cosmos-sdk 0.47
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
include!("side.incentive.tonic.rs");
// @@protoc_insertion_point(module)
