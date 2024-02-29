// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use async_trait::async_trait;
use fastcrypto::encoding::Base64;
use jsonrpsee::core::RpcResult;
use jsonrpsee::RpcModule;

use sui_json_rpc_api::{JsonRpcMetrics, WriteApiOpenRpc, WriteApiServer};
use sui_json_rpc_types::{
    DevInspectArgs, DevInspectResults, DryRunTransactionBlockResponse, SuiTransactionBlockResponse,
    SuiTransactionBlockResponseOptions,
};
use sui_open_rpc::Module;
use sui_types::base_types::SuiAddress;
use sui_types::quorum_driver_types::ExecuteTransactionRequestType;
use sui_types::sui_serde::BigInt;
use tracing::instrument;

use crate::SuiRpcModule;

pub struct TransactionExecutionApi {
    _metrics: Arc<JsonRpcMetrics>,
}

impl TransactionExecutionApi {
    pub fn new(metrics: Arc<JsonRpcMetrics>) -> Self {
        Self { _metrics: metrics }
    }
}

#[async_trait]
impl WriteApiServer for TransactionExecutionApi {
    #[instrument(skip(self))]
    async fn execute_transaction_block(
        &self,
        _tx_bytes: Base64,
        _signatures: Vec<Base64>,
        _opts: Option<SuiTransactionBlockResponseOptions>,
        _request_type: Option<ExecuteTransactionRequestType>,
    ) -> RpcResult<SuiTransactionBlockResponse> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn dev_inspect_transaction_block(
        &self,
        _sender_address: SuiAddress,
        _tx_bytes: Base64,
        _gas_price: Option<BigInt<u64>>,
        _epoch: Option<BigInt<u64>>,
        _additional_args: Option<DevInspectArgs>,
    ) -> RpcResult<DevInspectResults> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn dry_run_transaction_block(
        &self,
        _tx_bytes: Base64,
    ) -> RpcResult<DryRunTransactionBlockResponse> {
        unimplemented!()
    }
}

impl SuiRpcModule for TransactionExecutionApi {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        WriteApiOpenRpc::module_doc()
    }
}
