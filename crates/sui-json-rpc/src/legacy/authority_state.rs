// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use arc_swap::Guard;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sui_json_rpc_types::SuiObjectDataFilter;
use sui_types::{
    base_types::{
        MoveObjectType, ObjectID, ObjectInfo, ObjectRef, SequenceNumber, SuiAddress, VersionNumber,
    },
    committee::EpochId,
    digests::{
        ChainIdentifier, CheckpointContentsDigest, CheckpointDigest, ObjectDigest,
        TransactionDigest, TransactionEventsDigest,
    },
    dynamic_field::DynamicFieldInfo,
    effects::{TransactionEffects, TransactionEvents},
    error::SuiResult,
    messages_checkpoint::{
        CertifiedCheckpointSummary, CheckpointContents, CheckpointSequenceNumber,
        VerifiedCheckpoint,
    },
    object::{Object, ObjectRead, PastObjectRead},
    storage::{BackingPackageStore, ObjectStore},
    transaction::Transaction,
    TypeTag,
};

use super::{
    authority_per_epoch_store::AuthorityPerEpochStore, committee_store::CommitteeStore,
    execution_cache::ExecutionCacheRead, index_store::IndexStore,
    subscription_handler::SubscriptionHandler,
    transaction_key_value_store::TransactionKeyValueStoreTrait,
};

pub type SuiLockResult = SuiResult<ObjectLockStatus>;

#[derive(Debug, PartialEq, Eq)]
pub enum ObjectLockStatus {
    Initialized,
    LockedToTx { locked_by_tx: LockDetails },
    LockedAtDifferentVersion { locked_ref: ObjectRef },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LockDetailsV1 {
    pub epoch: EpochId,
    pub tx_digest: TransactionDigest,
}

pub type LockDetails = LockDetailsV1;

pub struct CoinInfo {
    pub version: SequenceNumber,
    pub digest: ObjectDigest,
    pub balance: u64,
    pub previous_transaction: TransactionDigest,
}

/// The authority state encapsulates all state, drives execution, and ensures safety.
///
/// Note the authority operations can be accessed through a read ref (&) and do not
/// require &mut. Internally a database is synchronized through a mutex lock.
///
/// Repeating valid commands should produce no changes and return no error.
pub struct AuthorityState {
    pub subscription_handler: Arc<SubscriptionHandler>,
    pub indexes: Option<Arc<IndexStore>>,
}

#[async_trait]
impl TransactionKeyValueStoreTrait for AuthorityState {
    async fn multi_get(
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

    async fn multi_get_checkpoints(
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

    async fn deprecated_get_transaction_checkpoint(
        &self,
        _digest: TransactionDigest,
    ) -> SuiResult<Option<CheckpointSequenceNumber>> {
        unimplemented!()
    }

    async fn get_object(
        &self,
        _object_id: ObjectID,
        _version: VersionNumber,
    ) -> SuiResult<Option<Object>> {
        unimplemented!()
    }

    async fn multi_get_transaction_checkpoint(
        &self,
        _digests: &[TransactionDigest],
    ) -> SuiResult<Vec<Option<CheckpointSequenceNumber>>> {
        unimplemented!()
    }
}

impl AuthorityState {
    pub fn committee_store(&self) -> &Arc<CommitteeStore> {
        unimplemented!()
    }

    pub async fn get_object(&self, _: &ObjectID) -> SuiResult<Option<Object>> {
        unimplemented!()
    }

    pub fn get_past_object_read(
        &self,
        _object_id: &ObjectID,
        _version: SequenceNumber,
    ) -> SuiResult<PastObjectRead> {
        unimplemented!()
    }

    pub fn get_object_read(&self, _object_id: &ObjectID) -> SuiResult<ObjectRead> {
        unimplemented!()
    }

    pub fn get_owner_objects(
        &self,
        _owner: SuiAddress,
        // If `Some`, the query will start from the next item after the specified cursor
        _cursor: Option<ObjectID>,
        _limit: usize,
        _filter: Option<SuiObjectDataFilter>,
    ) -> SuiResult<Vec<ObjectInfo>> {
        unimplemented!()
    }

    pub fn get_owner_objects_iterator(
        &self,
        _owner: SuiAddress,
        // If `Some`, the query will start from the next item after the specified cursor
        _cursor: Option<ObjectID>,
        _filter: Option<SuiObjectDataFilter>,
    ) -> SuiResult<impl Iterator<Item = ObjectInfo> + '_> {
        Ok(Vec::new().into_iter())
    }

    pub fn get_owned_coins_iterator_with_cursor(
        &self,
        _owner: SuiAddress,
        // If `Some`, the query will start from the next item after the specified cursor
        _cursor: (String, ObjectID),
        _limit: usize,
        _one_coin_type_only: bool,
    ) -> SuiResult<impl Iterator<Item = (String, ObjectID, CoinInfo)> + '_> {
        Ok(Vec::new().into_iter())
    }

