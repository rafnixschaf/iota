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
pub fn is_vested_reward_expired(
    basic_output: &BasicOutput,
    target_milestone_timestamp_sec: u32,
) -> Result<bool> {
    let timelock_uc = timelock_uc(basic_output)?;

    Ok(timelock_uc.timestamp() <= target_milestone_timestamp_sec)
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
pub fn new(
    header: OutputHeader,
    basic_output: BasicOutput,
    target_milestone_timestamp_sec: u32,
) -> Result<TimeLock<Balance>> {
    if basic_output.unlock_conditions().len() != 2 {
        anyhow::bail!("a vested reward must have two unlock conditions");
    }

    if basic_output.native_tokens().len() > 0 {
        anyhow::bail!("a vested reward must not contain native tokens");
    }

    if is_vested_reward_expired(&basic_output, target_milestone_timestamp_sec)? {
        anyhow::bail!("only unexpired vested rewards can be migrated as `TimeLock<Balance<IOTA>>`");
    }

    let id = UID::new(ObjectID::new(header.output_id().hash()));
    let locked = Balance::new(basic_output.amount());

    let timelock_uc = timelock_uc(&basic_output)?;
    let expiration_timestamp_ms = Into::<u64>::into(timelock_uc.timestamp()) * 1000;

    Ok(sui_types::timelock::timelock::TimeLock::new(
        id,
        locked,
        expiration_timestamp_ms,
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
    use std::str::FromStr;

    use iota_sdk::types::block::{
        address::Ed25519Address,
        output::{
            unlock_condition::{
                AddressUnlockCondition, StorageDepositReturnUnlockCondition,
                TimelockUnlockCondition,
            },
            BasicOutput, BasicOutputBuilder, NativeToken, TokenId,
        },
    };

    use crate::stardust::types::{snapshot::OutputHeader, timelock};

    fn vested_reward_header(output_id: &str) -> OutputHeader {
        OutputHeader::new_testing(
            prefix_hex::decode(output_id).unwrap(),
            rand::random(),
            rand::random(),
            rand::random(),
        )
    }

    fn vested_reward_output(amount: u64, expiration_time_sec: u32) -> BasicOutput {
        BasicOutputBuilder::new_with_amount(amount)
            .add_unlock_condition(AddressUnlockCondition::new(
                Ed25519Address::from_str(
                    "0xebe40a263480190dcd7939447ee01aefa73d6f3cc33c90ef7bf905abf8728655",
                )
                .unwrap(),
            ))
            .add_unlock_condition(TimelockUnlockCondition::new(expiration_time_sec).unwrap())
            .finish()
            .unwrap()
    }

    #[test]
    fn test_is_vested_reward() {
        let header = vested_reward_header(
            "0xb191c4bc825ac6983789e50545d5ef07a1d293a98ad974fc9498cb1812345678",
        );

        assert!(timelock::is_vested_reward(&header));
    }

    #[test]
    fn test_is_vested_reward_min_address() {
        let header = vested_reward_header(
            "0xb191c4bc825ac6983789e50545d5ef07a1d293a98ad974fc9498cb1800000000",
        );

        assert!(timelock::is_vested_reward(&header));
    }

    #[test]
    fn test_is_vested_reward_max_address() {
        let header = vested_reward_header(
            "0xb191c4bc825ac6983789e50545d5ef07a1d293a98ad974fc9498cb18ffffffff",
        );

        assert!(timelock::is_vested_reward(&header));
    }

    #[test]
    fn test_is_vested_reward_wrong_address() {
        let header = vested_reward_header(
            "0xb191c4bc825ac6983789e50545d5ef07a1d293a98ad974fc9498cb1712345678",
        );

        assert!(!timelock::is_vested_reward(&header));
    }

    #[test]
    fn test_is_vested_reward_expired() {
        let output = vested_reward_output(10, 1000);

        assert!(timelock::is_vested_reward_expired(&output, 10000).unwrap());
    }

    #[test]
    fn test_is_vested_reward_not_expired() {
        let output = vested_reward_output(10, 1000);

        assert!(!timelock::is_vested_reward_expired(&output, 100).unwrap());
    }

    #[test]
    fn test_is_vested_reward_expired_with_same_ts() {
        let output = vested_reward_output(10, 1000);

        assert!(timelock::is_vested_reward_expired(&output, 1000).unwrap());
    }

    #[test]
    fn test_new_timelock() {
        let header = vested_reward_header(
            "0xb191c4bc825ac6983789e50545d5ef07a1d293a98ad974fc9498cb1812345678",
        );
        let output = vested_reward_output(10, 1000);

        let timelock = timelock::new(header, output, 100).unwrap();

        assert!(timelock.locked().value() == 10);
        assert!(timelock.expiration_timestamp_ms() == 1_000_000);
    }

    #[test]
    fn test_new_timelock_with_expired_output() {
        let header = vested_reward_header(
            "0xb191c4bc825ac6983789e50545d5ef07a1d293a98ad974fc9498cb1812345678",
        );
        let output = vested_reward_output(10, 1000);

        let err = timelock::new(header, output, 1000).unwrap_err();

        assert!(
            err.to_string()
                == "only unexpired vested rewards can be migrated as `TimeLock<Balance<IOTA>>`"
        );
    }

    #[test]
    fn test_new_timelock_extra_unlock_conditions() {
        let header = vested_reward_header(
            "0xb191c4bc825ac6983789e50545d5ef07a1d293a98ad974fc9498cb1812345678",
        );
        let output = BasicOutputBuilder::new_with_amount(10)
            .add_unlock_condition(AddressUnlockCondition::new(
                Ed25519Address::from_str(
                    "0xebe40a263480190dcd7939447ee01aefa73d6f3cc33c90ef7bf905abf8728655",
                )
                .unwrap(),
            ))
            .add_unlock_condition(TimelockUnlockCondition::new(100).unwrap())
            .add_unlock_condition(
                StorageDepositReturnUnlockCondition::new(
                    Ed25519Address::from_str(
                        "0xebe40a263480190dcd7939447ee01aefa73d6f3cc33c90ef7bf905abf8728655",
                    )
                    .unwrap(),
                    100,
                    100,
                )
                .unwrap(),
            )
            .finish()
            .unwrap();

        let err = timelock::new(header, output, 1000).unwrap_err();

        assert!(err.to_string() == "a vested reward must have two unlock conditions");
    }

    #[test]
    fn test_new_timelock_with_native_tokens() {
        let header = vested_reward_header(
            "0xb191c4bc825ac6983789e50545d5ef07a1d293a98ad974fc9498cb1812345678",
        );
        let output = BasicOutputBuilder::new_with_amount(10)
            .add_unlock_condition(AddressUnlockCondition::new(
                Ed25519Address::from_str(
                    "0xebe40a263480190dcd7939447ee01aefa73d6f3cc33c90ef7bf905abf8728655",
                )
                .unwrap(),
            ))
            .add_unlock_condition(TimelockUnlockCondition::new(100).unwrap())
            .add_native_token(NativeToken::new(TokenId::null(), 1).unwrap())
            .finish()
            .unwrap();

        let err = timelock::new(header, output, 1000).unwrap_err();

        assert!(err.to_string() == "a vested reward must not contain native tokens");
    }
}
