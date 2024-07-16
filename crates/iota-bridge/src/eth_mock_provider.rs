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
use serde_json::Value;

/// Helper type that can be used to pass through the `params` value.
/// This is necessary because the wrapper provider is supposed to skip the
/// `params` if it's of size 0, see `crate::transports::common::Request`
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum MockParams {
    Value(String),
    Zst,
}

#[derive(Clone, Debug, Default)]
pub struct EthMockTransport;

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
        let resp = match req {
            RequestPacket::Single(req) => ResponsePacket::Single(Response {
                id: req.id().clone(),
                payload: ResponsePayload::Success(req.into_serialized()),
            }),
            RequestPacket::Batch(reqs) => ResponsePacket::Batch(
                reqs.into_iter()
                    .map(|req| Response {
                        id: req.id().clone(),
                        payload: ResponsePayload::Success(req.into_serialized()),
                    })
                    .collect(),
            ),
        };

        // create a response in a future.
        let fut = async { Ok(resp) };

        // Return the response as an immediate future
        Box::pin(fut)
    }
}

/// Mock transport used in test environments.
#[derive(Clone, Debug)]
pub struct EthMockProvider {
    inner: RootProvider<EthMockTransport>,
    responses: Arc<Mutex<HashMap<(String, MockParams), Value>>>,
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
        let params = if std::mem::size_of::<P>() == 0 {
            MockParams::Zst
        } else {
            MockParams::Value(
                serde_json::to_value(params)
                    .map_err(|err| RpcError::SerError(err))?
                    .to_string(),
            )
        };
        let element = self
            .responses
            .lock()
            .unwrap()
            .get(&(method.to_string(), params))
            .ok_or(RpcError::NullResp)?
            .clone();
        let res: R = R::deserialize(&element).map_err(|err| RpcError::DeserError {
            err,
            text: element.to_string(),
        })?;

        Ok(res)
    }
}

impl EthMockProvider {
    pub fn new() -> Self {
        Self {
            inner: RootProvider::new(RpcClient::new(EthMockTransport::default(), true)),
            responses: Arc::new(Mutex::new(HashMap::new())),
        }
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
