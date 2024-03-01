// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use sui_types::{
    base_types::ObjectRef, committee::EpochId, digests::TransactionDigest, error::SuiResult,
};

pub type SuiLockResult = SuiResult<ObjectLockStatus>;
pub type LockDetails = LockDetailsV1;

#[derive(Debug, PartialEq, Eq)]
pub enum ObjectLockStatus {
    Initialized,
    LockedToTx { locked_by_tx: LockDetails },
    LockedAtDifferentVersion { locked_ref: ObjectRef },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LockDetailsV1 {
    pub epoch: EpochId,
    pub tx_digest: TransactionDigest,
}
