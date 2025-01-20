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
    /// Indicates if deposit is enabled
    #[prost(bool, tag = "4")]
    pub deposit_enabled: bool,
    /// Indicates if withdrawal is enabled
    #[prost(bool, tag = "5")]
    pub withdraw_enabled: bool,
    /// Trusted relayers to submit bitcoin block headers
    #[prost(string, repeated, tag = "6")]
    pub trusted_btc_relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Trusted relayers for non-btc asset deposit
    #[prost(string, repeated, tag = "7")]
    pub trusted_non_btc_relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Trusted fee providers to submit bitcoin fee rate
    #[prost(string, repeated, tag = "8")]
    pub trusted_fee_providers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Period of validity for the fee rate
    #[prost(int64, tag = "9")]
    pub fee_rate_validity_period: i64,
    /// Asset vaults
    #[prost(message, repeated, tag = "10")]
    pub vaults: ::prost::alloc::vec::Vec<Vault>,
    /// Withdrawal params
    #[prost(message, optional, tag = "11")]
    pub withdraw_params: ::core::option::Option<WithdrawParams>,
    /// Protocol limitations
    #[prost(message, optional, tag = "12")]
    pub protocol_limits: ::core::option::Option<ProtocolLimits>,
    /// Protocol fees
    #[prost(message, optional, tag = "13")]
    pub protocol_fees: ::core::option::Option<ProtocolFees>,
    /// TSS params
    #[prost(message, optional, tag = "14")]
    pub tss_params: ::core::option::Option<TssParams>,
}
/// Vault defines the asset vault
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vault {
    /// the vault address for deposit
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// public key of the vault
    #[prost(string, tag = "2")]
    pub pub_key: ::prost::alloc::string::String,
    /// the asset type supported by the vault
    #[prost(enumeration = "AssetType", tag = "3")]
    pub asset_type: i32,
    /// version
    #[prost(uint64, tag = "4")]
    pub version: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawParams {
    /// Maximum number of utxos used to build the signing request; O means unlimited
    #[prost(uint32, tag = "1")]
    pub max_utxo_num: u32,
    /// Period for handling btc withdrawal requests
    #[prost(int64, tag = "2")]
    pub btc_batch_withdraw_period: i64,
    /// Maximum number of btc withdrawal requests to be handled per batch
    #[prost(uint32, tag = "3")]
    pub max_btc_batch_withdraw_num: u32,
}
/// ProtocolLimits defines the params related to the the protocol limitations
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtocolLimits {
    /// The minimum deposit amount for btc in sat
    #[prost(int64, tag = "1")]
    pub btc_min_deposit: i64,
    /// The minimum withdrawal amount for btc in sat
    #[prost(int64, tag = "2")]
    pub btc_min_withdraw: i64,
    /// The maximum withdrawal amount for btc in sat
    #[prost(int64, tag = "3")]
    pub btc_max_withdraw: i64,
}
/// ProtocolFees defines the params related to the protocol fees
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtocolFees {
    /// Protocol fee amount for deposit in sat
    #[prost(int64, tag = "1")]
    pub deposit_fee: i64,
    /// Protocol fee amount for withdrawal in sat
    #[prost(int64, tag = "2")]
    pub withdraw_fee: i64,
    /// Protocol fee collector
    #[prost(string, tag = "3")]
    pub collector: ::prost::alloc::string::String,
}
/// TSSParams defines the params related to TSS
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TssParams {
    /// Timeout duration for DKG request
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
    /// RUNE: dog•go•to•the•moon
    Runes = 3,
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
            AssetType::Runes => "ASSET_TYPE_RUNES",
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
/// Fee rate
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeRate {
    /// fee rate
    #[prost(int64, tag = "1")]
    pub value: i64,
    /// block height at which the fee rate is submitted
    #[prost(int64, tag = "2")]
    pub height: i64,
}
/// Bitcoin Signing Request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SigningRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub sequence: u64,
    #[prost(string, tag = "3")]
    pub txid: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub psbt: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub creation_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(enumeration = "SigningStatus", tag = "6")]
    pub status: i32,
}
/// Withdrawal Request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub sequence: u64,
    #[prost(string, tag = "4")]
    pub txid: ::prost::alloc::string::String,
}
/// Bitcoin UTXO
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Utxo {
    #[prost(string, tag = "1")]
    pub txid: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub vout: u64,
    #[prost(string, tag = "3")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub amount: u64,
    #[prost(uint64, tag = "5")]
    pub height: u64,
    #[prost(bytes = "vec", tag = "6")]
    pub pub_key_script: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "7")]
    pub is_locked: bool,
    /// rune balances associated with the UTXO
    #[prost(message, repeated, tag = "8")]
    pub runes: ::prost::alloc::vec::Vec<RuneBalance>,
}
/// Rune Balance
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuneBalance {
    /// serialized rune id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// rune amount
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
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
/// BTC UTXO Consolidation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BtcConsolidation {
    /// maximum threshold of the btc value
    #[prost(int64, tag = "1")]
    pub target_threshold: i64,
    /// maximum number of the utxos to be consolidated; 0 means all
    #[prost(uint32, tag = "2")]
    pub max_num: u32,
}
/// Runes UTXO Consolidation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunesConsolidation {
    /// rune id
    #[prost(string, tag = "1")]
    pub rune_id: ::prost::alloc::string::String,
    /// maximum threshold of the corresponding rune balance
    #[prost(string, tag = "2")]
    pub target_threshold: ::prost::alloc::string::String,
    /// maximum number of the utxos to be consolidated; 0 means all
    #[prost(uint32, tag = "3")]
    pub max_num: u32,
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
    /// the consensus public key of the corresponding validator
    #[prost(string, tag = "3")]
    pub consensus_pubkey: ::prost::alloc::string::String,
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
    /// asset types of vaults to be generated
    #[prost(enumeration = "AssetType", repeated, tag = "4")]
    pub vault_types: ::prost::alloc::vec::Vec<i32>,
    /// indicates if transferring assets to the newly generated vaults when the DKG request is completed
    #[prost(bool, tag = "5")]
    pub enable_transfer: bool,
    /// target number of the UTXOs to be transferred each time
    #[prost(uint32, tag = "6")]
    pub target_utxo_num: u32,
    /// expiration time
    #[prost(message, optional, tag = "7")]
    pub expiration: ::core::option::Option<::prost_types::Timestamp>,
    /// status
    #[prost(enumeration = "DkgRequestStatus", tag = "8")]
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
    /// consensus address of the corresponding validator
    #[prost(string, tag = "4")]
    pub consensus_address: ::prost::alloc::string::String,
    /// hex encoded validator signature
    #[prost(string, tag = "5")]
    pub signature: ::prost::alloc::string::String,
}
/// Bitcoin Signing Status
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SigningStatus {
    /// SIGNING_STATUS_UNSPECIFIED - Default value, should not be used
    Unspecified = 0,
    /// SIGNING_STATUS_PENDING - The signing request is pending
    Pending = 1,
    /// SIGNING_STATUS_BROADCASTED - The signing request is broadcasted
    Broadcasted = 2,
    /// SIGNING_STATUS_CONFIRMED - The signing request is confirmed
    Confirmed = 3,
    /// SIGNING_STATUS_FAILED - The signing request failed to be signed or broadcast due to unexpected exceptions
    Failed = 4,
}
impl SigningStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SigningStatus::Unspecified => "SIGNING_STATUS_UNSPECIFIED",
            SigningStatus::Pending => "SIGNING_STATUS_PENDING",
            SigningStatus::Broadcasted => "SIGNING_STATUS_BROADCASTED",
            SigningStatus::Confirmed => "SIGNING_STATUS_CONFIRMED",
            SigningStatus::Failed => "SIGNING_STATUS_FAILED",
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
    /// DKG_REQUEST_STATUS_TIMEDOUT defines the status of the DKG request which timed out
    Timedout = 4,
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
            DkgRequestStatus::Timedout => "DKG_REQUEST_STATUS_TIMEDOUT",
        }
    }
}
/// GenesisState defines the btc bridge module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// the chain tip of the bitcoin chain
    #[prost(message, optional, tag = "2")]
    pub best_block_header: ::core::option::Option<BlockHeader>,
    #[prost(message, repeated, tag = "3")]
    pub block_headers: ::prost::alloc::vec::Vec<BlockHeader>,
    #[prost(message, repeated, tag = "4")]
    pub utxos: ::prost::alloc::vec::Vec<Utxo>,
    #[prost(message, optional, tag = "5")]
    pub dkg_request: ::core::option::Option<DkgRequest>,
}
/// QueryWithdrawRequestsByAddressRequest is request type for the Query/WithdrawRequestsByAddress RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawRequestsByAddressRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryWithdrawRequestsByAddressResponse is response type for the Query/WithdrawRequestsByAddress RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawRequestsByAddressResponse {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<WithdrawRequest>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryWithdrawRequestsByTxHashRequest is request type for the Query/WithdrawRequestsByTxHash RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawRequestsByTxHashRequest {
    #[prost(string, tag = "1")]
    pub txid: ::prost::alloc::string::String,
}
/// QueryWithdrawRequestsByTxHashResponse is response type for the Query/WithdrawRequestsByTxHash RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawRequestsByTxHashResponse {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<WithdrawRequest>,
}
/// QueryPendingBtcWithdrawRequestsRequest is request type for the Query/PendingBtcWithdrawRequests RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPendingBtcWithdrawRequestsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryPendingBtcWithdrawRequestsResponse is response type for the Query/PendingBtcWithdrawRequests RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPendingBtcWithdrawRequestsResponse {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<WithdrawRequest>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QuerySigningRequestsRequest is request type for the Query/SigningRequests RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySigningRequestsRequest {
    #[prost(enumeration = "SigningStatus", tag = "1")]
    pub status: i32,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QuerySigningRequestsResponse is response type for the Query/SigningRequests RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySigningRequestsResponse {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<SigningRequest>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QuerySigningRequestsByAddressRequest is request type for the Query/SigningRequestsByAddress RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySigningRequestsByAddressRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QuerySigningRequestsByAddressResponse is response type for the Query/SigningRequestsByAddress RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySigningRequestsByAddressResponse {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<SigningRequest>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QuerySigningRequestByTxHashRequest is request type for the Query/SigningRequestByTxHash RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySigningRequestByTxHashRequest {
    #[prost(string, tag = "1")]
    pub txid: ::prost::alloc::string::String,
}
/// QuerySigningRequestByTxHashResponse is response type for the Query/SigningRequestByTxHashResponse RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySigningRequestByTxHashResponse {
    #[prost(message, optional, tag = "1")]
    pub request: ::core::option::Option<SigningRequest>,
}
/// QueryFeeRateRequest is request type for the Query/FeeRate RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeRateRequest {}
/// QueryFeeRateResponse is response type for the Query/FeeRate RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeRateResponse {
    #[prost(message, optional, tag = "1")]
    pub fee_rate: ::core::option::Option<FeeRate>,
}
/// QueryWithdrawalNetworkFeeRequest is request type for the Query/WithdrawalNetworkFee RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawalNetworkFeeRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub fee_rate: i64,
}
/// QueryWithdrawalNetworkFeeResponse is response type for the Query/WithdrawalNetworkFee RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawalNetworkFeeResponse {
    #[prost(int64, tag = "1")]
    pub fee_rate: i64,
    #[prost(string, tag = "2")]
    pub fee: ::prost::alloc::string::String,
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
/// QueryUTXOsRequest is the request type for the Query/UTXOs RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUtxOsRequest {}
/// QueryUTXOsResponse is the response type for the Query/UTXOs RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUtxOsResponse {
    #[prost(message, repeated, tag = "1")]
    pub utxos: ::prost::alloc::vec::Vec<Utxo>,
}
/// QueryUTXOsByAddressRequest is the request type for the Query/UTXOsByAddress RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUtxOsByAddressRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryUTXOsByAddressResponse is the response type for the Query/UTXOsByAddress RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUtxOsByAddressResponse {
    #[prost(message, repeated, tag = "1")]
    pub utxos: ::prost::alloc::vec::Vec<Utxo>,
}
/// QueryUTXOCountAndBalancesByAddressRequest is the request type for the Query/UTXOCountAndBalancesByAddress RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUtxoCountAndBalancesByAddressRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryUTXOCountAndBalancesByAddressResponse is the response type for the Query/UTXOCountAndBalancesByAddress RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUtxoCountAndBalancesByAddressResponse {
    #[prost(uint32, tag = "1")]
    pub count: u32,
    #[prost(int64, tag = "2")]
    pub value: i64,
    #[prost(message, repeated, tag = "3")]
    pub rune_balances: ::prost::alloc::vec::Vec<RuneBalance>,
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
    /// this is the relayer address who submits the bitcoin transaction to the side chain
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
    /// this is the relayer address who submits the bitcoin transaction to the side chain
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub blockhash: ::prost::alloc::string::String,
    /// the tx bytes in base64 format
    #[prost(string, tag = "3")]
    pub tx_bytes: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "4")]
    pub proof: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgSubmitWithdrawTransactionResponse defines the Msg/SubmitWithdrawTransaction response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitWithdrawTransactionResponse {}
