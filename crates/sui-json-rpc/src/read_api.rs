// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use jsonrpsee::RpcModule;
use tracing::instrument;

use sui_json_rpc_api::{JsonRpcMetrics, ReadApiOpenRpc, ReadApiServer};
use sui_json_rpc_types::SuiLoadedChildObjectsResponse;
use sui_json_rpc_types::{
    Checkpoint, CheckpointId, CheckpointPage, ProtocolConfigResponse, SuiEvent,
    SuiGetPastObjectRequest, SuiObjectDataOptions, SuiObjectResponse, SuiPastObjectResponse,
    SuiTransactionBlockResponse, SuiTransactionBlockResponseOptions,
};
use sui_open_rpc::Module;
use sui_types::base_types::{ObjectID, SequenceNumber, TransactionDigest};
use sui_types::sui_serde::BigInt;

use crate::SuiRpcModule;

// An implementation of the read portion of the JSON-RPC interface intended for use in
// Fullnodes.
#[derive(Clone)]
pub struct ReadApi {
    pub metrics: Arc<JsonRpcMetrics>,
}

impl ReadApi {
    pub fn new(metrics: Arc<JsonRpcMetrics>) -> Self {
        Self { metrics }
    }
}

#[async_trait]
impl ReadApiServer for ReadApi {
    #[instrument(skip(self))]
    async fn get_object(
        &self,
        _object_id: ObjectID,
        _options: Option<SuiObjectDataOptions>,
    ) -> RpcResult<SuiObjectResponse> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn multi_get_objects(
        &self,
        _object_ids: Vec<ObjectID>,
        _options: Option<SuiObjectDataOptions>,
    ) -> RpcResult<Vec<SuiObjectResponse>> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn try_get_past_object(
        &self,
        _object_id: ObjectID,
        _version: SequenceNumber,
        _options: Option<SuiObjectDataOptions>,
    ) -> RpcResult<SuiPastObjectResponse> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn try_multi_get_past_objects(
        &self,
        _past_objects: Vec<SuiGetPastObjectRequest>,
        _options: Option<SuiObjectDataOptions>,
    ) -> RpcResult<Vec<SuiPastObjectResponse>> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn get_total_transaction_blocks(&self) -> RpcResult<BigInt<u64>> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn get_transaction_block(
        &self,
        _digest: TransactionDigest,
        _opts: Option<SuiTransactionBlockResponseOptions>,
    ) -> RpcResult<SuiTransactionBlockResponse> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn multi_get_transaction_blocks(
        &self,
        _digests: Vec<TransactionDigest>,
        _opts: Option<SuiTransactionBlockResponseOptions>,
    ) -> RpcResult<Vec<SuiTransactionBlockResponse>> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn get_events(&self, _transaction_digest: TransactionDigest) -> RpcResult<Vec<SuiEvent>> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn get_latest_checkpoint_sequence_number(&self) -> RpcResult<BigInt<u64>> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn get_checkpoint(&self, _id: CheckpointId) -> RpcResult<Checkpoint> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn get_checkpoints(
        &self,
        // If `Some`, the query will start from the next item after the specified cursor
        _cursor: Option<BigInt<u64>>,
        _limit: Option<usize>,
        _descending_order: bool,
    ) -> RpcResult<CheckpointPage> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn get_checkpoints_deprecated_limit(
        &self,
        _cursor: Option<BigInt<u64>>,
        _limit: Option<BigInt<u64>>,
        _descending_order: bool,
    ) -> RpcResult<CheckpointPage> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn get_loaded_child_objects(
        &self,
        _digest: TransactionDigest,
    ) -> RpcResult<SuiLoadedChildObjectsResponse> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn get_protocol_config(
        &self,
        _version: Option<BigInt<u64>>,
    ) -> RpcResult<ProtocolConfigResponse> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn get_chain_identifier(&self) -> RpcResult<String> {
        unimplemented!()
    }
}

