// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

use alloy::{
    primitives::{Address as EthAddress, TxHash},
    providers::{Provider, ProviderBuilder, RootProvider},
    rpc::types::{Block, Filter},
    transports::{http::Http, Transport, TransportResult},
};
use reqwest::Client;
use tap::TapFallible;

#[cfg(test)]
use crate::eth_mock_provider::EthMockProvider;
use crate::{
    abi::EthBridgeEvent,
    error::{BridgeError, BridgeResult},
    types::{BridgeAction, EthLog},
};
pub struct EthClient<P> {
    provider: P,
    contract_addresses: HashSet<EthAddress>,
}

impl EthClient<RootProvider<Http<Client>>> {
    pub async fn new(
        provider_url: &str,
        contract_addresses: HashSet<EthAddress>,
    ) -> anyhow::Result<Self> {
        let provider = ProviderBuilder::new().on_http(provider_url.parse()?);
        let self_ = Self {
            provider,
            contract_addresses,
        };
        self_.describe().await?;
        Ok(self_)
    }
}

#[cfg(test)]
impl EthClient<EthMockProvider> {
    pub fn new_mocked(provider: EthMockProvider, contract_addresses: HashSet<EthAddress>) -> Self {
        let provider = ProviderBuilder::new().on_provider(provider);
        Self {
            provider,
            contract_addresses,
        }
    }
}

impl<P> EthClient<P> {
    // TODO assert chain identifier
    async fn describe<T: Transport + Clone>(&self) -> anyhow::Result<()>
    where
        P: Provider<T>,
    {
        let chain_id = self.provider.get_chain_id().await?;
        let block_number = self.provider.get_block_number().await?;
        tracing::info!(
            "EthClient is connected to chain {chain_id}, current block number: {block_number}"
        );
        Ok(())
    }

    /// Returns BridgeAction from an Eth Transaction with transaction hash
    /// and the event index. If event is declared in an unrecognized
    /// contract, return error.
    pub async fn get_finalized_bridge_action_maybe<T: Transport + Clone>(
        &self,
        tx_hash: TxHash,
        event_idx: u16,
    ) -> BridgeResult<BridgeAction>
    where
        P: Provider<T>,
    {
        let receipt = self
            .provider
            .get_transaction_receipt(tx_hash)
            .await
            .map_err(BridgeError::from)?
            .ok_or(BridgeError::TxNotFound)?;
        let receipt_block_num = receipt.block_number.ok_or(BridgeError::ProviderError(
            "Provider returns log without block_number".into(),
        ))?;
        let last_finalized_block_id = self.get_last_finalized_block_id().await?;
        if receipt_block_num > last_finalized_block_id {
            return Err(BridgeError::TxNotFinalized);
        }
        let log = receipt
            .inner
            .logs()
            .get(event_idx as usize)
            .ok_or(BridgeError::NoBridgeEventsInTxPosition)?;

        // Ignore events emitted from unrecognized contracts
        if !self.contract_addresses.contains(&log.inner.address) {
            return Err(BridgeError::BridgeEventInUnrecognizedEthContract);
        }

        let eth_log = EthLog {
            block_number: receipt_block_num,
            tx_hash,
            log_index_in_tx: event_idx,
            log: log.clone(),
        };
        let bridge_event = EthBridgeEvent::try_from_eth_log(&eth_log)
            .ok_or(BridgeError::NoBridgeEventsInTxPosition)?;
        bridge_event
            .try_into_bridge_action(tx_hash, event_idx)
            .ok_or(BridgeError::BridgeEventNotActionable)
    }

    pub async fn get_last_finalized_block_id<T: Transport + Clone>(&self) -> BridgeResult<u64>
    where
        P: Provider<T>,
    {
        let block: TransportResult<Option<Block<TxHash>>> = self
            .provider
            .raw_request("eth_getBlockByNumber".into(), ("finalized", false))
            .await;
        let block = block?.ok_or(BridgeError::TransientProviderError(
            "Provider fails to return last finalized block".into(),
        ))?;
        Ok(block
            .header
            .number
            .ok_or(BridgeError::TransientProviderError(
                "Provider returns block without number".into(),
            ))?)
    }

    // Note: query may fail if range is too big. Callsite is responsible
    // for chunking the query.
    pub async fn get_events_in_range<T: Transport + Clone>(
        &self,
        address: alloy::primitives::Address,
        start_block: u64,
        end_block: u64,
    ) -> BridgeResult<Vec<EthLog>>
    where
        P: Provider<T>,
    {
        let filter = Filter::new()
            .from_block(start_block)
            .to_block(end_block)
            .address(address);
        let logs = self
            .provider
            .get_logs(&filter)
            .await
            .map_err(BridgeError::from)
            .tap_err(|e| {
                tracing::error!(
                    "get_events_in_range failed. Filter: {:?}. Error {:?}",
                    filter,
                    e
                )
            })?;
        if logs.is_empty() {
            return Ok(vec![]);
        }
        // Safeguard check that all events are emitted from requested contract address
        assert!(logs.iter().all(|log| log.address() == address));

        let tasks = logs.into_iter().map(|log| self.get_log_tx_details(log));
        let results = futures::future::join_all(tasks)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .tap_err(|e| {
                tracing::error!(
                    "get_log_tx_details failed. Filter: {:?}. Error {:?}",
                    filter,
                    e
                )
            })?;
        Ok(results)
    }

