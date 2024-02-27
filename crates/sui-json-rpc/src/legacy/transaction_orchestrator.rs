// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use sui_types::quorum_driver_types::{
    ExecuteTransactionRequest, ExecuteTransactionResponse, QuorumDriverError,
};

/// Transaction Orchestrator is a Node component that utilizes Quorum Driver to
/// submit transactions to validators for finality, and proactively executes
/// finalized transactions locally, when possible.
pub struct TransactiondOrchestrator {}

impl TransactiondOrchestrator {
    pub async fn execute_transaction_block(
        &self,
        _: ExecuteTransactionRequest,
    ) -> Result<ExecuteTransactionResponse, QuorumDriverError> {
        unimplemented!()
    }
}
