// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use move_core_types::{ident_str, identifier::IdentStr, language_storage::StructTag};
use serde::{Deserialize, Serialize};

use crate::{
    IOTA_SYSTEM_ADDRESS,
    balance::Balance,
    base_types::ObjectID,
    collection_types::{Entry, VecMap},
    committee::EpochId,
    display::DisplayVersionUpdatedEvent,
    error::IotaError,
    gas_coin::NANOS_PER_IOTA,
    id::{ID, UID},
    object::{Data, Object},
};

/// Maximum number of active validators at any moment.
/// We do not allow the number of validators in any epoch to go above this.
pub const MAX_VALIDATOR_COUNT: u64 = 150;

/// Lower-bound on the amount of stake required to become a validator.
///
/// 2 million IOTA
pub const MIN_VALIDATOR_JOINING_STAKE_NANOS: u64 = 2_000_000 * NANOS_PER_IOTA;

/// Validators with stake amount below `validator_low_stake_threshold` are
/// considered to have low stake and will be escorted out of the validator set
/// after being below this threshold for more than
/// `validator_low_stake_grace_period` number of epochs.
///
/// 1.5 million IOTA
pub const VALIDATOR_LOW_STAKE_THRESHOLD_NANOS: u64 = 1_500_000 * NANOS_PER_IOTA;

/// Validators with stake below `validator_very_low_stake_threshold` will be
/// removed immediately at epoch change, no grace period.
///
/// 1 million IOTA
pub const VALIDATOR_VERY_LOW_STAKE_THRESHOLD_NANOS: u64 = 1_000_000 * NANOS_PER_IOTA;

/// A validator can have stake below `validator_low_stake_threshold`
/// for this many epochs before being kicked out.
pub const VALIDATOR_LOW_STAKE_GRACE_PERIOD: u64 = 7;

pub const STAKING_POOL_MODULE_NAME: &IdentStr = ident_str!("staking_pool");
pub const STAKED_IOTA_STRUCT_NAME: &IdentStr = ident_str!("StakedIota");

pub const ADD_STAKE_MUL_COIN_FUN_NAME: &IdentStr = ident_str!("request_add_stake_mul_coin");
pub const ADD_STAKE_FUN_NAME: &IdentStr = ident_str!("request_add_stake");
pub const WITHDRAW_STAKE_FUN_NAME: &IdentStr = ident_str!("request_withdraw_stake");

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct StakedIota {
    id: UID,
    pool_id: ID,
    stake_activation_epoch: u64,
    principal: Balance,
}

impl StakedIota {
    pub fn type_() -> StructTag {
        StructTag {
            address: IOTA_SYSTEM_ADDRESS,
            module: STAKING_POOL_MODULE_NAME.to_owned(),
            name: STAKED_IOTA_STRUCT_NAME.to_owned(),
            type_params: vec![],
        }
    }

    pub fn is_staked_iota(s: &StructTag) -> bool {
        s.address == IOTA_SYSTEM_ADDRESS
            && s.module.as_ident_str() == STAKING_POOL_MODULE_NAME
            && s.name.as_ident_str() == STAKED_IOTA_STRUCT_NAME
            && s.type_params.is_empty()
    }

    pub fn id(&self) -> ObjectID {
        self.id.id.bytes
    }

    pub fn pool_id(&self) -> ObjectID {
        self.pool_id.bytes
    }

    pub fn activation_epoch(&self) -> EpochId {
        self.stake_activation_epoch
    }

    pub fn request_epoch(&self) -> EpochId {
        // TODO: this might change when we implement warm up period.
        self.stake_activation_epoch.saturating_sub(1)
    }

    pub fn principal(&self) -> u64 {
        self.principal.value()
    }
}

impl TryFrom<&Object> for StakedIota {
    type Error = IotaError;
    fn try_from(object: &Object) -> Result<Self, Self::Error> {
        match &object.data {
            Data::Move(o) => {
                if o.type_().is_staked_iota() {
                    return bcs::from_bytes(o.contents()).map_err(|err| IotaError::Type {
                        error: format!("Unable to deserialize StakedIota object: {:?}", err),
                    });
                }
            }
            Data::Package(_) => {}
        }

        Err(IotaError::Type {
            error: format!("Object type is not a StakedIota: {:?}", object),
        })
    }
}

pub fn staked_iota_display_version_update_event() -> DisplayVersionUpdatedEvent {
    let id = ID::new(ObjectID::ZERO);
    let version = 0_u16;
    let contents = vec![
        Entry {
            key: "name".to_string(),
            value: "staked_iota".to_string(),
        },
        Entry {
            key: "image_url".to_string(),
            value: "https://d315pvdvxi2gex.cloudfront.net/d96a337f84c5c900f31e08803.svg"
                .to_string(),
        },
    ];
    let fields = VecMap { contents };
    DisplayVersionUpdatedEvent {
        id,
        version,
        fields,
    }
}