    /// This function converts a `Log` to `EthLog`, to make sure the
    /// `block_num`, `tx_hash` and `log_index_in_tx` are available for
    /// downstream.
    // It's frustratingly ugly because of the nulliability of many fields in `Log`.
    async fn get_log_tx_details<T: Transport + Clone>(
        &self,
        log: alloy::rpc::types::Log,
    ) -> BridgeResult<EthLog>
    where
        P: Provider<T>,
    {
        let block_number = log.block_number.ok_or(BridgeError::ProviderError(
            "Provider returns log without block_number".into(),
        ))?;
        let tx_hash = log.transaction_hash.ok_or(BridgeError::ProviderError(
            "Provider returns log without transaction_hash".into(),
        ))?;
        // This is the log index in the block, rather than transaction.
        let log_index = log.log_index.ok_or(BridgeError::ProviderError(
            "Provider returns log without log_index".into(),
        ))?;

        // Now get the log's index in the transaction. There is `transaction_log_index`
        // field in `Log`, but I never saw it populated.

        let receipt = self
            .provider
            .get_transaction_receipt(tx_hash)
            .await
            .map_err(BridgeError::from)?
            .ok_or(BridgeError::ProviderError(format!(
                "Provide cannot find eth transaction for log: {:?})",
                log
            )))?;

        let receipt_block_num = receipt.block_number.ok_or(BridgeError::ProviderError(
            "Provider returns log without block_number".into(),
        ))?;
        if receipt_block_num != block_number {
            return Err(BridgeError::ProviderError(format!(
                "Provider returns receipt with different block number from log. Receipt: {:?}, Log: {:?}",
                receipt, log
            )));
        }

        // Find the log index in the transaction
        let mut log_index_in_tx = None;
        for (idx, receipt_log) in receipt.inner.logs().iter().enumerate() {
            // match log index (in the block)
            if receipt_log.log_index == Some(log_index) {
                // make sure the topics and data match
                if receipt_log.topics() != log.topics() || receipt_log.data() != log.data() {
                    return Err(BridgeError::ProviderError(format!(
                        "Provider returns receipt with different log from log. Receipt: {:?}, Log: {:?}",
                        receipt, log
                    )));
                }
                log_index_in_tx = Some(idx);
            }
        }
        let log_index_in_tx = log_index_in_tx.ok_or(BridgeError::ProviderError(format!(
            "Couldn't find matching log {:?} in transaction {}",
            log, tx_hash
        )))?;

        Ok(EthLog {
            block_number,
            tx_hash,
            log_index_in_tx: log_index_in_tx as u16,
            log,
        })
    }
}

#[cfg(test)]
mod tests {
    use alloy::{
        consensus::Eip658Value,
        primitives::{Address as EthAddress, Bloom},
        rpc::types::{Log, Receipt, ReceiptEnvelope, ReceiptWithBloom, TransactionReceipt},
    };
    use prometheus::Registry;

    use super::*;
    use crate::test_utils::{get_test_log_and_action, mock_last_finalized_block};

