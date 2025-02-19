// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use diesel::{prelude::*, r2d2::R2D2Connection};
use iota_json_rpc_types::{
    BalanceChange, IotaEvent, IotaTransactionBlock, IotaTransactionBlockEffects,
    IotaTransactionBlockEvents, IotaTransactionBlockResponse, IotaTransactionBlockResponseOptions,
    ObjectChange,
};
use iota_package_resolver::{PackageStore, Resolver};
use iota_types::{
    digests::TransactionDigest,
    effects::{TransactionEffects, TransactionEvents},
    event::Event,
    transaction::SenderSignedData,
};
use move_core_types::{
    annotated_value::{MoveDatatypeLayout, MoveTypeLayout},
    language_storage::TypeTag,
};

use crate::{
    db::ConnectionPool,
    errors::{Context, IndexerError},
    models::large_objects::{get_large_object_in_chunks, put_large_object_in_chunks},
    schema::transactions,
    types::{IndexedObjectChange, IndexedTransaction, IndexerResult},
};

#[derive(Clone, Debug, Queryable, Insertable, QueryableByName, Selectable)]
#[diesel(table_name = transactions)]
pub struct StoredTransaction {
    /// The index of the transaction in the global ordering that starts
    /// from genesis.
    pub tx_sequence_number: i64,
    pub transaction_digest: Vec<u8>,
    pub raw_transaction: Vec<u8>,
    pub raw_effects: Vec<u8>,
    pub checkpoint_sequence_number: i64,
    pub timestamp_ms: i64,
    #[cfg(feature = "postgres-feature")]
    pub object_changes: Vec<Option<Vec<u8>>>,
    #[cfg(feature = "mysql-feature")]
    #[cfg(not(feature = "postgres-feature"))]
    #[diesel(sql_type = diesel::sql_types::Json)]
    pub object_changes: serde_json::Value,
    #[cfg(feature = "postgres-feature")]
    pub balance_changes: Vec<Option<Vec<u8>>>,
    #[cfg(feature = "mysql-feature")]
    #[cfg(not(feature = "postgres-feature"))]
    #[diesel(sql_type = diesel::sql_types::Json)]
    pub balance_changes: serde_json::Value,
    #[cfg(feature = "postgres-feature")]
    pub events: Vec<Option<Vec<u8>>>,
    #[cfg(feature = "mysql-feature")]
    #[cfg(not(feature = "postgres-feature"))]
    #[diesel(sql_type = diesel::sql_types::Json)]
    pub events: serde_json::Value,
    pub transaction_kind: i16,
    pub success_command_count: i16,
}

#[cfg(feature = "postgres-feature")]
pub type StoredTransactionEvents = Vec<Option<Vec<u8>>>;

#[cfg(feature = "mysql-feature")]
#[cfg(not(feature = "postgres-feature"))]
pub type StoredTransactionEvents = serde_json::Value;

#[derive(Debug, Queryable)]
pub struct TxSeq {
    pub seq: i64,
}

impl Default for TxSeq {
    fn default() -> Self {
        Self { seq: -1 }
    }
}

#[derive(Clone, Debug, Queryable)]
pub struct StoredTransactionTimestamp {
    pub tx_sequence_number: i64,
    pub timestamp_ms: i64,
}

#[derive(Clone, Debug, Queryable)]
pub struct StoredTransactionCheckpoint {
    pub tx_sequence_number: i64,
    pub checkpoint_sequence_number: i64,
}

#[derive(Clone, Debug, Queryable)]
pub struct StoredTransactionSuccessCommandCount {
    pub tx_sequence_number: i64,
    pub checkpoint_sequence_number: i64,
    pub success_command_count: i16,
    pub timestamp_ms: i64,
}

