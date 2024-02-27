// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use sui_types::{
    base_types::{ObjectID, SequenceNumber, VersionNumber},
    digests::{
        CheckpointContentsDigest, CheckpointDigest, TransactionDigest, TransactionEventsDigest,
    },
    effects::{TransactionEffects, TransactionEvents},
    error::SuiResult,
    messages_checkpoint::{
        CertifiedCheckpointSummary, CheckpointContents, CheckpointSequenceNumber,
    },
    object::Object,
    transaction::Transaction,
};

use async_trait::async_trait;

pub type KVStoreTransactionData = (
    Vec<Option<Transaction>>,
    Vec<Option<TransactionEffects>>,
    Vec<Option<TransactionEvents>>,
);

pub type KVStoreCheckpointData = (
    Vec<Option<CertifiedCheckpointSummary>>,
    Vec<Option<CheckpointContents>>,
    Vec<Option<CertifiedCheckpointSummary>>,
    Vec<Option<CheckpointContents>>,
);

/// Immutable key/value store for storing/retrieving transactions, effects, and events
/// to/from a scalable.
pub struct TransactionKeyValueStore {}

impl TransactionKeyValueStore {
    pub fn new(
        _store_name: &'static str,
        _metrics: Arc<KeyValueStoreMetrics>,
        _inner: Arc<dyn TransactionKeyValueStoreTrait + Send + Sync>,
    ) -> Self {
        Self {}
    }

    /// Generic multi_get, allows implementors to get heterogenous values with a single round trip.
    pub async fn multi_get(
        &self,
        _transactions: &[TransactionDigest],
        _effects: &[TransactionDigest],
        _events: &[TransactionEventsDigest],
    ) -> SuiResult<(
        Vec<Option<Transaction>>,
        Vec<Option<TransactionEffects>>,
        Vec<Option<TransactionEvents>>,
    )> {
        unimplemented!()
    }

    pub async fn multi_get_checkpoints(
        &self,
        _checkpoint_summaries: &[CheckpointSequenceNumber],
        _checkpoint_contents: &[CheckpointSequenceNumber],
        _checkpoint_summaries_by_digest: &[CheckpointDigest],
        _checkpoint_contents_by_digest: &[CheckpointContentsDigest],
    ) -> SuiResult<(
        Vec<Option<CertifiedCheckpointSummary>>,
        Vec<Option<CheckpointContents>>,
        Vec<Option<CertifiedCheckpointSummary>>,
        Vec<Option<CheckpointContents>>,
    )> {
        unimplemented!()
    }

    pub async fn multi_get_checkpoints_summaries(
        &self,
        _keys: &[CheckpointSequenceNumber],
    ) -> SuiResult<Vec<Option<CertifiedCheckpointSummary>>> {
        unimplemented!()
    }

    pub async fn multi_get_checkpoints_contents(
        &self,
        _keys: &[CheckpointSequenceNumber],
    ) -> SuiResult<Vec<Option<CheckpointContents>>> {
        unimplemented!()
    }

    pub async fn multi_get_checkpoints_summaries_by_digest(
        &self,
        _keys: &[CheckpointDigest],
    ) -> SuiResult<Vec<Option<CertifiedCheckpointSummary>>> {
        unimplemented!()
    }

    pub async fn multi_get_checkpoints_contents_by_digest(
        &self,
        _keys: &[CheckpointContentsDigest],
    ) -> SuiResult<Vec<Option<CheckpointContents>>> {
        unimplemented!()
    }

    pub async fn multi_get_tx(
        &self,
        _keys: &[TransactionDigest],
    ) -> SuiResult<Vec<Option<Transaction>>> {
        unimplemented!()
    }

    pub async fn multi_get_fx_by_tx_digest(
        &self,
        _keys: &[TransactionDigest],
    ) -> SuiResult<Vec<Option<TransactionEffects>>> {
        unimplemented!()
    }

    pub async fn multi_get_events(
        &self,
        _keys: &[TransactionEventsDigest],
    ) -> SuiResult<Vec<Option<TransactionEvents>>> {
        unimplemented!()
    }

    /// Convenience method for fetching single digest, and returning an error if it's not found.
    /// Prefer using multi_get_tx whenever possible.
    pub async fn get_tx(&self, _digest: TransactionDigest) -> SuiResult<Transaction> {
        unimplemented!()
    }

