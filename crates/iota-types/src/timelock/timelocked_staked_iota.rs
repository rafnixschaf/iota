// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use move_core_types::{ident_str, identifier::IdentStr, language_storage::StructTag};
use serde::{Deserialize, Serialize};

use crate::{
    base_types::ObjectID,
    committee::EpochId,
    error::IOTAError,
    governance::StakedIOTA,
    id::UID,
    object::{Data, Object},
    TIMELOCK_ADDRESS,
};

pub const TIMELOCKED_STAKED_IOTA_MODULE_NAME: &IdentStr = ident_str!("timelocked_staked_iota");
pub const TIMELOCKED_STAKED_IOTA_STRUCT_NAME: &IdentStr = ident_str!("TimelockedStakedIOTA");

/// Rust version of the Move
/// stardust::timelocked_staked_iota::TimelockedStakedIOTA type.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct TimelockedStakedIOTA {
    id: UID,
    /// A self-custodial object holding the staked IOTA tokens.
    staked_iota: StakedIOTA,
    /// This is the epoch time stamp of when the lock expires.
    expiration_timestamp_ms: u64,
}

impl TimelockedStakedIOTA {
    /// Get the TimeLock's `type`.
    pub fn type_() -> StructTag {
        StructTag {
            address: TIMELOCK_ADDRESS,
            module: TIMELOCKED_STAKED_IOTA_MODULE_NAME.to_owned(),
            name: TIMELOCKED_STAKED_IOTA_STRUCT_NAME.to_owned(),
            type_params: vec![],
        }
    }

    /// Is this other StructTag representing a TimelockedStakedIOTA?
    pub fn is_timelocked_staked_iota(s: &StructTag) -> bool {
        s.address == TIMELOCK_ADDRESS
            && s.module.as_ident_str() == TIMELOCKED_STAKED_IOTA_MODULE_NAME
            && s.name.as_ident_str() == TIMELOCKED_STAKED_IOTA_STRUCT_NAME
            && s.type_params.is_empty()
    }

    /// Get the TimelockedStakedIOTA's `id`.
    pub fn id(&self) -> ObjectID {
        self.id.id.bytes
    }

    /// Get the wrapped StakedIOTA's `pool_id`.
    pub fn pool_id(&self) -> ObjectID {
        self.staked_iota.pool_id()
    }

    /// Get the wrapped StakedIOTA's `activation_epoch`.
    pub fn activation_epoch(&self) -> EpochId {
        self.staked_iota.activation_epoch()
    }

    /// Get the wrapped StakedIOTA's `request_epoch`.
    pub fn request_epoch(&self) -> EpochId {
        // TODO: this might change when we implement warm up period.
        self.staked_iota.activation_epoch().saturating_sub(1)
    }

    /// Get the wrapped StakedIOTA's `principal`.
    pub fn principal(&self) -> u64 {
        self.staked_iota.principal()
    }

    /// Get the TimelockedStakedIOTA's `expiration_timestamp_ms`.
    pub fn expiration_timestamp_ms(&self) -> u64 {
        self.expiration_timestamp_ms
    }
}

impl TryFrom<&Object> for TimelockedStakedIOTA {
    type Error = IOTAError;
    fn try_from(object: &Object) -> Result<Self, Self::Error> {
        match &object.data {
            Data::Move(o) => {
                if o.type_().is_timelocked_staked_iota() {
                    return bcs::from_bytes(o.contents()).map_err(|err| IOTAError::TypeError {
                        error: format!(
                            "Unable to deserialize TimelockedStakedIOTA object: {:?}",
                            err
                        ),
                    });
                }
            }
            Data::Package(_) => {}
        }

        Err(IOTAError::TypeError {
            error: format!("Object type is not a TimelockedStakedIOTA: {:?}", object),
        })
    }
}
