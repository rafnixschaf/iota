// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use mysten_metrics::metered_channel::ReceiverStream;
use sui_json_rpc_types::{EventFilter, SuiEvent, SuiTransactionBlockEffects, TransactionFilter};

/// User can subscribe to transaction effects stream using `TransactionFilter` object.
pub struct SubscriptionHandler {}

impl SubscriptionHandler {
    pub fn subscribe_events(&self, _filter: EventFilter) -> ReceiverStream<SuiEvent> {
        unimplemented!()
    }

    pub fn subscribe_transactions(
        &self,
        _filter: TransactionFilter,
    ) -> ReceiverStream<SuiTransactionBlockEffects> {
        unimplemented!()
    }
}
