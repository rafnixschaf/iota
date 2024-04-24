// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Example demonstrating building and compiling two native token packages.

use sui_genesis_builder::stardust::native_token::module::NativeTokenModule;
use sui_genesis_builder::stardust::native_token::package::{MoveTomlManifest, NativeTokenPackage};

fn main() -> anyhow::Result<()> {
    let native_token_a = NativeTokenPackage::new(
        MoveTomlManifest::new("native_token_example".to_string()),
        NativeTokenModule::new(
            "324823948".to_string(),
            "doge".to_string(),
            "DOGE".to_string(),
            0,
            "DOGE".to_string(),
            "Dogecoin".to_string(),
            "Much wow".to_string(),
            Option::None,
            "0x54654".to_string(),
        ),
    );

    let compiled_package_a = native_token_a.build_and_compile()?;
    println!("Compiled package: {:?}", compiled_package_a);

    let native_token_b = NativeTokenPackage::new(
        MoveTomlManifest::new("native_token_example".to_string()),
        NativeTokenModule::new(
            "34543525".to_string(),
            "smr".to_string(),
            "SMR".to_string(),
            0,
            "SMR".to_string(),
            "Shimmer".to_string(),
            "Shimmy Shimmy Ya".to_string(),
            Option::None,
            "0x54654".to_string(),
        ),
    );

    let compiled_package_b = native_token_b.build_and_compile()?;
    println!("Compiled package: {:?}", compiled_package_b);

    Ok(())
}
