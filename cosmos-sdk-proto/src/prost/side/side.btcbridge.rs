// @generated
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// The minimum number of confirmations required for a block to be accepted
    #[prost(int32, tag = "1")]
    pub confirmations: i32,
    /// Indicates the maximum depth or distance from the latest block up to which transactions are considered for acceptance.
    #[prost(uint64, tag = "2")]
    pub max_acceptable_block_depth: u64,
    /// The denomination of the voucher
    #[prost(string, tag = "3")]
    pub btc_voucher_denom: ::prost::alloc::string::String,
    /// Asset vaults
    #[prost(message, repeated, tag = "4")]
    pub vaults: ::prost::alloc::vec::Vec<Vault>,
    /// Protocol limitations
    #[prost(message, optional, tag = "5")]
    pub protocol_limits: ::core::option::Option<ProtocolLimits>,
    /// Protocol fees
    #[prost(message, optional, tag = "6")]
    pub protocol_fees: ::core::option::Option<ProtocolFees>,
    /// Network fee for withdrawal to bitcoin
    #[prost(int64, tag = "7")]
    pub network_fee: i64,
    /// Reward epoch for relayer and TSS participant incentivization
    #[prost(message, optional, tag = "8")]
    pub reward_epoch: ::core::option::Option<::prost_types::Duration>,
    /// TSS params
    #[prost(message, optional, tag = "9")]
    pub tss_params: ::core::option::Option<TssParams>,
}
/// Vault defines the asset vault
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vault {
    /// the depositor should send their btc to this address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// the pub key to which the voucher is sent
    #[prost(string, tag = "2")]
    pub pub_key: ::prost::alloc::string::String,
    /// the address to which the voucher is sent
    #[prost(enumeration = "AssetType", tag = "3")]
    pub asset_type: i32,
    /// version
    #[prost(uint64, tag = "4")]
    pub version: u64,
}
/// ProtocolLimits defines the params related to the the protocol limitations
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtocolLimits {
    /// The minimum deposit amount for btc
    #[prost(int64, tag = "1")]
    pub btc_min_deposit: i64,
    /// The minimum withdrawal amount for btc
    #[prost(int64, tag = "2")]
    pub btc_min_withdraw: i64,
    /// The maximum withdrawal amount for btc
    #[prost(int64, tag = "3")]
    pub btc_max_withdraw: i64,
}
/// ProtocolFees defines the params related to the protocol fees
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtocolFees {
    /// Protocol fee amount for deposit
    #[prost(int64, tag = "1")]
    pub deposit_fee: i64,
    /// Protocol fee amount for withdrawal
    #[prost(int64, tag = "2")]
    pub withdraw_fee: i64,
    /// Protocol fee collector
    #[prost(string, tag = "3")]
    pub collector: ::prost::alloc::string::String,
}
/// TSSParams defines the params related to TSS
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TssParams {
    /// timeout duration for DKG request
    #[prost(message, optional, tag = "1")]
    pub dkg_timeout_period: ::core::option::Option<::prost_types::Duration>,
    /// Transition period after which TSS participants update process is completed
    #[prost(message, optional, tag = "2")]
    pub participant_update_transition_period: ::core::option::Option<::prost_types::Duration>,
}
/// AssetType defines the type of asset
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AssetType {
    /// Unspecified asset type
    Unspecified = 0,
    /// BTC
    Btc = 1,
    /// BRC20: ordi, sats
    Brc20 = 2,
    /// RUNE, dog*go*to*the*moon
    Rune = 3,
}
impl AssetType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AssetType::Unspecified => "ASSET_TYPE_UNSPECIFIED",
            AssetType::Btc => "ASSET_TYPE_BTC",
            AssetType::Brc20 => "ASSET_TYPE_BRC20",
            AssetType::Rune => "ASSET_TYPE_RUNE",
        }
    }
}
/// Bitcoin Block Header
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockHeader {
    #[prost(uint64, tag = "1")]
    pub version: u64,
    #[prost(string, tag = "2")]
    pub hash: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub height: u64,
    #[prost(string, tag = "4")]
    pub previous_block_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub merkle_root: ::prost::alloc::string::String,
    #[prost(uint64, tag = "6")]
    pub nonce: u64,
    #[prost(string, tag = "7")]
    pub bits: ::prost::alloc::string::String,
    #[prost(uint64, tag = "8")]
    pub time: u64,
    #[prost(uint64, tag = "9")]
    pub ntx: u64,
}
/// Bitcoin Withdrawal Request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BitcoinWithdrawRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, tag = "3")]
    pub sequence: u64,
    #[prost(string, tag = "4")]
    pub txid: ::prost::alloc::string::String,
    #[prost(enumeration = "WithdrawStatus", tag = "5")]
    pub status: i32,
}
/// Rune ID
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuneId {
    /// block height
    #[prost(uint64, tag = "1")]
    pub block: u64,
    /// tx index
    #[prost(uint32, tag = "2")]
    pub tx: u32,
}
/// Rune Edict
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Edict {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<RuneId>,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub output: u32,
}
/// DKG Participant
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DkgParticipant {
    /// the moniker of the corresponding validator
    #[prost(string, tag = "1")]
    pub moniker: ::prost::alloc::string::String,
    /// the operator address of the corresponding validator
    #[prost(string, tag = "2")]
    pub operator_address: ::prost::alloc::string::String,
    /// the consensus address of the corresponding validator
    #[prost(string, tag = "3")]
    pub consensus_address: ::prost::alloc::string::String,
}
/// DKG Request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DkgRequest {
    /// the unique request id
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// participant set
    #[prost(message, repeated, tag = "2")]
    pub participants: ::prost::alloc::vec::Vec<DkgParticipant>,
    /// threshold required to perform DKG
    #[prost(uint32, tag = "3")]
    pub threshold: u32,
    /// expiration time
    #[prost(message, optional, tag = "4")]
    pub expiration: ::core::option::Option<::prost_types::Timestamp>,
    /// status
    #[prost(enumeration = "DkgRequestStatus", tag = "5")]
    pub status: i32,
}
/// DKG Completion Request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DkgCompletionRequest {
    /// request id
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// sender
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    /// new vaults generated by DKG
    #[prost(string, repeated, tag = "3")]
    pub vaults: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// validator address
    #[prost(string, tag = "4")]
    pub validator: ::prost::alloc::string::String,
    /// hex encoded validator signature
    #[prost(string, tag = "5")]
    pub signature: ::prost::alloc::string::String,
}
/// Bitcoin Withdrawal Status
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WithdrawStatus {
    /// WITHDRAW_STATUS_UNSPECIFIED - Default value, should not be used
    Unspecified = 0,
    /// WITHDRAW_STATUS_CREATED - The withdrawal request is created
    Created = 1,
    /// WITHDRAW_STATUS_BROADCASTED - The withdrawal tx is broadcasted
    Broadcasted = 2,
    /// WITHDRAW_STATUS_CONFIRMED - The withdrawal tx is confirmed
    Confirmed = 3,
}
impl WithdrawStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            WithdrawStatus::Unspecified => "WITHDRAW_STATUS_UNSPECIFIED",
            WithdrawStatus::Created => "WITHDRAW_STATUS_CREATED",
            WithdrawStatus::Broadcasted => "WITHDRAW_STATUS_BROADCASTED",
            WithdrawStatus::Confirmed => "WITHDRAW_STATUS_CONFIRMED",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DkgRequestStatus {
    /// DKG_REQUEST_STATUS_UNSPECIFIED defines the unknown DKG request status
    Unspecified = 0,
    /// DKG_REQUEST_STATUS_PENDING defines the status of the DKG request which is pending
    Pending = 1,
    /// DKG_REQUEST_STATUS_COMPLETED defines the status of the DKG request which is completed
    Completed = 2,
    /// DKG_REQUEST_STATUS_FAILED defines the status of the DKG request which failed
    Failed = 3,
}
impl DkgRequestStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DkgRequestStatus::Unspecified => "DKG_REQUEST_STATUS_UNSPECIFIED",
            DkgRequestStatus::Pending => "DKG_REQUEST_STATUS_PENDING",
            DkgRequestStatus::Completed => "DKG_REQUEST_STATUS_COMPLETED",
            DkgRequestStatus::Failed => "DKG_REQUEST_STATUS_FAILED",
        }
    }
}
/// GenesisState defines the btc light client module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// the chain tip of the bitcoin chain
    #[prost(message, optional, tag = "2")]
    pub best_block_header: ::core::option::Option<BlockHeader>,
    #[prost(message, repeated, tag = "3")]
    pub block_headers: ::prost::alloc::vec::Vec<BlockHeader>,
}
/// QueryWithdrawRequestsRequest is request type for the Query/WithdrawRequests RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawRequestsRequest {
    #[prost(enumeration = "WithdrawStatus", tag = "1")]
    pub status: i32,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryWithdrawRequestsResponse is response type for the Query/WithdrawRequests RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawRequestsResponse {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<BitcoinWithdrawRequest>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryWithdrawRequestsByAddressRequest is request type for the Query/WithdrawRequestsByAddress RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawRequestsByAddressRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryWithdrawRequestsByAddressResponse is response type for the Query/WithdrawRequestsByAddress RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawRequestsByAddressResponse {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<BitcoinWithdrawRequest>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryWithdrawRequestByTxHashRequest is request type for the Query/WithdrawRequestByTxHash RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawRequestByTxHashRequest {
    #[prost(string, tag = "1")]
    pub txid: ::prost::alloc::string::String,
}
/// QueryWithdrawRequestByTxHashResponse is response type for the Query/WithdrawRequestByTxHash RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawRequestByTxHashResponse {
    #[prost(message, optional, tag = "1")]
    pub request: ::core::option::Option<BitcoinWithdrawRequest>,
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
/// QueryChainTipRequest is request type for the Query/ChainTip RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryChainTipRequest {}
/// QueryChainTipResponse is response type for the Query/ChainTip RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryChainTipResponse {
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub height: u64,
}
/// QueryBlockHeaderByHeightRequest is the request type for the Query/BlockHeaderByHeight RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBlockHeaderByHeightRequest {
    #[prost(uint64, tag = "1")]
    pub height: u64,
}
/// QueryBlockHeaderByHeightResponse is the response type for the Query/BlockHeaderByHeight RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBlockHeaderByHeightResponse {
    #[prost(message, optional, tag = "1")]
    pub block_header: ::core::option::Option<BlockHeader>,
}
/// QueryBlockHeaderByHashRequest is the request type for the Query/BlockHeaderByHash RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBlockHeaderByHashRequest {
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
}
/// QueryBlockHeaderByHashResponse is the response type for the Query/BlockHeaderByHash RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBlockHeaderByHashResponse {
    #[prost(message, optional, tag = "1")]
    pub block_header: ::core::option::Option<BlockHeader>,
}
/// QueryDKGRequestRequest is the request type for the Query/DKGRequest RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDkgRequestRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
/// QueryDKGRequestResponse is the response type for the Query/DKGRequest RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDkgRequestResponse {
    #[prost(message, optional, tag = "1")]
    pub request: ::core::option::Option<DkgRequest>,
}
/// QueryDKGRequestsRequest is the request type for the Query/DKGRequests RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDkgRequestsRequest {
    #[prost(enumeration = "DkgRequestStatus", tag = "1")]
    pub status: i32,
}
/// QueryDKGRequestsResponse is the response type for the Query/DKGRequests RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDkgRequestsResponse {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<DkgRequest>,
}
/// QueryAllDKGRequestsRequest is the request type for the Query/AllDKGRequests RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllDkgRequestsRequest {}
/// QueryAllDKGRequestsResponse is the response type for the Query/AllDKGRequests RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllDkgRequestsResponse {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<DkgRequest>,
}
/// QueryDKGCompletionRequestsRequest is the request type for the Query/DKGCompletionRequests RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDkgCompletionRequestsRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
/// QueryDKGCompletionRequestsResponse is the response type for the Query/DKGCompletionRequests RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDkgCompletionRequestsResponse {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<DkgCompletionRequest>,
}
/// MsgSubmitWithdrawStatus defines the Msg/SubmitWithdrawStatus request type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitWithdrawStatus {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub sequence: u64,
    #[prost(string, tag = "3")]
    pub txid: ::prost::alloc::string::String,
    #[prost(enumeration = "WithdrawStatus", tag = "4")]
    pub status: i32,
}
/// MsgSubmitWithdrawStatusResponse defines the Msg/SubmitWithdrawStatus response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitWithdrawStatusResponse {}
/// MsgSubmitBlockHeaders defines the Msg/SubmitBlockHeaders request type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitBlockHeaders {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub block_headers: ::prost::alloc::vec::Vec<BlockHeader>,
}
/// MsgSubmitBlockHeadersResponse defines the Msg/SubmitBlockHeaders response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitBlockHeadersResponse {}
/// MsgSubmitDepositTransaction defines the Msg/SubmitDepositTransaction request type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitDepositTransaction {
    /// this is relayer address who submit the bitcoin transaction to the side chain
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub blockhash: ::prost::alloc::string::String,
    /// the tx bytes in base64 format
    /// used for parsing the sender of the transaction
    #[prost(string, tag = "3")]
    pub prev_tx_bytes: ::prost::alloc::string::String,
    /// the tx bytes in base64 format
    #[prost(string, tag = "4")]
    pub tx_bytes: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub proof: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgSubmitDepositTransactionResponse defines the Msg/SubmitDepositTransaction response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitDepositTransactionResponse {}
