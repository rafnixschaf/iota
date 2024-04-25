// SPDX-License-Identifier: Apache-2.0

//! The [`package_data`] module provides the [`NativeTokenPackageData`] struct, which encapsulates all the data necessary to build a Stardust native token package.

use iota_sdk::Url;

/// The [`NativeTokenPackageData`] struct encapsulates all the data necessary to build a Stardust native token package.
pub struct NativeTokenPackageData {
    move_toml: MoveTomlManifest,
    module: NativeTokenModuleData,
}
impl NativeTokenPackageData {
    /// Creates a new [`NativeTokenPackageData`] instance.
    pub fn new(cargo_toml_manifest: MoveTomlManifest, module: NativeTokenModuleData) -> Self {
        Self {
            move_toml: cargo_toml_manifest,
            module,
        }
    }

    /// Returns the Move.toml manifest.
    pub fn move_toml(&self) -> &MoveTomlManifest {
        &self.move_toml
    }

    /// Returns the native token module data.
    pub fn module(&self) -> &NativeTokenModuleData {
        &self.module
    }
}

/// The [`MoveTomlManifest`] struct encapsulates all the data necessary to build a Move.toml manifest.
pub struct MoveTomlManifest {
    package_name: String,
}

impl MoveTomlManifest {
    /// Creates a new [`MoveTomlManifest`] instance.
    pub fn new(package_name: String) -> Self {
        Self { package_name }
    }

    /// Returns the package name.
    pub fn package_name(&self) -> &str {
        &self.package_name
    }
}

/// The [`NativeTokenModuleData`] struct encapsulates all the data necessary to build a Stardust native token module.
pub struct NativeTokenModuleData {
    native_token_id: String,
    module_name: String,
    otw_name: String,
    decimals: u8,
    symbol: String,
    coin_name: String,
    coin_description: String,
    icon_url: Option<Url>,
    alias_address: String,
}

impl NativeTokenModuleData {
    /// Creates a new [`NativeTokenModuleData`] instance.
    pub fn new(
        native_token_id: String,
        module_name: String,
        otw_name: String,
        decimals: u8,
        symbol: String,
        coin_name: String,
        coin_description: String,
        icon_url: Option<Url>,
        alias_address: String,
    ) -> Self {
        Self {
            native_token_id,
            module_name,
            otw_name,
            decimals,
            symbol,
            coin_name,
            coin_description,
            icon_url,
            alias_address,
        }
    }

    /// Returns the native token ID.
    pub fn native_token_id(&self) -> &str {
        &self.native_token_id
    }

    /// Returns the module name.
    pub fn module_name(&self) -> &str {
        &self.module_name
    }

    /// Returns the One-Time-Witness (OTW) name.
    pub fn otw_name(&self) -> &str {
        &self.otw_name
    }

    /// Returns the number of decimals.
    pub fn decimals(&self) -> u8 {
        self.decimals
    }

    /// Returns the symbol.
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    /// Returns the coin name.
    pub fn coin_name(&self) -> &str {
        &self.coin_name
    }

    /// Returns the coin description.
    pub fn coin_description(&self) -> &str {
        &self.coin_description
    }

    /// Returns the icon URL.
    pub fn icon_url(&self) -> &Option<Url> {
        &self.icon_url
    }

    /// Returns the alias address.
    pub fn alias_address(&self) -> &str {
        &self.alias_address
    }
}
