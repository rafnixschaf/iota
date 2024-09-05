// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Example demonstrating the conversion of a stardust NFT into a custom user's
//! NFT. In order to work, it requires a network with test objects
//! generated from iota-genesis-builder/src/stardust/test_outputs.

use std::{fs, path::PathBuf, str::FromStr};

use anyhow::{anyhow, Result};
use iota_keys::keystore::{AccountKeystore, FileBasedKeystore};
use iota_move_build::BuildConfig;
use iota_replay::{execute_replay_command, ReplayToolCommand};
use iota_sdk::{
    rpc_types::IotaTransactionBlockResponseOptions,
    types::{
        base_types::IotaAddress,
        crypto::SignatureScheme::ED25519,
        programmable_transaction_builder::ProgrammableTransactionBuilder,
        quorum_driver_types::ExecuteTransactionRequestType,
        transaction::{Transaction, TransactionData},
    },
    IotaClient, IotaClientBuilder,
};
use shared_crypto::intent::Intent;
/// Got from iota-genesis-builder/src/stardust/test_outputs/stardust_mix.rs
const MAIN_ADDRESS_MNEMONIC: &str = "okay pottery arch air egg very cave cash poem gown sorry mind poem crack dawn wet car pink extra crane hen bar boring salt";
const PACKAGE_PATH: &str = "../../move/tx_instance";
const SNAPSHOTS_FOLDER: &str = "../../../tests/sandbox_snapshots";
const LOCALNET_ADDR: &str = "http://127.0.0.1:9000";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Build an iota client for a local network
    let iota_client = IotaClientBuilder::default().build_localnet().await?;
    // Localnet address, iota_client doesn't have such getter
    let localnet_addr = String::from_str(LOCALNET_ADDR).unwrap();

    let mut keystore = setup_keystore()?;

    let snapshot_path = PathBuf::from(SNAPSHOTS_FOLDER);

    // Derive the address of the first account and set it as default
    let sender = keystore.import_from_mnemonic(MAIN_ADDRESS_MNEMONIC, ED25519, None)?;

    println!("{sender:?}");

    // Publish the package of a custom NFT collection and then get the package id.
    let tx_digest = publish_package(sender, &mut keystore, &iota_client, PACKAGE_PATH).await?;

    let cmd = ReplayToolCommand::PersistSandbox {
        tx_digest,
        base_path: snapshot_path.into(),
    };

    execute_replay_command(Some(localnet_addr), true, true, None, cmd).await?;
    clean_keystore()
}

fn setup_keystore() -> Result<FileBasedKeystore, anyhow::Error> {
    // Create a temporary keystore
    let keystore_path = PathBuf::from("iotatempdb");
    if !keystore_path.exists() {
        let keystore = FileBasedKeystore::new(&keystore_path)?;
        keystore.save()?;
    }
    // Read iota keystore
    FileBasedKeystore::new(&keystore_path)
}

fn clean_keystore() -> Result<(), anyhow::Error> {
    // Remove files
    fs::remove_file("iotatempdb")?;
    fs::remove_file("iotatempdb.aliases")?;
    Ok(())
}

async fn publish_package(
    sender: IotaAddress,
    keystore: &mut FileBasedKeystore,
    iota_client: &IotaClient,
    package_path: &str,
) -> Result<String> {
    // Get a gas coin
    let gas_coin = iota_client
        .coin_read_api()
        .get_coins(sender, None, None, None)
        .await?
        .data
        .into_iter()
        .next()
        .ok_or(anyhow!("No coins found"))?;

    // Build custom nft package
    let compiled_package = BuildConfig::default().build(package_path.into())?;
    let modules = compiled_package
        .get_modules()
        .map(|module| {
            let mut buf = Vec::new();
            module.serialize(&mut buf)?;
            Ok(buf)
        })
        .collect::<Result<Vec<Vec<u8>>>>()?;
    let dependencies = compiled_package.get_dependency_original_package_ids();

    // Publish package
    let pt = {
        let mut builder = ProgrammableTransactionBuilder::new();
        builder.publish_immutable(modules, dependencies);
        builder.finish()
    };

    // Setup gas budget and gas price
    let gas_budget = 50_000_000;
    let gas_price = iota_client.read_api().get_reference_gas_price().await?;

    // Create the transaction data that will be sent to the network
    let tx_data = TransactionData::new_programmable(
        sender,
        vec![gas_coin.object_ref()],
        pt,
        gas_budget,
        gas_price,
    );

    // Sign the transaction
    let signature = keystore.sign_secure(&sender, &tx_data, Intent::iota_transaction())?;

    // Execute transaction
    let transaction_response = iota_client
        .quorum_driver_api()
        .execute_transaction_block(
            Transaction::from_data(tx_data, vec![signature]),
            IotaTransactionBlockResponseOptions::full_content(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    
    let tx_digest = transaction_response.digest;
    println!("Package publishing transaction digest: {}", tx_digest);

    Ok(tx_digest.base58_encode())
}
