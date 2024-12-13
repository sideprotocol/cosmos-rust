// @generated
// ----------------------------------------
// Request types

/// Request represents a request to the ABCI application.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    /// Sum of all possible messages.
    #[prost(
        oneof = "request::Value",
        tags = "1, 2, 3, 5, 6, 8, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20"
    )]
    pub value: ::core::option::Option<request::Value>,
}
/// Nested message and enum types in `Request`.
pub mod request {
    /// Sum of all possible messages.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(message, tag = "1")]
        Echo(super::EchoRequest),
        #[prost(message, tag = "2")]
        Flush(super::FlushRequest),
        #[prost(message, tag = "3")]
        Info(super::InfoRequest),
        #[prost(message, tag = "5")]
        InitChain(super::InitChainRequest),
        #[prost(message, tag = "6")]
        Query(super::QueryRequest),
        #[prost(message, tag = "8")]
        CheckTx(super::CheckTxRequest),
        #[prost(message, tag = "11")]
        Commit(super::CommitRequest),
        #[prost(message, tag = "12")]
        ListSnapshots(super::ListSnapshotsRequest),
        #[prost(message, tag = "13")]
        OfferSnapshot(super::OfferSnapshotRequest),
        #[prost(message, tag = "14")]
        LoadSnapshotChunk(super::LoadSnapshotChunkRequest),
        #[prost(message, tag = "15")]
        ApplySnapshotChunk(super::ApplySnapshotChunkRequest),
        #[prost(message, tag = "16")]
        PrepareProposal(super::PrepareProposalRequest),
        #[prost(message, tag = "17")]
        ProcessProposal(super::ProcessProposalRequest),
        #[prost(message, tag = "18")]
        ExtendVote(super::ExtendVoteRequest),
        #[prost(message, tag = "19")]
        VerifyVoteExtension(super::VerifyVoteExtensionRequest),
        #[prost(message, tag = "20")]
        FinalizeBlock(super::FinalizeBlockRequest),
    }
}
/// EchoRequest is a request to "echo" the given string.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EchoRequest {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
/// FlushRequest is a request to flush the write buffer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlushRequest {}
/// InfoRequest is a request for the ABCI application version.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoRequest {
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub block_version: u64,
    #[prost(uint64, tag = "3")]
    pub p2p_version: u64,
    #[prost(string, tag = "4")]
    pub abci_version: ::prost::alloc::string::String,
}
/// InitChainRequest is a request to initialize the blockchain.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitChainRequest {
    #[prost(message, optional, tag = "1")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "2")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub consensus_params: ::core::option::Option<super::super::types::v1::ConsensusParams>,
    #[prost(message, repeated, tag = "4")]
    pub validators: ::prost::alloc::vec::Vec<ValidatorUpdate>,
    #[prost(bytes = "vec", tag = "5")]
    pub app_state_bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "6")]
    pub initial_height: i64,
}
/// QueryRequest is a request to query the application state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub height: i64,
    #[prost(bool, tag = "4")]
    pub prove: bool,
}
/// CheckTxRequest is a request to check that the transaction is valid.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckTxRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub tx: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "CheckTxType", tag = "3")]
    pub r#type: i32,
}
/// CommitRequest is a request to commit the pending application state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitRequest {}
/// Request to list available snapshots.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSnapshotsRequest {}
/// Request offering a snapshot to the application.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfferSnapshotRequest {
    /// snapshot offered by peers
    #[prost(message, optional, tag = "1")]
    pub snapshot: ::core::option::Option<Snapshot>,
    /// light client-verified app hash for snapshot height
    #[prost(bytes = "vec", tag = "2")]
    pub app_hash: ::prost::alloc::vec::Vec<u8>,
}
/// Request to load a snapshot chunk.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadSnapshotChunkRequest {
    #[prost(uint64, tag = "1")]
    pub height: u64,
    #[prost(uint32, tag = "2")]
    pub format: u32,
    #[prost(uint32, tag = "3")]
    pub chunk: u32,
}
/// Request to apply a snapshot chunk.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplySnapshotChunkRequest {
    #[prost(uint32, tag = "1")]
    pub index: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub chunk: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub sender: ::prost::alloc::string::String,
}
/// PrepareProposalRequest is a request for the ABCI application to prepare a new
/// block proposal.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareProposalRequest {
    /// the modified transactions cannot exceed this size.
    #[prost(int64, tag = "1")]
    pub max_tx_bytes: i64,
    /// txs is an array of transactions that will be included in a block,
    /// sent to the app for possible modifications.
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "3")]
    pub local_last_commit: ::core::option::Option<ExtendedCommitInfo>,
    #[prost(message, repeated, tag = "4")]
    pub misbehavior: ::prost::alloc::vec::Vec<Misbehavior>,
    #[prost(int64, tag = "5")]
    pub height: i64,
    #[prost(message, optional, tag = "6")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bytes = "vec", tag = "7")]
    pub next_validators_hash: ::prost::alloc::vec::Vec<u8>,
    /// address of the public key of the validator proposing the block.
    #[prost(bytes = "vec", tag = "8")]
    pub proposer_address: ::prost::alloc::vec::Vec<u8>,
}
/// ProcessProposalRequest is a request for the ABCI application to process a proposal
/// received from another validator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessProposalRequest {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "2")]
    pub proposed_last_commit: ::core::option::Option<CommitInfo>,
    #[prost(message, repeated, tag = "3")]
    pub misbehavior: ::prost::alloc::vec::Vec<Misbehavior>,
    /// Merkle root hash of the fields of the proposed block.
    #[prost(bytes = "vec", tag = "4")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "5")]
    pub height: i64,
    #[prost(message, optional, tag = "6")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bytes = "vec", tag = "7")]
    pub next_validators_hash: ::prost::alloc::vec::Vec<u8>,
    /// address of the public key of the original proposer of the block.
    #[prost(bytes = "vec", tag = "8")]
    pub proposer_address: ::prost::alloc::vec::Vec<u8>,
}
/// ExtendVoteRequest extends a precommit vote with application-injected data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtendVoteRequest {
    /// the hash of the block that this vote may be referring to
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// the height of the extended vote
    #[prost(int64, tag = "2")]
    pub height: i64,
    /// info of the block that this vote may be referring to
    #[prost(message, optional, tag = "3")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "5")]
    pub proposed_last_commit: ::core::option::Option<CommitInfo>,
    #[prost(message, repeated, tag = "6")]
    pub misbehavior: ::prost::alloc::vec::Vec<Misbehavior>,
    #[prost(bytes = "vec", tag = "7")]
    pub next_validators_hash: ::prost::alloc::vec::Vec<u8>,
    /// address of the public key of the original proposer of the block.
    #[prost(bytes = "vec", tag = "8")]
    pub proposer_address: ::prost::alloc::vec::Vec<u8>,
}
/// VerifyVoteExtensionRequest is a request for the application to verify a vote extension
/// produced by a different validator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyVoteExtensionRequest {
    /// the hash of the block that this received vote corresponds to
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// the validator that signed the vote extension
    #[prost(bytes = "vec", tag = "2")]
    pub validator_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "3")]
    pub height: i64,
    #[prost(bytes = "vec", tag = "4")]
    pub vote_extension: ::prost::alloc::vec::Vec<u8>,
}
/// FinalizeBlockRequest is a request to finalize the block.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizeBlockRequest {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag = "2")]
    pub decided_last_commit: ::core::option::Option<CommitInfo>,
    #[prost(message, repeated, tag = "3")]
    pub misbehavior: ::prost::alloc::vec::Vec<Misbehavior>,
    /// Merkle root hash of the fields of the decided block.
    #[prost(bytes = "vec", tag = "4")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "5")]
    pub height: i64,
    #[prost(message, optional, tag = "6")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bytes = "vec", tag = "7")]
    pub next_validators_hash: ::prost::alloc::vec::Vec<u8>,
    /// address of the public key of the original proposer of the block.
    #[prost(bytes = "vec", tag = "8")]
    pub proposer_address: ::prost::alloc::vec::Vec<u8>,
}
// ----------------------------------------
// Response types

