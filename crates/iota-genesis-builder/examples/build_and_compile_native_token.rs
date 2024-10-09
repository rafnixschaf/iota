// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Example demonstrating building and compiling two native token packages.

use iota_genesis_builder::stardust::native_token::{
    package_builder,
    package_data::{NativeTokenModuleData, NativeTokenPackageData},
};
use iota_sdk::types::block::{
    address::AliasAddress,
    output::{AliasId, FoundryId},
};

fn main() -> anyhow::Result<()> {
    let native_token_a = NativeTokenPackageData::new(
        "doge_coin",
        NativeTokenModuleData::new(
            FoundryId::new([0; FoundryId::LENGTH]),
            "doge",
            "DOGE",
            0,
            "DOGE",
            0,
            100_000_000_000,
            "Dogecoin",
            "Much wow",
            Some("https://raw.githubusercontent.com/dogecoin/dogecoin/master/share/pixmaps/dogecoin256.png".to_string()),
            AliasAddress::new(AliasId::new([0; AliasId::LENGTH]))
        ),
    );

    println!("DOGE token: {:?}", native_token_a);

    let compiled_package_a = package_builder::build_and_compile(native_token_a)?;
    println!(
        "Compiled package: {:?}",
        compiled_package_a.package.compiled_package_info
    );

    Ok(())
}