impl From<&IndexedTransaction> for StoredTransaction {
    fn from(tx: &IndexedTransaction) -> Self {
        StoredTransaction {
            tx_sequence_number: tx.tx_sequence_number as i64,
            transaction_digest: tx.tx_digest.into_inner().to_vec(),
            raw_transaction: bcs::to_bytes(&tx.sender_signed_data).unwrap(),
            raw_effects: bcs::to_bytes(&tx.effects).unwrap(),
            checkpoint_sequence_number: tx.checkpoint_sequence_number as i64,
            #[cfg(feature = "postgres-feature")]
            object_changes: tx
                .object_changes
                .iter()
                .map(|oc| Some(bcs::to_bytes(&oc).unwrap()))
                .collect(),
            #[cfg(feature = "mysql-feature")]
            #[cfg(not(feature = "postgres-feature"))]
            object_changes: serde_json::to_value(
                tx.object_changes
                    .iter()
                    .map(|oc| bcs::to_bytes(&oc).unwrap())
                    .collect::<Vec<Vec<u8>>>(),
            )
            .unwrap(),
            #[cfg(feature = "postgres-feature")]
            balance_changes: tx
                .balance_change
                .iter()
                .map(|bc| Some(bcs::to_bytes(&bc).unwrap()))
                .collect(),
            #[cfg(feature = "mysql-feature")]
            #[cfg(not(feature = "postgres-feature"))]
            balance_changes: serde_json::to_value(
                tx.balance_change
                    .iter()
                    .map(|bc| bcs::to_bytes(&bc).unwrap())
                    .collect::<Vec<Vec<u8>>>(),
            )
            .unwrap(),
            #[cfg(feature = "postgres-feature")]
            events: tx
                .events
                .iter()
                .map(|e| Some(bcs::to_bytes(&e).unwrap()))
                .collect(),
            #[cfg(feature = "mysql-feature")]
            #[cfg(not(feature = "postgres-feature"))]
            events: serde_json::to_value(
                tx.events
                    .iter()
                    .map(|e| bcs::to_bytes(&e).unwrap())
                    .collect::<Vec<Vec<u8>>>(),
            )
            .unwrap(),
            timestamp_ms: tx.timestamp_ms as i64,
            transaction_kind: tx.transaction_kind.clone() as i16,
            success_command_count: tx.successful_tx_num as i16,
        }
    }
}

impl StoredTransaction {
    pub fn get_balance_len(&self) -> usize {
        #[cfg(feature = "postgres-feature")]
        {
            self.balance_changes.len()
        }
        #[cfg(feature = "mysql-feature")]
        #[cfg(not(feature = "postgres-feature"))]
        {
            self.balance_changes.as_array().unwrap().len()
        }
    }

    pub fn get_balance_at_idx(&self, idx: usize) -> Option<Vec<u8>> {
        #[cfg(feature = "postgres-feature")]
        {
            self.balance_changes.get(idx).cloned().flatten()
        }
        #[cfg(feature = "mysql-feature")]
        #[cfg(not(feature = "postgres-feature"))]
        {
            self.balance_changes.as_array().unwrap()[idx]
                .as_str()
                .map(|s| s.as_bytes().to_vec())
        }
    }

    pub fn get_object_len(&self) -> usize {
        #[cfg(feature = "postgres-feature")]
        {
            self.object_changes.len()
        }
        #[cfg(feature = "mysql-feature")]
        #[cfg(not(feature = "postgres-feature"))]
        {
            self.object_changes.as_array().unwrap().len()
        }
    }

    pub fn get_object_at_idx(&self, idx: usize) -> Option<Vec<u8>> {
        #[cfg(feature = "postgres-feature")]
        {
            self.object_changes.get(idx).cloned().flatten()
        }
        #[cfg(feature = "mysql-feature")]
        #[cfg(not(feature = "postgres-feature"))]
        {
            self.object_changes.as_array().unwrap()[idx]
                .as_str()
                .map(|s| s.as_bytes().to_vec())
        }
    }