/// Response represents a response from the ABCI application.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    /// Sum of all possible messages.
    #[prost(
        oneof = "response::Value",
        tags = "1, 2, 3, 4, 6, 7, 9, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21"
    )]
    pub value: ::core::option::Option<response::Value>,
}
/// Nested message and enum types in `Response`.
pub mod response {
    /// Sum of all possible messages.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(message, tag = "1")]
        Exception(super::ExceptionResponse),
        #[prost(message, tag = "2")]
        Echo(super::EchoResponse),
        #[prost(message, tag = "3")]
        Flush(super::FlushResponse),
        #[prost(message, tag = "4")]
        Info(super::InfoResponse),
        #[prost(message, tag = "6")]
        InitChain(super::InitChainResponse),
        #[prost(message, tag = "7")]
        Query(super::QueryResponse),
        #[prost(message, tag = "9")]
        CheckTx(super::CheckTxResponse),
        #[prost(message, tag = "12")]
        Commit(super::CommitResponse),
        #[prost(message, tag = "13")]
        ListSnapshots(super::ListSnapshotsResponse),
        #[prost(message, tag = "14")]
        OfferSnapshot(super::OfferSnapshotResponse),
        #[prost(message, tag = "15")]
        LoadSnapshotChunk(super::LoadSnapshotChunkResponse),
        #[prost(message, tag = "16")]
        ApplySnapshotChunk(super::ApplySnapshotChunkResponse),
        #[prost(message, tag = "17")]
        PrepareProposal(super::PrepareProposalResponse),
        #[prost(message, tag = "18")]
        ProcessProposal(super::ProcessProposalResponse),
        #[prost(message, tag = "19")]
        ExtendVote(super::ExtendVoteResponse),
        #[prost(message, tag = "20")]
        VerifyVoteExtension(super::VerifyVoteExtensionResponse),
        #[prost(message, tag = "21")]
        FinalizeBlock(super::FinalizeBlockResponse),
    }
}
/// nondeterministic
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExceptionResponse {
    #[prost(string, tag = "1")]
    pub error: ::prost::alloc::string::String,
}
/// EchoResponse indicates that the connection is still alive.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EchoResponse {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
/// FlushResponse indicates that the write buffer was flushed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlushResponse {}
/// InfoResponse contains the ABCI application version information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoResponse {
    #[prost(string, tag = "1")]
    pub data: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub app_version: u64,
    #[prost(int64, tag = "4")]
    pub last_block_height: i64,
    #[prost(bytes = "vec", tag = "5")]
    pub last_block_app_hash: ::prost::alloc::vec::Vec<u8>,
}
/// InitChainResponse contains the ABCI application's hash and updates to the
/// validator set and/or the consensus params, if any.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitChainResponse {
    #[prost(message, optional, tag = "1")]
    pub consensus_params: ::core::option::Option<super::super::types::v1::ConsensusParams>,
    #[prost(message, repeated, tag = "2")]
    pub validators: ::prost::alloc::vec::Vec<ValidatorUpdate>,
    #[prost(bytes = "vec", tag = "3")]
    pub app_hash: ::prost::alloc::vec::Vec<u8>,
}
/// QueryResponse contains the ABCI application data along with a proof.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResponse {
    #[prost(uint32, tag = "1")]
    pub code: u32,
    /// bytes data = 2; // use "value" instead.
    ///
    /// nondeterministic
    #[prost(string, tag = "3")]
    pub log: ::prost::alloc::string::String,
    /// nondeterministic
    #[prost(string, tag = "4")]
    pub info: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub index: i64,
    #[prost(bytes = "vec", tag = "6")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "7")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "8")]
    pub proof_ops: ::core::option::Option<super::super::crypto::v1::ProofOps>,
    #[prost(int64, tag = "9")]
    pub height: i64,
    #[prost(string, tag = "10")]
    pub codespace: ::prost::alloc::string::String,
}
/// CheckTxResponse shows if the transaction was deemed valid by the ABCI
/// application.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckTxResponse {
    #[prost(uint32, tag = "1")]
    pub code: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// nondeterministic
    #[prost(string, tag = "3")]
    pub log: ::prost::alloc::string::String,
    /// nondeterministic
    #[prost(string, tag = "4")]
    pub info: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub gas_wanted: i64,
    #[prost(int64, tag = "6")]
    pub gas_used: i64,
    /// nondeterministic
    #[prost(message, repeated, tag = "7")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    #[prost(string, tag = "8")]
    pub codespace: ::prost::alloc::string::String,
}
/// CommitResponse indicates how much blocks should CometBFT retain.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitResponse {
    #[prost(int64, tag = "3")]
    pub retain_height: i64,
}
/// ListSnapshotsResponse contains the list of snapshots.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSnapshotsResponse {
    #[prost(message, repeated, tag = "1")]
    pub snapshots: ::prost::alloc::vec::Vec<Snapshot>,
}
/// OfferSnapshotResponse indicates the ABCI application decision whenever to
/// provide a snapshot to the requester or not.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfferSnapshotResponse {
    #[prost(enumeration = "OfferSnapshotResult", tag = "1")]
    pub result: i32,
}
/// LoadSnapshotChunkResponse returns a snapshot's chunk.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadSnapshotChunkResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub chunk: ::prost::alloc::vec::Vec<u8>,
}
/// ApplySnapshotChunkResponse returns a result of applying the specified chunk.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplySnapshotChunkResponse {
    #[prost(enumeration = "ApplySnapshotChunkResult", tag = "1")]
    pub result: i32,
    /// Chunks to refetch and reapply
    #[prost(uint32, repeated, tag = "2")]
    pub refetch_chunks: ::prost::alloc::vec::Vec<u32>,
    /// Chunk senders to reject and ban
    #[prost(string, repeated, tag = "3")]
    pub reject_senders: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PrepareProposalResponse contains a list of transactions, which will form a block.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareProposalResponse {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// ProcessProposalResponse indicates the ABCI application's decision whenever
/// the given proposal should be accepted or not.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessProposalResponse {
    #[prost(enumeration = "ProcessProposalStatus", tag = "1")]
    pub status: i32,
}
/// ExtendVoteResponse contains the vote extension that the application would like to
/// attach to its next precommit vote.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtendVoteResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub vote_extension: ::prost::alloc::vec::Vec<u8>,
}
/// VerifyVoteExtensionResponse indicates the ABCI application's decision
/// whenever the vote extension should be accepted or not.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyVoteExtensionResponse {
    #[prost(enumeration = "VerifyVoteExtensionStatus", tag = "1")]
    pub status: i32,
}
/// FinalizeBlockResponse contains the result of executing the block.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizeBlockResponse {
    /// set of block events emitted as part of executing the block
    ///
    /// nondeterministic
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    /// the result of executing each transaction including the events
    /// the particular transaction emitted. This should match the order
    /// of the transactions delivered in the block itself
    #[prost(message, repeated, tag = "2")]
    pub tx_results: ::prost::alloc::vec::Vec<ExecTxResult>,
    /// a list of updates to the validator set. These will reflect the validator set at current height + 2.
    #[prost(message, repeated, tag = "3")]
    pub validator_updates: ::prost::alloc::vec::Vec<ValidatorUpdate>,
    /// updates to the consensus params, if any.
    #[prost(message, optional, tag = "4")]
    pub consensus_param_updates: ::core::option::Option<super::super::types::v1::ConsensusParams>,
    /// app_hash is the hash of the applications' state which is used to confirm
    /// that execution of the transactions was deterministic.
    /// It is up to the application to decide which algorithm to use.
    #[prost(bytes = "vec", tag = "5")]
    pub app_hash: ::prost::alloc::vec::Vec<u8>,
}
// ----------------------------------------
// Misc.

