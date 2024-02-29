// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use jsonrpsee::{core::RpcResult, types::SubscriptionResult, RpcModule, SubscriptionSink};
use std::sync::Arc;
use sui_json_rpc_api::{IndexerApiOpenRpc, IndexerApiServer, JsonRpcMetrics, ReadApiServer};
use sui_json_rpc_types::{
    DynamicFieldPage, EventFilter, EventPage, ObjectsPage, Page, SuiObjectResponse,
    SuiObjectResponseQuery, SuiTransactionBlockResponseQuery, TransactionBlocksPage,
    TransactionFilter,
};
use sui_open_rpc::Module;
use sui_types::{
    base_types::{ObjectID, SuiAddress},
    digests::TransactionDigest,
    dynamic_field::DynamicFieldName,
    event::EventID,
};
use tokio::sync::Semaphore;
use tracing::{instrument, warn};

use crate::{name_service::NameServiceConfig, SuiRpcModule};

const DEFAULT_MAX_SUBSCRIPTIONS: usize = 100;

pub struct IndexerApi<R> {
    _read_api: R,
    _name_service_config: NameServiceConfig,
    pub metrics: Arc<JsonRpcMetrics>,
    _subscription_semaphore: Arc<Semaphore>,
}

impl<R: ReadApiServer> IndexerApi<R> {
    pub fn new(
        read_api: R,
        name_service_config: NameServiceConfig,
        metrics: Arc<JsonRpcMetrics>,
        max_subscriptions: Option<usize>,
    ) -> Self {
        let max_subscriptions = max_subscriptions.unwrap_or(DEFAULT_MAX_SUBSCRIPTIONS);
        Self {
            _read_api: read_api,
            _name_service_config: name_service_config,
            metrics,
            _subscription_semaphore: Arc::new(Semaphore::new(max_subscriptions)),
        }
    }
}

#[async_trait]
impl<R: ReadApiServer> IndexerApiServer for IndexerApi<R> {
    #[instrument(skip(self))]
    async fn get_owned_objects(
        &self,
        _address: SuiAddress,
        _query: Option<SuiObjectResponseQuery>,
        _cursor: Option<ObjectID>,
        _limit: Option<usize>,
    ) -> RpcResult<ObjectsPage> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn query_transaction_blocks(
        &self,
        _query: SuiTransactionBlockResponseQuery,
        // If `Some`, the query will start from the next item after the specified cursor
        _cursor: Option<TransactionDigest>,
        _limit: Option<usize>,
        _descending_order: Option<bool>,
    ) -> RpcResult<TransactionBlocksPage> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn query_events(
        &self,
        _query: EventFilter,
        // exclusive cursor if `Some`, otherwise start from the beginning
        _cursor: Option<EventID>,
        _limit: Option<usize>,
        _descending_order: Option<bool>,
    ) -> RpcResult<EventPage> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    fn subscribe_event(&self, sink: SubscriptionSink, filter: EventFilter) -> SubscriptionResult {
        unimplemented!()
    }

    fn subscribe_transaction(
        &self,
        _sink: SubscriptionSink,
        _filter: TransactionFilter,
    ) -> SubscriptionResult {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn get_dynamic_fields(
        &self,
        _parent_object_id: ObjectID,
        // If `Some`, the query will start from the next item after the specified cursor
        _cursor: Option<ObjectID>,
        _limit: Option<usize>,
    ) -> RpcResult<DynamicFieldPage> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn get_dynamic_field_object(
        &self,
        _parent_object_id: ObjectID,
        _name: DynamicFieldName,
    ) -> RpcResult<SuiObjectResponse> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn resolve_name_service_address(&self, _name: String) -> RpcResult<Option<SuiAddress>> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn resolve_name_service_names(
        &self,
        _address: SuiAddress,
        _cursor: Option<ObjectID>,
        _limit: Option<usize>,
    ) -> RpcResult<Page<String, ObjectID>> {
        unimplemented!()
    }
}

impl<R: ReadApiServer> SuiRpcModule for IndexerApi<R> {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        IndexerApiOpenRpc::module_doc()
    }
}