    pub fn get_event_len(&self) -> usize {
        #[cfg(feature = "postgres-feature")]
        {
            self.events.len()
        }
        #[cfg(feature = "mysql-feature")]
        #[cfg(not(feature = "postgres-feature"))]
        {
            self.events.as_array().unwrap().len()
        }
    }

    pub fn get_event_at_idx(&self, idx: usize) -> Option<Vec<u8>> {
        #[cfg(feature = "postgres-feature")]
        {
            self.events.get(idx).cloned().flatten()
        }
        #[cfg(feature = "mysql-feature")]
        #[cfg(not(feature = "postgres-feature"))]
        {
            self.events.as_array().unwrap()[idx]
                .as_str()
                .map(|s| s.as_bytes().to_vec())
        }
    }

    const LARGE_OBJECT_CHUNK_SIZE: usize = 100 * 1024 * 1024;
    pub async fn try_into_iota_transaction_block_response(
        self,
        options: IotaTransactionBlockResponseOptions,
        package_resolver: Arc<Resolver<impl PackageStore>>,
    ) -> IndexerResult<IotaTransactionBlockResponse> {
        let options = options.clone();
        let tx_digest =
            TransactionDigest::try_from(self.transaction_digest.as_slice()).map_err(|e| {
                IndexerError::PersistentStorageDataCorruption(format!(
                    "Can't convert {:?} as tx_digest. Error: {e}",
                    self.transaction_digest
                ))
            })?;

        let transaction = if options.show_input {
            let sender_signed_data = self.try_into_sender_signed_data()?;
            let tx_block = IotaTransactionBlock::try_from_with_package_resolver(
                sender_signed_data,
                package_resolver.clone(),
                tx_digest,
            )
            .await?;
            Some(tx_block)
        } else {
            None
        };

        let effects = options
            .show_effects
            .then(|| self.try_into_iota_transaction_effects())
            .transpose()?;

        let raw_transaction = options
            .show_raw_input
            .then_some(self.raw_transaction)
            .unwrap_or_default();

        let events = if options.show_events {
            let events = {
                #[cfg(feature = "postgres-feature")]
                {
                    self
                        .events
                        .into_iter()
                        .map(|event| match event {
                            Some(event) => {
                                let event: Event = bcs::from_bytes(&event).map_err(|e| {
                                    IndexerError::PersistentStorageDataCorruption(format!(
                                        "Can't convert event bytes into Event. tx_digest={:?} Error: {e}",
                                        tx_digest
                                    ))
                                })?;
                                Ok(event)
                            }
                            None => Err(IndexerError::PersistentStorageDataCorruption(format!(
                                "Event should not be null, tx_digest={:?}",
                                tx_digest
                            ))),
                        })
                        .collect::<Result<Vec<Event>, IndexerError>>()?
                }
                #[cfg(feature = "mysql-feature")]
                #[cfg(not(feature = "postgres-feature"))]
                {
                    self.events
                        .as_array()
                        .ok_or_else(|| {
                            IndexerError::PersistentStorageDataCorruption(
                                "Failed to parse events as array".to_string(),
                            )
                        })?
                        .iter()
                        .map(|event| match event {
                            serde_json::Value::Null => Err(IndexerError::PersistentStorageDataCorruption(
                                "events should not contain null elements".to_string(),
                            )),
                            serde_json::Value::String(event) => {
                                let event: Event = bcs::from_bytes(event.as_bytes()).map_err(|e| {
                                    IndexerError::PersistentStorageDataCorruption(format!(
                                        "Can't convert event bytes into Event. tx_digest={:?} Error: {e}",
                                        tx_digest
                                    ))
                                })?;
                                Ok(event)
                            }
                            _ => Err(IndexerError::PersistentStorageDataCorruption(
                                "events should contain only string elements".to_string(),
                            )),
                        })
                        .collect::<Result<Vec<Event>, IndexerError>>()?
                }
            };
            let timestamp = self.timestamp_ms as u64;
            let tx_events = TransactionEvents { data: events };

            tx_events_to_iota_tx_events(tx_events, package_resolver, tx_digest, timestamp).await?
        } else {
            None
        };

        let object_changes = if options.show_object_changes {
            let object_changes = {
                #[cfg(feature = "postgres-feature")]
                {
                    self.object_changes.into_iter().map(|object_change| {
                        match object_change {
                            Some(object_change) => {
                                let object_change: IndexedObjectChange = bcs::from_bytes(&object_change)
                                    .map_err(|e| IndexerError::PersistentStorageDataCorruption(
                                        format!("Can't convert object_change bytes into IndexedObjectChange. tx_digest={:?} Error: {e}", tx_digest)
                                    ))?;
                                Ok(ObjectChange::from(object_change))
                            }
                            None => Err(IndexerError::PersistentStorageDataCorruption(format!("object_change should not be null, tx_digest={:?}", tx_digest))),
                        }
                    }).collect::<Result<Vec<ObjectChange>, IndexerError>>()?
                }
                #[cfg(feature = "mysql-feature")]
                #[cfg(not(feature = "postgres-feature"))]
                {
                    self.object_changes
                        .as_array()
                        .ok_or_else(|| {
                            IndexerError::PersistentStorageDataCorruption(
                                "Failed to parse object_changes as array".to_string(),
                            )
                        })?
                        .iter()
                        .map(|object_change| match object_change {
                            serde_json::Value::Null => Err(IndexerError::PersistentStorageDataCorruption(
                                "object_changes should not contain null elements".to_string(),
                            )),
                            serde_json::Value::String(object_change) => {
                                let object_change: IndexedObjectChange = bcs::from_bytes(object_change.as_bytes())
                                    .map_err(|e| IndexerError::PersistentStorageDataCorruption(
                                        format!("Can't convert object_change bytes into IndexedObjectChange. tx_digest={:?} Error: {e}", tx_digest)
                                    ))?;
                                Ok(ObjectChange::from(object_change))
                            }
                            _ => Err(IndexerError::PersistentStorageDataCorruption(
                                "object_changes should contain only string elements".to_string(),
                            )),
                        })
                        .collect::<Result<Vec<ObjectChange>, IndexerError>>()?
                }
            };
            Some(object_changes)
        } else {
            None
        };

        let balance_changes = if options.show_balance_changes {
            let balance_changes = {
                #[cfg(feature = "postgres-feature")]
                {
                    self.balance_changes.into_iter().map(|balance_change| {
                        match balance_change {
                            Some(balance_change) => {
                                let balance_change: BalanceChange = bcs::from_bytes(&balance_change)
                                    .map_err(|e| IndexerError::PersistentStorageDataCorruption(
                                        format!("Can't convert balance_change bytes into BalanceChange. tx_digest={:?} Error: {e}", tx_digest)
                                    ))?;
                                Ok(balance_change)
                            }
                            None => Err(IndexerError::PersistentStorageDataCorruption(format!("object_change should not be null, tx_digest={:?}", tx_digest))),
                        }
                    }).collect::<Result<Vec<BalanceChange>, IndexerError>>()?
                }
                #[cfg(feature = "mysql-feature")]
                #[cfg(not(feature = "postgres-feature"))]
                {
                    self.balance_changes
                        .as_array()
                        .ok_or_else(|| {
                            IndexerError::PersistentStorageDataCorruption(
                                "Failed to parse balance_changes as array".to_string(),
                            )
                        })?
                        .iter()
                        .map(|balance_change| match balance_change {
                            serde_json::Value::Null => Err(IndexerError::PersistentStorageDataCorruption(
                                "balance_changes should not contain null elements".to_string(),
                            )),
                            serde_json::Value::String(balance_change) => {
                                let balance_change: BalanceChange = bcs::from_bytes(balance_change.as_bytes())
                                    .map_err(|e| IndexerError::PersistentStorageDataCorruption(
                                        format!("Can't convert balance_change bytes into BalanceChange. tx_digest={:?} Error: {e}", tx_digest)
                                    ))?;
                                Ok(balance_change)
                            }
                            _ => Err(IndexerError::PersistentStorageDataCorruption(
                                "balance_changes should contain only string elements".to_string(),
                            )),
                        })
                        .collect::<Result<Vec<BalanceChange>, IndexerError>>()?
                }
            };
            Some(balance_changes)
        } else {
            None
        };

        let raw_effects = options
            .show_raw_effects
            .then_some(self.raw_effects)
            .unwrap_or_default();

        Ok(IotaTransactionBlockResponse {
            digest: tx_digest,
            transaction,
            raw_transaction,
            effects,
            events,
            object_changes,
            balance_changes,
            timestamp_ms: Some(self.timestamp_ms as u64),
            checkpoint: Some(self.checkpoint_sequence_number as u64),
            confirmed_local_execution: None,
            errors: vec![],
            raw_effects,
        })
    }