/// CommitInfo contains votes for the particular round.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitInfo {
    #[prost(int32, tag = "1")]
    pub round: i32,
    #[prost(message, repeated, tag = "2")]
    pub votes: ::prost::alloc::vec::Vec<VoteInfo>,
}
/// ExtendedCommitInfo is similar to CommitInfo except that it is only used in
/// the PrepareProposal request such that Tendermint can provide vote extensions
/// to the application.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtendedCommitInfo {
    /// The round at which the block proposer decided in the previous height.
    #[prost(int32, tag = "1")]
    pub round: i32,
    /// List of validators' addresses in the last validator set with their voting
    /// information, including vote extensions.
    #[prost(message, repeated, tag = "2")]
    pub votes: ::prost::alloc::vec::Vec<ExtendedVoteInfo>,
}
/// Event allows application developers to attach additional information to
/// ResponseFinalizeBlock and ResponseCheckTx.
/// Up to 0.37, this could also be used in ResponseBeginBlock, ResponseEndBlock,
/// and ResponseDeliverTx.
/// Later, transactions may be queried using these events.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub attributes: ::prost::alloc::vec::Vec<EventAttribute>,
}
/// EventAttribute is a single key-value pair, associated with an event.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAttribute {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
    /// nondeterministic
    #[prost(bool, tag = "3")]
    pub index: bool,
}
/// ExecTxResult contains results of executing one individual transaction.
///
/// * Its structure is equivalent to #ResponseDeliverTx which will be deprecated/deleted
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecTxResult {
    #[prost(uint32, tag = "1")]
    pub code: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// nondeterministic
    #[prost(string, tag = "3")]
    pub log: ::prost::alloc::string::String,
    /// nondeterministic
    #[prost(string, tag = "4")]
    pub info: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub gas_wanted: i64,
    #[prost(int64, tag = "6")]
    pub gas_used: i64,
    /// nondeterministic
    #[prost(message, repeated, tag = "7")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    #[prost(string, tag = "8")]
    pub codespace: ::prost::alloc::string::String,
}
/// TxResult contains results of executing the transaction.
///
/// One usage is indexing transaction results.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxResult {
    #[prost(int64, tag = "1")]
    pub height: i64,
    #[prost(uint32, tag = "2")]
    pub index: u32,
    #[prost(bytes = "vec", tag = "3")]
    pub tx: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "4")]
    pub result: ::core::option::Option<ExecTxResult>,
}
// ----------------------------------------
// Blockchain Types

