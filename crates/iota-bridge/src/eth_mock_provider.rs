// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A mock implementation of Ethereum JSON-RPC client, based on `MockProvider`
//! from `ethers-rs`.

use std::{
    borrow::{Borrow, Cow},
    collections::HashMap,
    fmt::Debug,
    pin::Pin,
    sync::{Arc, Mutex},
};

use alloy::{
    providers::{Provider, RootProvider},
    rpc::{
        client::RpcClient,
        json_rpc::{RequestPacket, Response, ResponsePacket, ResponsePayload, RpcParam, RpcReturn},
    },
    transports::{RpcError, TransportErrorKind, TransportResult},
};
use async_trait::async_trait;
use futures::Future;
use serde::Serialize;
use serde_json::{
    value::{to_raw_value, RawValue},
    Value,
};

/// Helper type that can be used to pass through the `params` value.
/// This is necessary because the wrapper provider is supposed to skip the
/// `params` if it's of size 0, see `crate::transports::common::Request`
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum MockParams {
    Value(String),
    Zst,
}

#[derive(Clone, Debug, Default)]
pub struct EthMockTransport {
    responses: Arc<Mutex<HashMap<(String, MockParams), Value>>>,
}

impl EthMockTransport {
    async fn request(
        responses: &Mutex<HashMap<(String, MockParams), Value>>,
        method: &str,
        params: Option<&RawValue>,
    ) -> Result<Value, RpcError<TransportErrorKind>> {
        let params = if params.is_none() || params.unwrap().get().is_empty() {
            MockParams::Zst
        } else {
            MockParams::Value(params.unwrap().get().to_string())
        };
        Ok(responses
            .lock()
            .unwrap()
            .get(&(method.to_string(), params))
            .ok_or(RpcError::NullResp)?
            .clone())
    }

    pub fn add_response<P: Serialize + Send + Sync, S: Serialize + Send + Sync, K: Borrow<S>>(
        &self,
        method: &str,
        params: P,
        data: K,
    ) -> anyhow::Result<()> {
        let params = if std::mem::size_of::<P>() == 0 {
            MockParams::Zst
        } else {
            MockParams::Value(serde_json::to_value(params)?.to_string())
        };
        let value = serde_json::to_value(data.borrow())?;
        self.responses
            .lock()
            .unwrap()
            .insert((method.to_owned(), params), value);
        Ok(())
    }

    async fn inner_call(
        responses: Arc<Mutex<HashMap<(String, MockParams), Value>>>,
        req: RequestPacket,
    ) -> Result<ResponsePacket, RpcError<TransportErrorKind>> {
        Ok(match req {
            RequestPacket::Single(req) => ResponsePacket::Single(Response {
                id: req.id().clone(),
                payload: Self::request(&responses, req.method(), req.params())
                    .await
                    .and_then(|v| to_raw_value(&v).map_err(RpcError::SerError))
                    .map(ResponsePayload::Success)?,
            }),
            RequestPacket::Batch(reqs) => {
                let mut resps = Vec::new();
                for req in reqs {
                    resps.push(Response {
                        id: req.id().clone(),
                        payload: Self::request(&responses, req.method(), req.params())
                            .await
                            .and_then(|v| to_raw_value(&v).map_err(RpcError::SerError))
                            .map(ResponsePayload::Success)?,
                    });
                }
                ResponsePacket::Batch(resps)
            }
        })
    }
}

impl tower::Service<RequestPacket> for EthMockTransport {
    type Response = ResponsePacket;
    type Error = RpcError<TransportErrorKind>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: RequestPacket) -> Self::Future {
        Box::pin(Self::inner_call(self.responses.clone(), req))
    }
}

/// Mock transport used in test environments.
#[derive(Clone, Debug)]
pub struct EthMockProvider {
    inner: RootProvider<EthMockTransport>,
}

impl Default for EthMockProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Provider<EthMockTransport> for EthMockProvider {
    fn root(&self) -> &RootProvider<EthMockTransport> {
        &self.inner
    }

    /// If `method` and `params` match previously set response by
    /// `add_response`, return the response. Otherwise return
    /// MockError::EmptyResponses.
    async fn raw_request<P, R>(&self, method: Cow<'static, str>, params: P) -> TransportResult<R>
    where
        P: RpcParam,
        R: RpcReturn,
        Self: Sized,
    {
        self.inner.raw_request(method, params).await
    }
}

impl EthMockProvider {
    pub fn new() -> Self {
        Self {
            inner: RootProvider::new(RpcClient::new(EthMockTransport::default(), true)),
        }
    }

    pub fn add_response<P: Serialize + Send + Sync, S: Serialize + Send + Sync, K: Borrow<S>>(
        &self,
        method: &str,
        params: P,
        data: K,
    ) -> anyhow::Result<()> {
        self.inner
            .client()
            .transport()
            .add_response(method, params, data)
    }
}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_basic_responses_match() {
        let mock = EthMockProvider::new();

        mock.add_response("eth_blockNumber", (), 12u64).unwrap();
        let block: u64 = mock
            .raw_request("eth_blockNumber".into(), ())
            .await
            .unwrap();

        assert_eq!(block, 12);
        let block: u64 = mock
            .raw_request("eth_blockNumber".into(), ())
            .await
            .unwrap();
        assert_eq!(block, 12);

        mock.add_response("eth_blockNumber", (), 13u64).unwrap();
        let block: u64 = mock
            .raw_request("eth_blockNumber".into(), ())
            .await
            .unwrap();
        assert_eq!(block, 13);

        mock.add_response("eth_foo", (), 0u64).unwrap();
        let block: u64 = mock
            .raw_request("eth_blockNumber".into(), ())
            .await
            .unwrap();
        assert_eq!(block, 13);

        let err = mock
            .raw_request::<_, ()>("eth_blockNumber".into(), "bar")
            .await
            .unwrap_err();
        match err {
            RpcError::NullResp => {}
            _ => panic!("expected empty responses"),
        };

        mock.add_response("eth_blockNumber", "bar", 14u64).unwrap();
        let block: u64 = mock
            .raw_request("eth_blockNumber".into(), "bar")
            .await
            .unwrap();
        assert_eq!(block, 14);
    }

    #[tokio::test]
    async fn test_with_provider() {
        let mock = EthMockProvider::new();

        mock.add_response("eth_blockNumber", (), 12u64).unwrap();
        let block = mock.get_block_number().await.unwrap();
        assert_eq!(block, 12);
    }
}
