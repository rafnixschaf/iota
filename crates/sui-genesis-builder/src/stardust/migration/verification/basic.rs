// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use anyhow::{anyhow, ensure, Result};
use iota_sdk::types::block::output::{BasicOutput, TokenId};
use sui_types::{
    balance::Balance, coin::Coin, dynamic_field::Field, in_memory_storage::InMemoryStorage, TypeTag,
};

use crate::stardust::migration::{
    executor::FoundryLedgerData,
    verification::{
        created_objects::CreatedObjects,
        util::{
            verify_address_owner, verify_coin, verify_expiration_unlock_condition,
            verify_metadata_feature, verify_native_tokens, verify_parent, verify_sender_feature,
            verify_storage_deposit_unlock_condition, verify_tag_feature,
            verify_timelock_unlock_condition,
        },
        AggregateData,
    },
};

pub(super) fn verify_basic_output(
    output: &BasicOutput,
    created_objects: &CreatedObjects,
    foundry_data: &HashMap<TokenId, FoundryLedgerData>,
    storage: &InMemoryStorage,
    aggregate_data: &mut AggregateData,
) -> Result<()> {
    // If the output has multiple unlock conditions, then a genesis object should
    // have been created.
    if output.unlock_conditions().len() > 1 {
        ensure!(created_objects.coin().is_err(), "unexpected coin created");

        let created_output_obj = created_objects.output().and_then(|id| {
            storage
                .get_object(id)
                .ok_or_else(|| anyhow!("missing object"))
        })?;
        let created_output = created_output_obj
            .to_rust::<crate::stardust::types::output::BasicOutput>()
            .ok_or_else(|| anyhow!("invalid basic output object"))?;

        // Owner
        verify_address_owner(output.address(), created_output_obj, "basic output")?;

        // Amount
        ensure!(
            created_output.iota.value() == output.amount(),
            "amount mismatch: found {}, expected {}",
            created_output.iota.value(),
            output.amount()
        );
        aggregate_data.total_iota_amount += output.amount();
        *aggregate_data
            .address_balances
            .entry(*output.address())
            .or_default() += output.amount();

        // Native Tokens
        verify_native_tokens::<Field<String, Balance>>(
            output.native_tokens(),
            foundry_data,
            created_output.native_tokens,
            created_objects.native_tokens().ok(),
            storage,
        )?;

        // Storage Deposit Return Unlock Condition
        verify_storage_deposit_unlock_condition(
            output.unlock_conditions().storage_deposit_return(),
            created_output.storage_deposit_return.as_ref(),
        )?;

        // Timelock Unlock Condition
        verify_timelock_unlock_condition(
            output.unlock_conditions().timelock(),
            created_output.timelock.as_ref(),
        )?;

        // Expiration Unlock Condition
        verify_expiration_unlock_condition(
            output.unlock_conditions().expiration(),
            created_output.expiration.as_ref(),
            output.address(),
        )?;

        // Metadata Feature
        verify_metadata_feature(
            output.features().metadata(),
            created_output.metadata.as_ref(),
        )?;

        // Tag Feature
        verify_tag_feature(output.features().tag(), created_output.tag.as_ref())?;

        // Sender Feature
        verify_sender_feature(output.features().sender(), created_output.sender)?;

    // Otherwise the output contains only an address unlock condition and only a
    // coin and possibly native tokens should have been created.
    } else {
        ensure!(
            created_objects.output().is_err(),
            "unexpected output object created for simple deposit"
        );

        // Coin value and owner
        verify_coin(output.amount(), output.address(), created_objects, storage)?;

        aggregate_data.total_iota_amount += output.amount();
        *aggregate_data
            .address_balances
            .entry(*output.address())
            .or_default() += output.amount();

        // Native Tokens
        verify_native_tokens::<(TypeTag, Coin)>(
            output.native_tokens(),
            foundry_data,
            None,
            created_objects.native_tokens().ok(),
            storage,
        )?;
    }

    verify_parent(output.address(), storage)?;

    ensure!(
        created_objects.coin_metadata().is_err(),
        "unexpected coin metadata found"
    );

    ensure!(
        created_objects.minted_coin().is_err(),
        "unexpected minted coin found"
    );

    ensure!(
        created_objects.max_supply_policy().is_err(),
        "unexpected max supply policy found"
    );

    ensure!(
        created_objects.package().is_err(),
        "unexpected package found"
    );

    Ok(())
}
