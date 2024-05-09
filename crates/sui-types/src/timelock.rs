// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::{balance::Balance, base_types::ObjectID, id::UID, STARDUST_ADDRESS};
use move_core_types::{
    //annotated_value::{MoveFieldLayout, MoveStructLayout, MoveTypeLayout},
    ident_str,
    identifier::IdentStr,
    language_storage::{StructTag, TypeTag},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const TIMELOCK_MODULE_NAME: &IdentStr = ident_str!("timelock");
pub const TIMELOCK_STRUCT_NAME: &IdentStr = ident_str!("TimeLock");

// Rust version of the Move stardust::timelock::TimeLock type.
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, Eq, PartialEq)]
pub struct TimeLock<T> {
    pub id: UID,
    /// The locked object.
    pub locked: T,
    /// This is the epoch time stamp of when the lock expires.
    pub expire_timestamp_ms: u64,
}

impl<'de, T> TimeLock<T>
where
    T: Serialize + Deserialize<'de>,
{
    /// Constructor.
    pub fn new(id: UID, locked: T, expire_timestamp_ms: u64) -> Self {
        Self {
            id,
            locked,
            expire_timestamp_ms,
        }
    }

    /// The `TimeLock` type accessor.
    pub fn type_(type_param: TypeTag) -> StructTag {
        StructTag {
            address: STARDUST_ADDRESS,
            name: TIMELOCK_STRUCT_NAME.to_owned(),
            module: TIMELOCK_MODULE_NAME.to_owned(),
            type_params: vec![type_param],
        }
    }

    pub fn id(&self) -> &ObjectID {
        self.id.object_id()
    }

    pub fn locked(&self) -> &T {
        &self.locked
    }

    pub fn expire_timestamp_ms(&self) -> u64 {
        self.expire_timestamp_ms
    }

    /// Create a `TimeLock` from BCS bytes.
    pub fn from_bcs_bytes(content: &'de [u8]) -> Result<Self, bcs::Error> {
        bcs::from_bytes(content)
    }

    /// Serialize a `TimeLock` as a `Vec<u8>` of BCS.
    pub fn to_bcs_bytes(&self) -> Vec<u8> {
        bcs::to_bytes(&self).unwrap()
    }

    // TODO
    // pub fn layout(type_param: TypeTag) -> MoveStructLayout {
    //     MoveStructLayout {
    //         type_: Self::type_(type_param.clone()),
    //         fields: vec![
    //             MoveFieldLayout::new(
    //                 ident_str!("id").to_owned(),
    //                 MoveTypeLayout::Struct(UID::layout()),
    //             ),
    //             // MoveFieldLayout::new(
    //             //     ident_str!("locked").to_owned(),
    //             //     MoveTypeLayout::Struct(locked.),
    //             // ),
    //             MoveFieldLayout::new(
    //                 ident_str!("expire_timestamp_ms").to_owned(),
    //                 MoveTypeLayout::U64,
    //             ),
    //         ],
    //     }
    // }
}

/// Is this other StructTag representing a TimeLock?
pub fn is_timelock(other: &StructTag) -> bool {
    other.address == STARDUST_ADDRESS
        && other.module.as_ident_str() == TIMELOCK_MODULE_NAME
        && other.name.as_ident_str() == TIMELOCK_STRUCT_NAME
}

/// Is this other StructTag representing a TimeLock<Balance<T>>?
pub fn is_timelocked_balance(other: &StructTag) -> bool {
    if !is_timelock(other) {
        return false;
    }

    if other.type_params.len() != 1 {
        return false;
    }

    let param = &other.type_params[0];

    match param {
        TypeTag::Struct(tag) => Balance::is_balance(tag),
        _ => false,
    }
}