    fn try_into_sender_signed_data(&self) -> IndexerResult<SenderSignedData> {
        let sender_signed_data: SenderSignedData =
            bcs::from_bytes(&self.raw_transaction).map_err(|e| {
                IndexerError::PersistentStorageDataCorruption(format!(
                    "Can't convert raw_transaction of {} into SenderSignedData. Error: {e}",
                    self.tx_sequence_number
                ))
            })?;
        Ok(sender_signed_data)
    }

    pub fn try_into_iota_transaction_effects(&self) -> IndexerResult<IotaTransactionBlockEffects> {
        let effects: TransactionEffects = bcs::from_bytes(&self.raw_effects).map_err(|e| {
            IndexerError::PersistentStorageDataCorruption(format!(
                "Can't convert raw_effects of {} into TransactionEffects. Error: {e}",
                self.tx_sequence_number
            ))
        })?;
        let effects = IotaTransactionBlockEffects::try_from(effects)?;
        Ok(effects)
    }

    /// Check if this is the genesis transaction relying on the global ordering.
    pub fn is_genesis(&self) -> bool {
        self.tx_sequence_number == 0
    }
}

#[cfg(feature = "postgres-feature")]
impl StoredTransaction {
    /// Store the raw transaction data as a large object in postgres
    /// and replace `self.raw_transaction` with a pointer to the large object.
    fn store_raw_transaction_as_large_object<T: R2D2Connection + Send + 'static>(
        mut self,
        pool: &ConnectionPool<T>,
    ) -> Result<Self, IndexerError> {
        tracing::debug!("Storing raw transaction as large object data");
        let raw_tx = std::mem::take(&mut self.raw_transaction);
        let oid = put_large_object_in_chunks(raw_tx, Self::LARGE_OBJECT_CHUNK_SIZE, pool)?;
        self.raw_transaction = oid.to_le_bytes().to_vec();
        Ok(self)
    }

    /// Store object changes as a large object in postgres
    /// and replace `self.object_changes` with a pointer to the large object.
    fn store_object_changes_as_large_object<T: R2D2Connection + Send + 'static>(
        mut self,
        pool: &ConnectionPool<T>,
    ) -> Result<Self, IndexerError> {
        tracing::debug!("Storing object changes as large object data");
        let object_changes = std::mem::take(&mut self.object_changes);
        let data = bcs::to_bytes(&object_changes)
            .map_err(IndexerError::from)
            .context("failed to encode object changes as large object")?;
        let oid = put_large_object_in_chunks(data, Self::LARGE_OBJECT_CHUNK_SIZE, pool)?;
        self.object_changes.push(Some(oid.to_le_bytes().to_vec()));

        Ok(self)
    }

    /// Store the raw effects data as a large object in postgres
    /// and replace `self.raw_effects` with a pointer to the large object.
    fn store_raw_effects_as_large_object<T: R2D2Connection + Send + 'static>(
        mut self,
        pool: &ConnectionPool<T>,
    ) -> Result<Self, IndexerError> {
        tracing::debug!("Storing raw effects as large object data");
        let raw_tx = std::mem::take(&mut self.raw_effects);
        let oid = put_large_object_in_chunks(raw_tx, Self::LARGE_OBJECT_CHUNK_SIZE, pool)?;
        self.raw_effects = oid.to_le_bytes().to_vec();
        Ok(self)
    }

    /// This method checks if this is the genesis transaction,
    /// and if true it stores the raw data as a large object
    /// in postgres, replacing `self.raw_transaction`,`self.raw_effects`,
    /// and `self.object_changes` with the respective pointers.
    pub fn store_inner_genesis_data_as_large_object<T: R2D2Connection + Send + 'static>(
        self,
        pool: &ConnectionPool<T>,
    ) -> Result<Self, IndexerError> {
        if !self.is_genesis() {
            return Ok(self);
        }
        self.store_raw_transaction_as_large_object(pool)?
            .store_raw_effects_as_large_object(pool)?
            .store_object_changes_as_large_object(pool)
    }

    /// This method replaces `self.raw_transaction` large object id with the
    /// actual large object data
    fn set_large_object_as_raw_transaction<T: R2D2Connection + Send + 'static>(
        mut self,
        pool: &ConnectionPool<T>,
    ) -> Result<Self, IndexerError> {
        tracing::debug!("Setting large object data as raw transaction");
        let raw_oid = std::mem::take(&mut self.raw_transaction);
        let raw_oid: [u8; 4] = raw_oid
            .try_into()
            .map_err(|_| IndexerError::Generic("invalid large object identifier".to_owned()))?;
        let oid = u32::from_le_bytes(raw_oid);

        self.raw_transaction =
            get_large_object_in_chunks(oid, Self::LARGE_OBJECT_CHUNK_SIZE, pool)?;

        Ok(self)
    }

    /// This method replaces `self.raw_effects` large object id with the actual
    /// large object data
    fn set_large_object_as_raw_effects<T: R2D2Connection + Send + 'static>(
        mut self,
        pool: &ConnectionPool<T>,
    ) -> Result<Self, IndexerError> {
        tracing::debug!("Setting large object data as raw effects");
        let raw_oid = std::mem::take(&mut self.raw_effects);
        let raw_oid: [u8; 4] = raw_oid
            .try_into()
            .map_err(|_| IndexerError::Generic("invalid large object identifier".to_owned()))?;
        let oid = u32::from_le_bytes(raw_oid);

        self.raw_effects = get_large_object_in_chunks(oid, Self::LARGE_OBJECT_CHUNK_SIZE, pool)?;

        Ok(self)
    }

    /// This method replaces `self.object_changes` large object id with the
    /// actual large object data
    fn set_large_object_as_object_changes<T: R2D2Connection + Send + 'static>(
        mut self,
        pool: &ConnectionPool<T>,
    ) -> Result<Self, IndexerError> {
        tracing::debug!("Setting large object data as object changes");
        let raw_oid = std::mem::take(&mut self.object_changes)
            .pop()
            .flatten()
            .ok_or_else(|| IndexerError::Generic("invalid large object identifier".to_owned()))?;

        // The first values is the raw oid
        let oid =
            u32::from_le_bytes(raw_oid.try_into().map_err(|_| {
                IndexerError::Generic("invalid large object identifier".to_owned())
            })?);
        let stored_data = get_large_object_in_chunks(oid, Self::LARGE_OBJECT_CHUNK_SIZE, pool)?;
        self.object_changes = bcs::from_bytes(&stored_data)
            .map_err(IndexerError::from)
            .context("failed to decode object changes from large object")?;

        Ok(self)
    }

    /// The genesis transactions uses a pointer to the large object
    /// as inner data.
    ///
    /// This replaces the pointer with the actual large object data.
    pub fn set_genesis_large_object_as_inner_data<T: R2D2Connection + Send + 'static>(
        self,
        pool: &ConnectionPool<T>,
    ) -> Result<Self, IndexerError> {
        if !self.is_genesis() {
            return Ok(self);
        }
        self.set_large_object_as_raw_transaction(pool)?
            .set_large_object_as_raw_effects(pool)?
            .set_large_object_as_object_changes(pool)
    }
}