/// Validator in the validator set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Validator {
    /// The first 20 bytes of SHA256(public key)
    #[prost(bytes = "vec", tag = "1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    /// PubKey pub_key = 2 \[(gogoproto.nullable)=false\];
    ///
    /// The voting power
    #[prost(int64, tag = "3")]
    pub power: i64,
}
/// ValidatorUpdate is a singular update to a validator set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorUpdate {
    #[prost(message, optional, tag = "1")]
    pub pub_key: ::core::option::Option<super::super::crypto::v1::PublicKey>,
    #[prost(int64, tag = "2")]
    pub power: i64,
}
/// VoteInfo contains the information about the vote.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteInfo {
    #[prost(message, optional, tag = "1")]
    pub validator: ::core::option::Option<Validator>,
    #[prost(enumeration = "super::super::types::v1::BlockIdFlag", tag = "3")]
    pub block_id_flag: i32,
}
/// ExtendedVoteInfo extends VoteInfo with the vote extensions (non-deterministic).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtendedVoteInfo {
    /// The validator that sent the vote.
    #[prost(message, optional, tag = "1")]
    pub validator: ::core::option::Option<Validator>,
    /// Non-deterministic extension provided by the sending validator's application.
    #[prost(bytes = "vec", tag = "3")]
    pub vote_extension: ::prost::alloc::vec::Vec<u8>,
    /// Vote extension signature created by CometBFT
    #[prost(bytes = "vec", tag = "4")]
    pub extension_signature: ::prost::alloc::vec::Vec<u8>,
    /// block_id_flag indicates whether the validator voted for a block, nil, or did not vote at all
    #[prost(enumeration = "super::super::types::v1::BlockIdFlag", tag = "5")]
    pub block_id_flag: i32,
}
/// Misbehavior is a type of misbehavior committed by a validator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Misbehavior {
    #[prost(enumeration = "MisbehaviorType", tag = "1")]
    pub r#type: i32,
    /// The offending validator
    #[prost(message, optional, tag = "2")]
    pub validator: ::core::option::Option<Validator>,
    /// The height when the offense occurred
    #[prost(int64, tag = "3")]
    pub height: i64,
    /// The corresponding time where the offense occurred
    #[prost(message, optional, tag = "4")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// Total voting power of the validator set in case the ABCI application does
    /// not store historical validators.
    /// <https://github.com/tendermint/tendermint/issues/4581>
    #[prost(int64, tag = "5")]
    pub total_voting_power: i64,
}
// ----------------------------------------
// State Sync Types

