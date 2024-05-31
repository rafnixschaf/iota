// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use ethers::{
    abi::RawLog,
    contract::{abigen, EthLogDecode},
    types::Address as EthAddress,
};
use serde::{Deserialize, Serialize};
use iota_types::base_types::IOTAAddress;

use crate::{
    error::{BridgeError, BridgeResult},
    types::{BridgeAction, BridgeChainId, EthLog, EthToIOTABridgeAction, TokenId},
};

// TODO: write a macro to handle variants

// TODO: Add other events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EthBridgeEvent {
    EthIOTABridgeEvents(EthIOTABridgeEvents),
}

abigen!(
    EthIOTABridge,
    "abi/iota_bridge.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

impl EthBridgeEvent {
    pub fn try_from_eth_log(log: &EthLog) -> Option<EthBridgeEvent> {
        let raw_log = RawLog {
            topics: log.log.topics.clone(),
            data: log.log.data.to_vec(),
        };

        if let Ok(decoded) = EthIOTABridgeEvents::decode_log(&raw_log) {
            return Some(EthBridgeEvent::EthIOTABridgeEvents(decoded));
        }

        // TODO: try other variants
        None
    }
}

impl EthBridgeEvent {
    pub fn try_into_bridge_action(
        self,
        eth_tx_hash: ethers::types::H256,
        eth_event_index: u16,
    ) -> Option<BridgeAction> {
        match self {
            EthBridgeEvent::EthIOTABridgeEvents(event) => {
                match event {
                    EthIOTABridgeEvents::TokensBridgedToIOTAFilter(event) => {
                        let bridge_event = match EthToIOTATokenBridgeV1::try_from(&event) {
                            Ok(bridge_event) => bridge_event,
                            // This only happens when solidity code does not align with rust code.
                            // When this happens in production, there is a risk of stuck bridge
                            // transfers. We log error here.
                            // TODO: add metrics and alert
                            Err(e) => {
                                tracing::error!(
                                    ?eth_tx_hash,
                                    eth_event_index,
                                    "Failed to convert TokensBridgedToIOTA log to EthToIOTATokenBridgeV1. This indicates incorrect parameters or a bug in the code: {:?}. Err: {:?}",
                                    event,
                                    e
                                );
                                return None;
                            }
                        };

                        Some(BridgeAction::EthToIOTABridgeAction(EthToIOTABridgeAction {
                            eth_tx_hash,
                            eth_event_index,
                            eth_bridge_event: bridge_event,
                        }))
                    }
                    _ => None,
                }
            }
        }
    }
}

// Sanity checked version of TokensBridgedToIOTAFilter
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct EthToIOTATokenBridgeV1 {
    pub nonce: u64,
    pub iota_chain_id: BridgeChainId,
    pub eth_chain_id: BridgeChainId,
    pub iota_address: IOTAAddress,
    pub eth_address: EthAddress,
    pub token_id: TokenId,
    pub amount: u64,
}

impl TryFrom<&TokensBridgedToIOTAFilter> for EthToIOTATokenBridgeV1 {
    type Error = BridgeError;
    fn try_from(event: &TokensBridgedToIOTAFilter) -> BridgeResult<Self> {
        Ok(Self {
            nonce: event.nonce,
            iota_chain_id: BridgeChainId::try_from(event.destination_chain_id)?,
            eth_chain_id: BridgeChainId::try_from(event.source_chain_id)?,
            iota_address: IOTAAddress::from_bytes(event.target_address.as_ref())?,
            eth_address: event.source_address,
            token_id: TokenId::try_from(event.token_code)?,
            amount: event.iota_adjusted_amount,
        })
    }
}
