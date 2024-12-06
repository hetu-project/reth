//! Narwhal Consensus implementation.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/paradigmxyz/reth/main/assets/reth-docs.png",
    html_favicon_url = "https://avatars0.githubusercontent.com/u/97369466?s=256",
    issue_tracker_base_url = "https://github.com/paradigmxyz/reth/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]


use alloy_eips::{eip1898::BlockHashOrNumber, eip7685::Requests};
use alloy_primitives::{BlockHash, BlockNumber, Bloom, B256, U256};
use reth_beacon_consensus::BeaconEngineMessage;
use reth_chainspec::{EthChainSpec, EthereumHardforks};
use reth_consensus::{Consensus, ConsensusError, PostExecutionInput};
use reth_engine_primitives::EngineTypes;
use reth_execution_errors::{
    BlockExecutionError, BlockValidationError, InternalBlockExecutionError,
};
use reth_execution_types::ExecutionOutcome;
use reth_primitives::{
    proofs, Block, BlockBody, BlockWithSenders, Header, SealedBlock, SealedHeader,
    TransactionSigned, Withdrawals,
};
use reth_provider::{BlockReaderIdExt, StateProviderFactory, StateRootProvider};
use reth_revm::database::StateProviderDatabase;
use reth_transaction_pool::TransactionPool;
use reth_trie::HashedPostState;
use std::{
    collections::HashMap,
    fmt::Debug,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::sync::{mpsc::UnboundedSender, RwLock, RwLockReadGuard, RwLockWriteGuard};
use tracing::trace;

use reth_evm::execute::{BlockExecutorProvider, Executor};

/// A consensus implementation intended for local development and testing purposes.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct NarwhalConsensus<ChainSpec> {
    /// Configuration
    chain_spec: Arc<ChainSpec>,
}

impl<ChainSpec> NarwhalConsensus<ChainSpec> {
    /// Create a new instance of [`NarwhalConsensus`]
    pub const fn new(chain_spec: Arc<ChainSpec>) -> Self {
        Self { chain_spec }
    }
}

impl<ChainSpec: Send + Sync + Debug> Consensus for NarwhalConsensus<ChainSpec> {
    fn validate_header(&self, _header: &SealedHeader) -> Result<(), ConsensusError> {
        Ok(())
    }

    fn validate_header_against_parent(
        &self,
        _header: &SealedHeader,
        _parent: &SealedHeader,
    ) -> Result<(), ConsensusError> {
        Ok(())
    }

    fn validate_header_with_total_difficulty(
        &self,
        _header: &Header,
        _total_difficulty: U256,
    ) -> Result<(), ConsensusError> {
        Ok(())
    }

    fn validate_block_pre_execution(&self, _block: &SealedBlock) -> Result<(), ConsensusError> {
        Ok(())
    }

    fn validate_block_post_execution(
        &self,
        _block: &BlockWithSenders,
        _input: PostExecutionInput<'_>,
    ) -> Result<(), ConsensusError> {
        Ok(())
    }
}