impl SuiRpcModule for ReadApi {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        ReadApiOpenRpc::module_doc()
    }
}

#[cfg(test)]
mod tests {
    use sui_types::messages_checkpoint::CheckpointSequenceNumber;

    fn calculate_checkpoint_numbers(
        // If `Some`, the query will start from the next item after the specified cursor
        cursor: Option<CheckpointSequenceNumber>,
        limit: u64,
        descending_order: bool,
        max_checkpoint: CheckpointSequenceNumber,
    ) -> Vec<CheckpointSequenceNumber> {
        let (start_index, end_index) = match cursor {
            Some(t) => {
                if descending_order {
                    let start = std::cmp::min(t.saturating_sub(1), max_checkpoint);
                    let end = start.saturating_sub(limit - 1);
                    (end, start)
                } else {
                    let start =
                        std::cmp::min(t.checked_add(1).unwrap_or(max_checkpoint), max_checkpoint);
                    let end = std::cmp::min(
                        start.checked_add(limit - 1).unwrap_or(max_checkpoint),
                        max_checkpoint,
                    );
                    (start, end)
                }
            }
            None => {
                if descending_order {
                    (max_checkpoint.saturating_sub(limit - 1), max_checkpoint)
                } else {
                    (0, std::cmp::min(limit - 1, max_checkpoint))
                }
            }
        };

        if descending_order {
            (start_index..=end_index).rev().collect()
        } else {
            (start_index..=end_index).collect()
        }
    }

    #[test]
    fn test_calculate_checkpoint_numbers() {
        let cursor = Some(10);
        let limit = 5;
        let descending_order = true;
        let max_checkpoint = 15;

        let checkpoint_numbers =
            calculate_checkpoint_numbers(cursor, limit, descending_order, max_checkpoint);

        assert_eq!(checkpoint_numbers, vec![9, 8, 7, 6, 5]);
    }

    #[test]
    fn test_calculate_checkpoint_numbers_descending_no_cursor() {
        let cursor = None;
        let limit = 5;
        let descending_order = true;
        let max_checkpoint = 15;

        let checkpoint_numbers =
            calculate_checkpoint_numbers(cursor, limit, descending_order, max_checkpoint);

        assert_eq!(checkpoint_numbers, vec![15, 14, 13, 12, 11]);
    }

    #[test]
    fn test_calculate_checkpoint_numbers_ascending_no_cursor() {
        let cursor = None;
        let limit = 5;
        let descending_order = false;
        let max_checkpoint = 15;

        let checkpoint_numbers =
            calculate_checkpoint_numbers(cursor, limit, descending_order, max_checkpoint);

        assert_eq!(checkpoint_numbers, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_calculate_checkpoint_numbers_ascending_with_cursor() {
        let cursor = Some(10);
        let limit = 5;
        let descending_order = false;
        let max_checkpoint = 15;

        let checkpoint_numbers =
            calculate_checkpoint_numbers(cursor, limit, descending_order, max_checkpoint);

        assert_eq!(checkpoint_numbers, vec![11, 12, 13, 14, 15]);
    }

    #[test]
    fn test_calculate_checkpoint_numbers_ascending_limit_exceeds_max() {
        let cursor = None;
        let limit = 20;
        let descending_order = false;
        let max_checkpoint = 15;

        let checkpoint_numbers =
            calculate_checkpoint_numbers(cursor, limit, descending_order, max_checkpoint);

        assert_eq!(checkpoint_numbers, (0..=15).collect::<Vec<_>>());
    }

    #[test]
    fn test_calculate_checkpoint_numbers_descending_limit_exceeds_max() {
        let cursor = None;
        let limit = 20;
        let descending_order = true;
        let max_checkpoint = 15;

        let checkpoint_numbers =
            calculate_checkpoint_numbers(cursor, limit, descending_order, max_checkpoint);

        assert_eq!(checkpoint_numbers, (0..=15).rev().collect::<Vec<_>>());
    }
}