/// Snapshot of the ABCI application state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Snapshot {
    /// The height at which the snapshot was taken
    #[prost(uint64, tag = "1")]
    pub height: u64,
    /// The application-specific snapshot format
    #[prost(uint32, tag = "2")]
    pub format: u32,
    /// Number of chunks in the snapshot
    #[prost(uint32, tag = "3")]
    pub chunks: u32,
    /// Arbitrary snapshot hash, equal only if identical
    #[prost(bytes = "vec", tag = "4")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// Arbitrary application metadata
    #[prost(bytes = "vec", tag = "5")]
    pub metadata: ::prost::alloc::vec::Vec<u8>,
}
/// Type of the transaction check request.
///
/// This enumeration is incompatible with the CheckTxType definition in
/// cometbft.abci.v1beta1 and therefore shall not be used in encoding with the same
/// field number.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CheckTxType {
    /// Unknown
    Unknown = 0,
    /// Recheck (2nd, 3rd, etc.)
    Recheck = 1,
    /// Check (1st time)
    Check = 2,
}
impl CheckTxType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CheckTxType::Unknown => "CHECK_TX_TYPE_UNKNOWN",
            CheckTxType::Recheck => "CHECK_TX_TYPE_RECHECK",
            CheckTxType::Check => "CHECK_TX_TYPE_CHECK",
        }
    }
}
/// The result of offering a snapshot.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OfferSnapshotResult {
    /// Unknown result, abort all snapshot restoration
    Unknown = 0,
    /// Snapshot accepted, apply chunks
    Accept = 1,
    /// Abort all snapshot restoration
    Abort = 2,
    /// Reject this specific snapshot, try others
    Reject = 3,
    /// Reject all snapshots of this format, try others
    RejectFormat = 4,
    /// Reject all snapshots from the sender(s), try others
    RejectSender = 5,
}
impl OfferSnapshotResult {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OfferSnapshotResult::Unknown => "OFFER_SNAPSHOT_RESULT_UNKNOWN",
            OfferSnapshotResult::Accept => "OFFER_SNAPSHOT_RESULT_ACCEPT",
            OfferSnapshotResult::Abort => "OFFER_SNAPSHOT_RESULT_ABORT",
            OfferSnapshotResult::Reject => "OFFER_SNAPSHOT_RESULT_REJECT",
            OfferSnapshotResult::RejectFormat => "OFFER_SNAPSHOT_RESULT_REJECT_FORMAT",
            OfferSnapshotResult::RejectSender => "OFFER_SNAPSHOT_RESULT_REJECT_SENDER",
        }
    }
}
/// The result of applying a snapshot chunk.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ApplySnapshotChunkResult {
    /// Unknown result, abort all snapshot restoration
    Unknown = 0,
    /// Chunk successfully accepted
    Accept = 1,
    /// Abort all snapshot restoration
    Abort = 2,
    /// Retry chunk (combine with refetch and reject)
    Retry = 3,
    /// Retry snapshot (combine with refetch and reject)
    RetrySnapshot = 4,
    /// Reject this snapshot, try others
    RejectSnapshot = 5,
}
impl ApplySnapshotChunkResult {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ApplySnapshotChunkResult::Unknown => "APPLY_SNAPSHOT_CHUNK_RESULT_UNKNOWN",
            ApplySnapshotChunkResult::Accept => "APPLY_SNAPSHOT_CHUNK_RESULT_ACCEPT",
            ApplySnapshotChunkResult::Abort => "APPLY_SNAPSHOT_CHUNK_RESULT_ABORT",
            ApplySnapshotChunkResult::Retry => "APPLY_SNAPSHOT_CHUNK_RESULT_RETRY",
            ApplySnapshotChunkResult::RetrySnapshot => "APPLY_SNAPSHOT_CHUNK_RESULT_RETRY_SNAPSHOT",
            ApplySnapshotChunkResult::RejectSnapshot => {
                "APPLY_SNAPSHOT_CHUNK_RESULT_REJECT_SNAPSHOT"
            }
        }
    }
}
/// ProcessProposalStatus is the status of the proposal processing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProcessProposalStatus {
    /// Unknown
    Unknown = 0,
    /// Accepted
    Accept = 1,
    /// Rejected
    Reject = 2,
}
impl ProcessProposalStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProcessProposalStatus::Unknown => "PROCESS_PROPOSAL_STATUS_UNKNOWN",
            ProcessProposalStatus::Accept => "PROCESS_PROPOSAL_STATUS_ACCEPT",
            ProcessProposalStatus::Reject => "PROCESS_PROPOSAL_STATUS_REJECT",
        }
    }
}
/// VerifyVoteExtensionStatus is the status of the vote extension verification.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VerifyVoteExtensionStatus {
    /// Unknown
    Unknown = 0,
    /// Accepted
    Accept = 1,
    /// Rejecting the vote extension will reject the entire precommit by the sender.
    /// Incorrectly implementing this thus has liveness implications as it may affect
    /// CometBFT's ability to receive 2/3+ valid votes to finalize the block.
    /// Honest nodes should never be rejected.
    Reject = 2,
}
impl VerifyVoteExtensionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VerifyVoteExtensionStatus::Unknown => "VERIFY_VOTE_EXTENSION_STATUS_UNKNOWN",
            VerifyVoteExtensionStatus::Accept => "VERIFY_VOTE_EXTENSION_STATUS_ACCEPT",
            VerifyVoteExtensionStatus::Reject => "VERIFY_VOTE_EXTENSION_STATUS_REJECT",
        }
    }
}
/// The type of misbehavior committed by a validator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MisbehaviorType {
    /// Unknown
    Unknown = 0,
    /// Duplicate vote
    DuplicateVote = 1,
    /// Light client attack
    LightClientAttack = 2,
}
impl MisbehaviorType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MisbehaviorType::Unknown => "MISBEHAVIOR_TYPE_UNKNOWN",
            MisbehaviorType::DuplicateVote => "MISBEHAVIOR_TYPE_DUPLICATE_VOTE",
            MisbehaviorType::LightClientAttack => "MISBEHAVIOR_TYPE_LIGHT_CLIENT_ATTACK",
        }
    }
}
// @@protoc_insertion_point(module)