/// MsgSubmitWithdrawTransaction defines the Msg/SubmitWithdrawTransaction request type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitWithdrawTransaction {
    /// this is relayer address who submit the bitcoin transaction to the side chain
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub blockhash: ::prost::alloc::string::String,
    /// the previous tx bytes in base64 format
    #[prost(string, tag = "3")]
    pub prev_tx_bytes: ::prost::alloc::string::String,
    /// the tx bytes in base64 format
    #[prost(string, tag = "4")]
    pub tx_bytes: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub proof: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgSubmitWithdrawTransactionResponse defines the Msg/SubmitWithdrawTransaction response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitWithdrawTransactionResponse {}
/// MsgWithdrawToBitcoin defines the Msg/WithdrawToBitcoin request type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawToBitcoin {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// withdraw amount in satoshi, etc: 100000000sat = 1btc
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
/// MsgWithdrawToBitcoinResponse defines the Msg/WithdrawToBitcoin response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawToBitcoinResponse {}
/// MsgInitiateDKG is the Msg/InitiateDKG request type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInitiateDkg {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// expected participant set
    #[prost(message, repeated, tag = "2")]
    pub participants: ::prost::alloc::vec::Vec<DkgParticipant>,
    /// threshold required to perform DKG
    #[prost(uint32, tag = "3")]
    pub threshold: u32,
}
/// MsgInitiateDKGResponse defines the Msg/InitiateDKG response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInitiateDkgResponse {}
/// MsgCompleteDKG is the Msg/CompleteDKG request type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCompleteDkg {
    /// the sender
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// DKG request id
    #[prost(uint64, tag = "2")]
    pub id: u64,
    /// new vaults generated by DKG
    #[prost(string, repeated, tag = "3")]
    pub vaults: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// validator address
    #[prost(string, tag = "4")]
    pub validator: ::prost::alloc::string::String,
    /// hex encoded validator signature
    #[prost(string, tag = "5")]
    pub signature: ::prost::alloc::string::String,
}
/// MsgCompleteDKGResponse defines the Msg/CompleteDKG response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCompleteDkgResponse {}
/// MsgUpdateParams is the Msg/UpdateParams request type.
///
/// Since: cosmos-sdk 0.47
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/btcbridge parameters to be updated.
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
include!("side.btcbridge.tonic.rs");
// @@protoc_insertion_point(module)
