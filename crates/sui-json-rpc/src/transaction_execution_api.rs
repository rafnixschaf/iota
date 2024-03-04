// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use fastcrypto::encoding::Base64;
use jsonrpsee::core::RpcResult;
use jsonrpsee::RpcModule;

use shared_crypto::intent::{AppId, Intent, IntentMessage, IntentScope, IntentVersion};
use sui_json_rpc_api::{JsonRpcMetrics, WriteApiOpenRpc, WriteApiServer};
use sui_json_rpc_types::{
    DevInspectArgs, DevInspectResults, DryRunTransactionBlockResponse, SuiTransactionBlockResponse,
    SuiTransactionBlockResponseOptions,
};
use sui_open_rpc::Module;
use sui_types::base_types::SuiAddress;
use sui_types::crypto::default_hash;
use sui_types::digests::TransactionDigest;
use sui_types::effects::TransactionEffectsAPI;
use sui_types::quorum_driver_types::ExecuteTransactionRequestType;
use sui_types::sui_serde::BigInt;
use sui_types::transaction::{
    InputObjectKind, TransactionData, TransactionDataAPI, TransactionKind,
};
use tracing::instrument;

use crate::error::{Error, SuiRpcInputError};
use crate::state::StateRead;
use crate::{
    get_balance_changes_from_effect, get_object_changes, with_tracing, ObjectProviderCache,
    SuiRpcModule,
};

pub struct TransactionExecutionApi {
    state: Arc<dyn StateRead>,
    _metrics: Arc<JsonRpcMetrics>,
}

impl TransactionExecutionApi {
    pub fn new(state: Arc<dyn StateRead>, metrics: Arc<JsonRpcMetrics>) -> Self {
        Self {
            state,
            _metrics: metrics,
        }
    }

    pub fn convert_bytes<T: serde::de::DeserializeOwned>(
        &self,
        tx_bytes: Base64,
    ) -> Result<T, SuiRpcInputError> {
        let data: T = bcs::from_bytes(&tx_bytes.to_vec()?)?;
        Ok(data)
    }

    async fn execute_transaction_block(
        &self,
        _tx_bytes: Base64,
        _signatures: Vec<Base64>,
        _opts: Option<SuiTransactionBlockResponseOptions>,
        _request_type: Option<ExecuteTransactionRequestType>,
    ) -> Result<SuiTransactionBlockResponse, Error> {
        unimplemented!()
    }

    pub fn prepare_dry_run_transaction_block(
        &self,
        tx_bytes: Base64,
    ) -> Result<(TransactionData, TransactionDigest, Vec<InputObjectKind>), SuiRpcInputError> {
        let tx_data: TransactionData = self.convert_bytes(tx_bytes)?;
        let input_objs = tx_data.input_objects()?;
        let intent_msg = IntentMessage::new(
            Intent {
                version: IntentVersion::V0,
                scope: IntentScope::TransactionData,
                app_id: AppId::Sui,
            },
            tx_data,
        );
        let txn_digest = TransactionDigest::new(default_hash(&intent_msg.value));
        Ok((intent_msg.value, txn_digest, input_objs))
    }

    async fn dry_run_transaction_block(
        &self,
        tx_bytes: Base64,
    ) -> Result<DryRunTransactionBlockResponse, Error> {
        let (txn_data, txn_digest, input_objs) =
            self.prepare_dry_run_transaction_block(tx_bytes)?;
        let sender = txn_data.sender();
        let (resp, written_objects, transaction_effects, mock_gas) = self
            .state
            .dry_exec_transaction(txn_data.clone(), txn_digest)
            .await?;
        let object_cache = ObjectProviderCache::new_with_cache(self.state.clone(), written_objects);
        let balance_changes = get_balance_changes_from_effect(
            &object_cache,
            &transaction_effects,
            input_objs,
            mock_gas,
        )
        .await?;
        let object_changes = get_object_changes(
            &object_cache,
            sender,
            transaction_effects.modified_at_versions(),
            transaction_effects.all_changed_objects(),
            transaction_effects.all_removed_objects(),
        )
        .await?;

        Ok(DryRunTransactionBlockResponse {
            effects: resp.effects,
            events: resp.events,
            object_changes,
            balance_changes,
            input: resp.input,
        })
    }
}

#[async_trait]
impl WriteApiServer for TransactionExecutionApi {
    #[instrument(skip(self))]
    async fn execute_transaction_block(
        &self,
        tx_bytes: Base64,
        signatures: Vec<Base64>,
        opts: Option<SuiTransactionBlockResponseOptions>,
        request_type: Option<ExecuteTransactionRequestType>,
    ) -> RpcResult<SuiTransactionBlockResponse> {
        with_tracing!(Duration::from_secs(10), async move {
            self.execute_transaction_block(tx_bytes, signatures, opts, request_type)
                .await
        })
    }

    #[instrument(skip(self))]
    async fn dev_inspect_transaction_block(
        &self,
        sender_address: SuiAddress,
        tx_bytes: Base64,
        gas_price: Option<BigInt<u64>>,
        _epoch: Option<BigInt<u64>>,
        additional_args: Option<DevInspectArgs>,
    ) -> RpcResult<DevInspectResults> {
        with_tracing!(async move {
            let DevInspectArgs {
                gas_sponsor,
                gas_budget,
                gas_objects,
                show_raw_txn_data_and_effects,
                skip_checks,
            } = additional_args.unwrap_or_default();
            let tx_kind: TransactionKind = self.convert_bytes(tx_bytes)?;
            self.state
                .dev_inspect_transaction_block(
                    sender_address,
                    tx_kind,
                    gas_price.map(|i| *i),
                    gas_budget.map(|i| *i),
                    gas_sponsor,
                    gas_objects,
                    show_raw_txn_data_and_effects,
                    skip_checks,
                )
                .await
                .map_err(Error::from)
        })
    }

    #[instrument(skip(self))]
    async fn dry_run_transaction_block(
        &self,
        tx_bytes: Base64,
    ) -> RpcResult<DryRunTransactionBlockResponse> {
        with_tracing!(async move { self.dry_run_transaction_block(tx_bytes).await })
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
