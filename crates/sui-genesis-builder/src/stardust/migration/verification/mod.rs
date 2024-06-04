// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The [`verification`] module contains the validation logic to make sure that
//! the stardust outputs are correctly converted to the move objects.

use std::collections::HashMap;

use anyhow::{bail, ensure};
use iota_sdk::types::block::{
    address::Address,
    output::{Output, OutputId, TokenId},
};
use sui_types::in_memory_storage::InMemoryStorage;

use self::created_objects::CreatedObjects;
use crate::stardust::{migration::executor::FoundryLedgerData, types::snapshot::OutputHeader};

pub mod alias;
pub mod basic;
pub mod created_objects;
pub mod foundry;
pub mod nft;
mod util;

#[derive(Debug, Default)]
struct AggregateData {
    pub total_iota_amount: u64,
    pub address_balances: HashMap<Address, u64>,
}

pub(crate) fn verify_outputs<'a>(
    outputs: impl IntoIterator<Item = &'a (OutputHeader, Output)>,
    output_objects_map: &HashMap<OutputId, CreatedObjects>,
    foundry_data: &HashMap<TokenId, FoundryLedgerData>,
    total_supply: u64,
    storage: &InMemoryStorage,
) -> anyhow::Result<()> {
    let mut input_balances = HashMap::new();
    let mut output_data = AggregateData::default();
    for (header, output) in outputs {
        let created_objects = output_objects_map.get(&header.output_id()).ok_or_else(|| {
            anyhow::anyhow!("missing created objects for output {}", header.output_id())
        })?;
        verify_output(
            header,
            output,
            created_objects,
            foundry_data,
            storage,
            &mut input_balances,
            &mut output_data,
        )?;
    }
    ensure!(
        output_data.total_iota_amount == total_supply,
        "total amount mismatch: found {}, expected {}",
        output_data.total_iota_amount,
        total_supply
    );
    // Ensure all input balances match output balances
    for (address, expected) in input_balances {
        // Remove the entries so that we can ensure none are left over
        let found = output_data.address_balances.remove(&address);
        ensure!(
            found == Some(expected),
            "address balance mismatch for {address}: found {found:?}, expected {expected}",
        );
    }
    // Check if there are still unexpected balances in the outputs
    for (address, unexpected) in output_data.address_balances {
        bail!("unexpected output address balance for {address}: {unexpected}");
    }
    Ok(())
}

fn verify_output(
    header: &OutputHeader,
    output: &Output,
    created_objects: &CreatedObjects,
    foundry_data: &HashMap<TokenId, FoundryLedgerData>,
    storage: &InMemoryStorage,
    input_balances: &mut HashMap<Address, u64>,
    aggregate_data: &mut AggregateData,
) -> anyhow::Result<()> {
    match output {
        Output::Alias(output) => {
            *input_balances
                .entry(*output.governor_address())
                .or_default() += output.amount();
            alias::verify_alias_output(
                header.output_id(),
                output,
                created_objects,
                foundry_data,
                storage,
                aggregate_data,
            )
        }
        Output::Basic(output) => {
            *input_balances.entry(*output.address()).or_default() += output.amount();
            basic::verify_basic_output(
                output,
                created_objects,
                foundry_data,
                storage,
                aggregate_data,
            )
        }
        Output::Foundry(output) => {
            *input_balances
                .entry(
                    *output
                        .unlock_conditions()
                        .immutable_alias_address()
                        .expect("foundry outputs always have an alias address")
                        .address(),
                )
                .or_default() += output.amount();
            foundry::verify_foundry_output(
                output,
                created_objects,
                foundry_data,
                storage,
                aggregate_data,
            )
        }
        Output::Nft(output) => {
            *input_balances.entry(*output.address()).or_default() += output.amount();
            nft::verify_nft_output(
                header.output_id(),
                output,
                created_objects,
                foundry_data,
                storage,
                aggregate_data,
            )
        }
        // Treasury outputs aren't used since Stardust, so no need to verify anything here.
        Output::Treasury(_) => return Ok(()),
    }
    .map_err(|e| anyhow::anyhow!("error verifying output {}: {}", header.output_id(), e))
}