    #[tokio::test]
    async fn test_get_finalized_bridge_action_maybe() {
        telemetry_subscribers::init_for_testing();
        let registry = Registry::new();
        mysten_metrics::init_metrics(&registry);
        let mock_provider = EthMockProvider::new();
        mock_last_finalized_block(&mock_provider, 777);

        let client = EthClient::new_mocked(
            mock_provider.clone(),
            HashSet::from_iter(vec![EthAddress::ZERO]),
        );
        let result = client.get_last_finalized_block_id().await.unwrap();
        assert_eq!(result, 777);

        let eth_tx_hash = TxHash::new(rand::random());
        let log = Log {
            transaction_hash: Some(eth_tx_hash),
            block_number: Some(778),
            ..Default::default()
        };
        let (good_log, bridge_action) = get_test_log_and_action(EthAddress::ZERO, eth_tx_hash, 1);
        // Mocks `eth_getTransactionReceipt` to return `log` and `good_log` in order
        mock_provider
            .add_response::<[TxHash; 1], TransactionReceipt, TransactionReceipt>(
                "eth_getTransactionReceipt",
                [log.transaction_hash.unwrap()],
                TransactionReceipt {
                    block_number: log.block_number,
                    inner: ReceiptEnvelope::Legacy(ReceiptWithBloom::new(
                        Receipt {
                            status: Eip658Value::Eip658(true),
                            cumulative_gas_used: 0,
                            logs: vec![log, good_log],
                        },
                        Bloom::default(),
                    )),
                    transaction_hash: eth_tx_hash,
                    transaction_index: None,
                    block_hash: None,
                    gas_used: 0,
                    effective_gas_price: 0,
                    blob_gas_used: None,
                    blob_gas_price: None,
                    from: EthAddress::new(rand::random()),
                    to: None,
                    contract_address: None,
                    state_root: None,
                },
            )
            .unwrap();

        let error = client
            .get_finalized_bridge_action_maybe(eth_tx_hash, 0)
            .await
            .unwrap_err();
        assert_eq!(error, BridgeError::TxNotFinalized);

        // 778 is now finalized
        mock_last_finalized_block(&mock_provider, 778);

        let error = client
            .get_finalized_bridge_action_maybe(eth_tx_hash, 2)
            .await
            .unwrap_err();
        // Receipt only has 2 logs
        assert_eq!(error, BridgeError::NoBridgeEventsInTxPosition);

        let error = client
            .get_finalized_bridge_action_maybe(eth_tx_hash, 0)
            .await
            .unwrap_err();
        // Same, `log` is not a BridgeEvent
        assert_eq!(error, BridgeError::NoBridgeEventsInTxPosition);

        let action = client
            .get_finalized_bridge_action_maybe(eth_tx_hash, 1)
            .await
            .unwrap();
        assert_eq!(action, bridge_action);
    }

    #[tokio::test]
    async fn test_get_finalized_bridge_action_maybe_unrecognized_contract() {
        telemetry_subscribers::init_for_testing();
        let registry = Registry::new();
        mysten_metrics::init_metrics(&registry);
        let mock_provider = EthMockProvider::new();
        mock_last_finalized_block(&mock_provider, 777);

        let client = EthClient::new_mocked(
            mock_provider.clone(),
            HashSet::from_iter(vec![
                EthAddress::repeat_byte(5),
                EthAddress::repeat_byte(6),
                EthAddress::repeat_byte(7),
            ]),
        );
        let result = client.get_last_finalized_block_id().await.unwrap();
        assert_eq!(result, 777);

        let eth_tx_hash = TxHash::new(rand::random());
        // Event emitted from a different contract address
        let (log, _bridge_action) =
            get_test_log_and_action(EthAddress::repeat_byte(4), eth_tx_hash, 0);
        mock_provider
            .add_response::<[TxHash; 1], TransactionReceipt, TransactionReceipt>(
                "eth_getTransactionReceipt",
                [log.transaction_hash.unwrap()],
                TransactionReceipt {
                    block_number: log.block_number,
                    inner: ReceiptEnvelope::Legacy(ReceiptWithBloom::new(
                        Receipt {
                            status: Eip658Value::Eip658(true),
                            cumulative_gas_used: 0,
                            logs: vec![log],
                        },
                        Bloom::default(),
                    )),
                    transaction_hash: eth_tx_hash,
                    transaction_index: None,
                    block_hash: None,
                    gas_used: 0,
                    effective_gas_price: 0,
                    blob_gas_used: None,
                    blob_gas_price: None,
                    from: EthAddress::new(rand::random()),
                    to: None,
                    contract_address: None,
                    state_root: None,
                },
            )
            .unwrap();

        let error = client
            .get_finalized_bridge_action_maybe(eth_tx_hash, 0)
            .await
            .unwrap_err();
        assert_eq!(error, BridgeError::BridgeEventInUnrecognizedEthContract);

        // Ok if emitted from the right contract
        let (log, bridge_action) =
            get_test_log_and_action(EthAddress::repeat_byte(6), eth_tx_hash, 0);
        mock_provider
            .add_response::<[TxHash; 1], TransactionReceipt, TransactionReceipt>(
                "eth_getTransactionReceipt",
                [log.transaction_hash.unwrap()],
                TransactionReceipt {
                    block_number: log.block_number,
                    inner: ReceiptEnvelope::Legacy(ReceiptWithBloom::new(
                        Receipt {
                            status: Eip658Value::Eip658(true),
                            cumulative_gas_used: 0,
                            logs: vec![log],
                        },
                        Bloom::default(),
                    )),
                    transaction_hash: eth_tx_hash,
                    transaction_index: None,
                    block_hash: None,
                    gas_used: 0,
                    effective_gas_price: 0,
                    blob_gas_used: None,
                    blob_gas_price: None,
                    from: EthAddress::new(rand::random()),
                    to: None,
                    contract_address: None,
                    state_root: None,
                },
            )
            .unwrap();
        let action = client
            .get_finalized_bridge_action_maybe(eth_tx_hash, 0)
            .await
            .unwrap();
        assert_eq!(action, bridge_action);
    }
}