pub fn stored_events_to_events(
    stored_events: StoredTransactionEvents,
) -> Result<Vec<Event>, IndexerError> {
    #[cfg(feature = "postgres-feature")]
    {
        stored_events
            .into_iter()
            .map(|event| match event {
                Some(event) => {
                    let event: Event = bcs::from_bytes(&event).map_err(|e| {
                        IndexerError::PersistentStorageDataCorruption(format!(
                            "Can't convert event bytes into Event. Error: {e}",
                        ))
                    })?;
                    Ok(event)
                }
                None => Err(IndexerError::PersistentStorageDataCorruption(
                    "Event should not be null".to_string(),
                )),
            })
            .collect::<Result<Vec<Event>, IndexerError>>()
    }
    #[cfg(feature = "mysql-feature")]
    #[cfg(not(feature = "postgres-feature"))]
    {
        stored_events
            .as_array()
            .ok_or_else(|| {
                IndexerError::PersistentStorageDataCorruption(
                    "Failed to parse events as array".to_string(),
                )
            })?
            .iter()
            .map(|event| match event {
                serde_json::Value::Null => Err(IndexerError::PersistentStorageDataCorruption(
                    "events should not contain null elements".to_string(),
                )),
                serde_json::Value::String(event) => {
                    let event: Event = bcs::from_bytes(event.as_bytes()).map_err(|e| {
                        IndexerError::PersistentStorageDataCorruption(format!(
                            "Can't convert event bytes into Event. Error: {e}",
                        ))
                    })?;
                    Ok(event)
                }
                _ => Err(IndexerError::PersistentStorageDataCorruption(
                    "events should contain only string elements".to_string(),
                )),
            })
            .collect::<Result<Vec<Event>, IndexerError>>()
    }
}