    pub async fn get_move_objects<T>(
        &self,
        _owner: SuiAddress,
        _type: MoveObjectType,
    ) -> SuiResult<Vec<T>>
    where
        T: DeserializeOwned,
    {
        unimplemented!()
    }

    /// Chain Identifier is the digest of the genesis checkpoint.
    pub fn get_chain_identifier(&self) -> Option<ChainIdentifier> {
        unimplemented!()
    }

    pub fn load_epoch_store_one_call_per_task(&self) -> Guard<Arc<AuthorityPerEpochStore>> {
        unimplemented!()
    }

    pub fn get_dynamic_fields(
        &self,
        _owner: ObjectID,
        // If `Some`, the query will start from the next item after the specified cursor
        _cursor: Option<ObjectID>,
        _limit: usize,
    ) -> SuiResult<Vec<(ObjectID, DynamicFieldInfo)>> {
        unimplemented!()
    }

    pub fn get_dynamic_field_object_id(
        &self,
        _owner: ObjectID,
        _name_type: TypeTag,
        _name_bcs_bytes: &[u8],
    ) -> SuiResult<Option<ObjectID>> {
        unimplemented!()
    }

    pub fn get_cache_reader(&self) -> &Arc<dyn ExecutionCacheRead> {
        unimplemented!()
    }

    pub fn get_object_store(&self) -> &Arc<dyn ObjectStore + Send + Sync> {
        unimplemented!()
    }

    pub fn get_backing_package_store(&self) -> &Arc<dyn BackingPackageStore + Send + Sync> {
        unimplemented!()
    }

    pub fn find_publish_txn_digest(&self, _package_id: ObjectID) -> SuiResult<TransactionDigest> {
        unimplemented!()
    }

    pub fn get_verified_checkpoint_by_sequence_number(
        &self,
        _sequence_number: CheckpointSequenceNumber,
    ) -> SuiResult<VerifiedCheckpoint> {
        unimplemented!()
    }

    pub fn get_checkpoint_contents(
        &self,
        _digest: CheckpointContentsDigest,
    ) -> SuiResult<CheckpointContents> {
        unimplemented!()
    }

    pub fn get_verified_checkpoint_summary_by_digest(
        &self,
        _digest: CheckpointDigest,
    ) -> SuiResult<VerifiedCheckpoint> {
        unimplemented!()
    }

    pub fn multi_get_checkpoint_by_sequence_number(
        &self,
        _sequence_numbers: &[CheckpointSequenceNumber],
    ) -> SuiResult<Vec<Option<VerifiedCheckpoint>>> {
        unimplemented!()
    }

    pub fn get_total_transaction_blocks(&self) -> SuiResult<u64> {
        unimplemented!()
    }

    pub fn get_checkpoint_by_sequence_number(
        &self,
        _sequence_number: CheckpointSequenceNumber,
    ) -> SuiResult<Option<VerifiedCheckpoint>> {
        unimplemented!()
    }

    pub fn get_latest_checkpoint_sequence_number(&self) -> SuiResult<CheckpointSequenceNumber> {
        unimplemented!()
    }

    pub fn loaded_child_object_versions(
        &self,
        _transaction_digest: &TransactionDigest,
    ) -> SuiResult<Option<Vec<(ObjectID, SequenceNumber)>>> {
        unimplemented!()
    }
}