/// MsgSubmitFeeRate defines the Msg/SubmitFeeRate request type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitFeeRate {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub fee_rate: i64,
}
/// MsgSubmitFeeRateResponse defines the Msg/SubmitFeeRate response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitFeeRateResponse {}
/// MsgUpdateTrustedNonBtcRelayers defines the Msg/UpdateTrustedNonBtcRelayers request type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateTrustedNonBtcRelayers {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgUpdateTrustedNonBtcRelayersResponse defines the Msg/UpdateTrustedNonBtcRelayers response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateTrustedNonBtcRelayersResponse {}
/// MsgUpdateTrustedFeeProviders defines the Msg/UpdateTrustedFeeProviders request type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateTrustedFeeProviders {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub fee_providers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgUpdateTrustedFeeProvidersResponse defines the Msg/UpdateTrustedFeeProviders response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateTrustedFeeProvidersResponse {}
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
/// MsgSubmitSignatures defines the Msg/SubmitSignatures request type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitSignatures {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub txid: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub psbt: ::prost::alloc::string::String,
}
/// MsgSubmitSignaturesResponse defines the Msg/SubmitSignatures response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitSignaturesResponse {}
/// MsgConsolidateVaults is the Msg/ConsolidateVaults request type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConsolidateVaults {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// vault version
    #[prost(uint64, tag = "2")]
    pub vault_version: u64,
    /// btc consolidation
    #[prost(message, optional, tag = "3")]
    pub btc_consolidation: ::core::option::Option<BtcConsolidation>,
    /// runes consolidations
    #[prost(message, repeated, tag = "4")]
    pub runes_consolidations: ::prost::alloc::vec::Vec<RunesConsolidation>,
}
/// MsgConsolidateVaultsResponse defines the Msg/ConsolidateVaults response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConsolidateVaultsResponse {}
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
    /// asset types of vaults to be generated
    #[prost(enumeration = "AssetType", repeated, tag = "4")]
    pub vault_types: ::prost::alloc::vec::Vec<i32>,
    /// indicates if transferring the current vaults to the newly generated vaults when the DKG request is completed
    #[prost(bool, tag = "5")]
    pub enable_transfer: bool,
    /// target number of the UTXOs to be transferred each time
    #[prost(uint32, tag = "6")]
    pub target_utxo_num: u32,
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
    /// consensus address of the corresponding validator
    #[prost(string, tag = "4")]
    pub consensus_address: ::prost::alloc::string::String,
    /// hex encoded validator signature
    #[prost(string, tag = "5")]
    pub signature: ::prost::alloc::string::String,
}
/// MsgCompleteDKGResponse defines the Msg/CompleteDKG response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCompleteDkgResponse {}
/// MsgTransferVault is the Msg/TransferVault request type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransferVault {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// version of the source vault
    #[prost(uint64, tag = "2")]
    pub source_version: u64,
    /// version of the destination vault
    #[prost(uint64, tag = "3")]
    pub dest_version: u64,
    /// asset type
    #[prost(enumeration = "AssetType", tag = "4")]
    pub asset_type: i32,
    /// a set of optional pre-built PSBTs to perform the asset transfer
    #[prost(string, repeated, tag = "5")]
    pub psbts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// target number of the UTXOs to be transferred; only take effect when psbt not provided
    #[prost(uint32, tag = "6")]
    pub target_utxo_num: u32,
}
/// MsgTransferVaultResponse defines the Msg/TransferVault response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransferVaultResponse {}
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