pub async fn tx_events_to_iota_tx_events(
    tx_events: TransactionEvents,
    package_resolver: Arc<Resolver<impl PackageStore>>,
    tx_digest: TransactionDigest,
    timestamp: u64,
) -> Result<Option<IotaTransactionBlockEvents>, IndexerError> {
    let mut iota_event_futures = vec![];
    let tx_events_data_len = tx_events.data.len();
    for tx_event in tx_events.data.clone() {
        let package_resolver_clone = package_resolver.clone();
        iota_event_futures.push(tokio::task::spawn(async move {
            let resolver = package_resolver_clone;
            resolver
                .type_layout(TypeTag::Struct(Box::new(tx_event.type_.clone())))
                .await
        }));
    }
    let event_move_type_layouts = futures::future::join_all(iota_event_futures)
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| {
            IndexerError::ResolveMoveStruct(format!(
                "Failed to convert to iota event with Error: {e}",
            ))
        })?;
    let event_move_datatype_layouts = event_move_type_layouts
        .into_iter()
        .filter_map(|move_type_layout| match move_type_layout {
            MoveTypeLayout::Struct(s) => Some(MoveDatatypeLayout::Struct(s)),
            MoveTypeLayout::Enum(e) => Some(MoveDatatypeLayout::Enum(e)),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(tx_events_data_len == event_move_datatype_layouts.len());
    let iota_events = tx_events
        .data
        .into_iter()
        .enumerate()
        .zip(event_move_datatype_layouts)
        .map(|((seq, tx_event), move_datatype_layout)| {
            IotaEvent::try_from(
                tx_event,
                tx_digest,
                seq as u64,
                Some(timestamp),
                move_datatype_layout,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let iota_tx_events = IotaTransactionBlockEvents { data: iota_events };
    Ok(Some(iota_tx_events))
}
