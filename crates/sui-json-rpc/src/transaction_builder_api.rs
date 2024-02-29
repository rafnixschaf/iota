// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use async_trait::async_trait;
use fastcrypto::encoding::Base64;
use jsonrpsee::core::RpcResult;
use jsonrpsee::RpcModule;

use sui_json::SuiJsonValue;
use sui_json_rpc_api::{TransactionBuilderOpenRpc, TransactionBuilderServer};
use sui_json_rpc_types::RPCTransactionRequestParams;
use sui_json_rpc_types::{SuiTransactionBlockBuilderMode, SuiTypeTag, TransactionBlockBytes};
use sui_open_rpc::Module;
use sui_transaction_builder::DataReader;
use sui_types::base_types::{ObjectID, SuiAddress};
use sui_types::sui_serde::BigInt;

use crate::SuiRpcModule;

pub struct TransactionBuilderApi();

impl TransactionBuilderApi {
    pub fn new() -> Self {
        Self {}
    }

    pub fn new_with_data_reader(_data_reader: Arc<dyn DataReader + Sync + Send>) -> Self {
        Self {}
    }
}

impl Default for TransactionBuilderApi {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TransactionBuilderServer for TransactionBuilderApi {
    async fn transfer_object(
        &self,
        _signer: SuiAddress,
        _object_id: ObjectID,
        _gas: Option<ObjectID>,
        _gas_budget: BigInt<u64>,
        _recipient: SuiAddress,
    ) -> RpcResult<TransactionBlockBytes> {
        unimplemented!()
    }

    async fn transfer_sui(
        &self,
        _signer: SuiAddress,
        _sui_object_id: ObjectID,
        _gas_budget: BigInt<u64>,
        _recipient: SuiAddress,
        _amount: Option<BigInt<u64>>,
    ) -> RpcResult<TransactionBlockBytes> {
        unimplemented!()
    }

    async fn pay(
        &self,
        _signer: SuiAddress,
        _input_coins: Vec<ObjectID>,
        _recipients: Vec<SuiAddress>,
        _amounts: Vec<BigInt<u64>>,
        _gas: Option<ObjectID>,
        _gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        unimplemented!()
    }

    async fn pay_sui(
        &self,
        _signer: SuiAddress,
        _input_coins: Vec<ObjectID>,
        _recipients: Vec<SuiAddress>,
        _amounts: Vec<BigInt<u64>>,
        _gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        unimplemented!()
    }

    async fn pay_all_sui(
        &self,
        _signer: SuiAddress,
        _input_coins: Vec<ObjectID>,
        _recipient: SuiAddress,
        _gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        unimplemented!()
    }

    async fn publish(
        &self,
        _sender: SuiAddress,
        _compiled_modules: Vec<Base64>,
        _dependencies: Vec<ObjectID>,
        _gas: Option<ObjectID>,
        _gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        unimplemented!()
    }

    async fn split_coin(
        &self,
        _signer: SuiAddress,
        _coin_object_id: ObjectID,
        _split_amounts: Vec<BigInt<u64>>,
        _gas: Option<ObjectID>,
        _gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        unimplemented!()
    }

    async fn split_coin_equal(
        &self,
        _signer: SuiAddress,
        _coin_object_id: ObjectID,
        _split_count: BigInt<u64>,
        _gas: Option<ObjectID>,
        _gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        unimplemented!()
    }

    async fn merge_coin(
        &self,
        _signer: SuiAddress,
        _primary_coin: ObjectID,
        _coin_to_merge: ObjectID,
        _gas: Option<ObjectID>,
        _gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        unimplemented!()
    }

    async fn move_call(
        &self,
        _signer: SuiAddress,
        _package_object_id: ObjectID,
        _module: String,
        _function: String,
        _type_arguments: Vec<SuiTypeTag>,
        _rpc_arguments: Vec<SuiJsonValue>,
        _gas: Option<ObjectID>,
        _gas_budget: BigInt<u64>,
        _txn_builder_mode: Option<SuiTransactionBlockBuilderMode>,
    ) -> RpcResult<TransactionBlockBytes> {
        unimplemented!()
    }

    async fn batch_transaction(
        &self,
        _signer: SuiAddress,
        _params: Vec<RPCTransactionRequestParams>,
        _gas: Option<ObjectID>,
        _gas_budget: BigInt<u64>,
        _txn_builder_mode: Option<SuiTransactionBlockBuilderMode>,
    ) -> RpcResult<TransactionBlockBytes> {
        unimplemented!()
    }

    async fn request_add_stake(
        &self,
        _signer: SuiAddress,
        _coins: Vec<ObjectID>,
        _amount: Option<BigInt<u64>>,
        _validator: SuiAddress,
        _gas: Option<ObjectID>,
        _gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        unimplemented!()
    }

    async fn request_withdraw_stake(
        &self,
        _signer: SuiAddress,
        _staked_sui: ObjectID,
        _gas: Option<ObjectID>,
        _gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        unimplemented!()
    }
}

impl SuiRpcModule for TransactionBuilderApi {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        TransactionBuilderOpenRpc::module_doc()
    }
}
