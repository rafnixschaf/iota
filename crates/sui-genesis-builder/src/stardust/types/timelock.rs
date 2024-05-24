// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::output::{unlock_condition::TimelockUnlockCondition, BasicOutput};
use sui_protocol_config::ProtocolConfig;
use sui_types::{
    balance::Balance,
    base_types::{MoveObjectType, ObjectID, SequenceNumber, SuiAddress, TxContext},
    id::UID,
    object::{Data, MoveObject, Object, Owner},
    timelock::timelock::TimeLock,
};

use super::snapshot::OutputHeader;

use anyhow::Result;

/// All basic outputs whose IDs start with this prefix represent vested rewards
/// that were created during the stardust upgrade on IOTA mainnet.
const VESTED_REWARD_ID_PREFIX: &str = "0xb191c4bc825ac6983789e50545d5ef07a1d293a98ad974fc9498cb18";

/// Checks if an output is a vested reward were created during the merge.
pub fn is_vested_reward(header: &OutputHeader) -> bool {
    header
        .output_id()
        .to_string()
        .starts_with(VESTED_REWARD_ID_PREFIX)
}

/// Checks if a vested_reward is expired.
pub fn is_vested_reward_expired(header: &OutputHeader, basic_output: &BasicOutput) -> Result<bool> {
    let timelock_uc = timelock_uc(basic_output)?;

    Ok(timelock_uc.timestamp() <= header.ms_timestamp())
}

/// Gets an output timelock unlock condition.
fn timelock_uc(basic_output: &BasicOutput) -> Result<&TimelockUnlockCondition> {
    basic_output
        .unlock_conditions()
        .timelock()
        .ok_or(anyhow::anyhow!(
            "a vested reward must have a timelock unlock condition"
        ))
}

/// Creates a new time-locked balance.
pub fn new(header: OutputHeader, basic_output: BasicOutput) -> Result<TimeLock<Balance>> {
    if basic_output.unlock_conditions().len() != 1 {
        anyhow::bail!("a vested reward must have one unlock condition");
    }

    if basic_output.native_tokens().len() > 0 {
        anyhow::bail!("a vested reward must not contain native tokens");
    }

    if is_vested_reward_expired(&header, &basic_output)? {
        anyhow::bail!("only unexpired vested rewards can be migrated as `TimeLock<Balance<IOTA>>`");
    }

    let timelock_uc = timelock_uc(&basic_output)?;

    let id = UID::new(ObjectID::new(header.output_id().hash()));
    let locked = Balance::new(basic_output.amount());
    let expiration_timestamp_ms = timelock_uc.timestamp();

    Ok(sui_types::timelock::timelock::TimeLock::new(
        id,
        locked,
        expiration_timestamp_ms.into(),
    ))
}

/// Creates a genesis object from a time-locked balance.
pub fn to_genesis_object(
    timelock: TimeLock<Balance>,
    owner: SuiAddress,
    protocol_config: &ProtocolConfig,
    tx_context: &TxContext,
    version: SequenceNumber,
) -> Result<Object> {
    let move_object = unsafe {
        // Safety: we know from the definition of `TimeLock` in the timelock package
        // that it has not public transfer (`store` ability is absent).
        MoveObject::new_from_execution(
            MoveObjectType::timelocked_sui_balance(),
            false,
            version,
            timelock.to_bcs_bytes(),
            protocol_config,
        )?
    };

    Ok(Object::new_from_genesis(
        Data::Move(move_object),
        Owner::AddressOwner(owner),
        tx_context.digest(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::stardust::types::{snapshot::OutputHeader, timelock};

    fn output_header(output_id: &str) -> OutputHeader {
        OutputHeader::new_testing(
            prefix_hex::decode(output_id).unwrap(),
            rand::random(),
            rand::random(),
            rand::random(),
        )
    }

    #[test]
    fn test_is_vested_reward() {
        let header =
            output_header("0xb191c4bc825ac6983789e50545d5ef07a1d293a98ad974fc9498cb1812345678");

        assert!(timelock::is_vested_reward(&header));
    }

    #[test]
    fn test_is_vested_reward_min_address() {
        let header =
            output_header("0xb191c4bc825ac6983789e50545d5ef07a1d293a98ad974fc9498cb1800000000");

        assert!(timelock::is_vested_reward(&header));
    }

    #[test]
    fn test_is_vested_reward_max_address() {
        let header =
            output_header("0xb191c4bc825ac6983789e50545d5ef07a1d293a98ad974fc9498cb18ffffffff");

        assert!(timelock::is_vested_reward(&header));
    }

    #[test]
    fn test_is_vested_reward_wrong_address() {
        let header =
            output_header("0xb191c4bc825ac6983789e50545d5ef07a1d293a98ad974fc9498cb1712345678");

        assert!(!timelock::is_vested_reward(&header));
    }
}