    /// Convenience method for fetching single digest, and returning an error if it's not found.
    /// Prefer using multi_get_fx_by_tx_digest whenever possible.
    pub async fn get_fx_by_tx_digest(
        &self,
        _digest: TransactionDigest,
    ) -> SuiResult<TransactionEffects> {
        unimplemented!()
    }

    /// Convenience method for fetching single digest, and returning an error if it's not found.
    /// Prefer using multi_get_events whenever possible.
    pub async fn get_events(
        &self,
        _digest: TransactionEventsDigest,
    ) -> SuiResult<TransactionEvents> {
        unimplemented!()
    }

    /// Convenience method for fetching single checkpoint, and returning an error if it's not found.
    /// Prefer using multi_get_checkpoints_summaries whenever possible.
    pub async fn get_checkpoint_summary(
        &self,
        _checkpoint: CheckpointSequenceNumber,
    ) -> SuiResult<CertifiedCheckpointSummary> {
        unimplemented!()
    }

    /// Convenience method for fetching single checkpoint, and returning an error if it's not found.
    /// Prefer using multi_get_checkpoints_contents whenever possible.
    pub async fn get_checkpoint_contents(
        &self,
        _checkpoint: CheckpointSequenceNumber,
    ) -> SuiResult<CheckpointContents> {
        unimplemented!()
    }

    /// Convenience method for fetching single checkpoint, and returning an error if it's not found.
    /// Prefer using multi_get_checkpoints_summaries_by_digest whenever possible.
    pub async fn get_checkpoint_summary_by_digest(
        &self,
        _digest: CheckpointDigest,
    ) -> SuiResult<CertifiedCheckpointSummary> {
        unimplemented!()
    }

    /// Convenience method for fetching single checkpoint, and returning an error if it's not found.
    /// Prefer using multi_get_checkpoints_contents_by_digest whenever possible.
    pub async fn get_checkpoint_contents_by_digest(
        &self,
        _digest: CheckpointContentsDigest,
    ) -> SuiResult<CheckpointContents> {
        unimplemented!()
    }

    pub async fn deprecated_get_transaction_checkpoint(
        &self,
        _digest: TransactionDigest,
    ) -> SuiResult<Option<CheckpointSequenceNumber>> {
        unimplemented!()
    }

    pub async fn get_object(
        &self,
        _object_id: ObjectID,
        _version: VersionNumber,
    ) -> SuiResult<Option<Object>> {
        unimplemented!()
    }

    pub async fn multi_get_transaction_checkpoint(
        &self,
        _digests: &[TransactionDigest],
    ) -> SuiResult<Vec<Option<CheckpointSequenceNumber>>> {
        unimplemented!()
    }
}
pub struct KeyValueStoreMetrics {}

impl KeyValueStoreMetrics {
    pub fn new_for_tests() -> Arc<Self> {
        Arc::new(Self {})
    }
}

/// Immutable key/value store trait for storing/retrieving transactions, effects, and events.
/// Only defines multi_get/multi_put methods to discourage single key/value operations.
#[async_trait]
pub trait TransactionKeyValueStoreTrait {
    /// Generic multi_get, allows implementors to get heterogenous values with a single round trip.
    async fn multi_get(
        &self,
        transactions: &[TransactionDigest],
        effects: &[TransactionDigest],
        events: &[TransactionEventsDigest],
    ) -> SuiResult<KVStoreTransactionData>;

    /// Generic multi_get to allow implementors to get heterogenous values with a single round trip.
    async fn multi_get_checkpoints(
        &self,
        checkpoint_summaries: &[CheckpointSequenceNumber],
        checkpoint_contents: &[CheckpointSequenceNumber],
        checkpoint_summaries_by_digest: &[CheckpointDigest],
        checkpoint_contents_by_digest: &[CheckpointContentsDigest],
    ) -> SuiResult<KVStoreCheckpointData>;

    async fn deprecated_get_transaction_checkpoint(
        &self,
        digest: TransactionDigest,
    ) -> SuiResult<Option<CheckpointSequenceNumber>>;

    async fn get_object(
        &self,
        object_id: ObjectID,
        version: SequenceNumber,
    ) -> SuiResult<Option<Object>>;

    async fn multi_get_transaction_checkpoint(
        &self,
        digests: &[TransactionDigest],
    ) -> SuiResult<Vec<Option<CheckpointSequenceNumber>>>;
}
