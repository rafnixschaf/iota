// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloy::{
    primitives::{Address as EthAddress, B256},
    sol_types::SolEventInterface,
};
use iota_types::base_types::IotaAddress;
use serde::{Deserialize, Serialize};

use crate::{
    error::{BridgeError, BridgeResult},
    types::{BridgeAction, BridgeChainId, EthLog, EthToIotaBridgeAction, TokenId},
};

// TODO: write a macro to handle variants

// TODO: Add other events
#[derive(Debug, Serialize, Deserialize)]
pub enum EthBridgeEvent {
    EthIotaBridgeEvents(EthIotaBridge::EthIotaBridgeEvents),
}

alloy::sol!(
    #[derive(Debug, Serialize, Deserialize)]
    #[sol(rpc)]
    EthIotaBridge,
    "abi/iota_bridge.json"
);

impl EthBridgeEvent {
    pub fn try_from_eth_log(log: &EthLog) -> Option<EthBridgeEvent> {
        if let Ok(decoded) = EthIotaBridge::EthIotaBridgeEvents::decode_raw_log(
            log.log.topics(),
            log.log.data().data.as_ref(),
            true,
        ) {
            return Some(EthBridgeEvent::EthIotaBridgeEvents(decoded));
        }

        // TODO: try other variants
        None
    }
}

impl EthBridgeEvent {
    pub fn try_into_bridge_action(
        self,
        eth_tx_hash: B256,
        eth_event_index: u16,
    ) -> Option<BridgeAction> {
        match self {
            EthBridgeEvent::EthIotaBridgeEvents(event) => {
                match event {
                    EthIotaBridge::EthIotaBridgeEvents::TokensBridgedToIota(event) => {
                        let bridge_event = match EthToIotaTokenBridgeV1::try_from(&event) {
                            Ok(bridge_event) => bridge_event,
                            // This only happens when solidity code does not align with rust code.
                            // When this happens in production, there is a risk of stuck bridge
                            // transfers. We log error here.
                            // TODO: add metrics and alert
                            Err(e) => {
                                tracing::error!(
                                    ?eth_tx_hash,
                                    eth_event_index,
                                    "Failed to convert TokensBridgedToIota log to EthToIotaTokenBridgeV1. This indicates incorrect parameters or a bug in the code: {:?}. Err: {:?}",
                                    event,
                                    e
                                );
                                return None;
                            }
                        };

                        Some(BridgeAction::EthToIotaBridgeAction(EthToIotaBridgeAction {
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

// Sanity checked version of TokensBridgedToIotaFilter
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct EthToIotaTokenBridgeV1 {
    pub nonce: u64,
    pub iota_chain_id: BridgeChainId,
    pub eth_chain_id: BridgeChainId,
    pub iota_address: IotaAddress,
    pub eth_address: EthAddress,
    pub token_id: TokenId,
    pub amount: u64,
}

impl TryFrom<&EthIotaBridge::TokensBridgedToIota> for EthToIotaTokenBridgeV1 {
    type Error = BridgeError;
    fn try_from(event: &EthIotaBridge::TokensBridgedToIota) -> BridgeResult<Self> {
        Ok(Self {
            nonce: event.nonce,
            iota_chain_id: BridgeChainId::try_from(event.destinationChainId)?,
            eth_chain_id: BridgeChainId::try_from(event.sourceChainId)?,
            iota_address: IotaAddress::from_bytes(event.targetAddress.as_ref())?,
            eth_address: event.sourceAddress,
            token_id: TokenId::try_from(event.tokenCode)?,
            amount: event.iotaAdjustedAmount,
        })
    }
}
