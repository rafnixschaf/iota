// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The [`package_data`] module provides the [`NativeTokenPackageData`] struct, which encapsulates all the data necessary to build a Stardust native token package.

use crate::stardust::error::StardustError;
use anyhow::Result;
use iota_sdk::types::block::address::AliasAddress;
use iota_sdk::types::block::output::feature::Irc30Metadata;
use iota_sdk::types::block::output::{FoundryId, FoundryOutput};
use iota_sdk::Url;
use rand::Rng;
use regex::Regex;
use std::fmt;

static ALLOWED_NO_SELF_IDENTIFIERS: &str =
    r"[a-zA-Z]+(?:[a-zA-Z][a-zA-Z0-9_]*)|[a-zA-Z]+(?:_[a-zA-Z0-9_]+)";

/// The [`NativeTokenPackageData`] struct encapsulates all the data necessary to build a Stardust native token package.
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
pub struct NativeTokenModuleData {
    foundry_id: FoundryId,
    module_name: String,
    otw_name: String,
    decimals: u8,
    symbol: String,
    circulating_tokens: u64,
    maximum_supply: u64,
    coin_name: String,
    coin_description: String,
    icon_url: Option<Url>,
    alias_address: AliasAddress,
}

impl NativeTokenModuleData {
    /// Creates a new [`NativeTokenModuleData`] instance.
    pub fn new(
        foundry_id: FoundryId,
        module_name: String,
        otw_name: String,
        decimals: u8,
        symbol: String,
        circulating_tokens: u64,
        maximum_supply: u64,
        coin_name: String,
        coin_description: String,
        icon_url: Option<Url>,
        alias_address: AliasAddress,
    ) -> Self {
        Self {
            foundry_id,
            module_name,
            otw_name,
            decimals,
            symbol,
            circulating_tokens,
            maximum_supply,
            coin_name,
            coin_description,
            icon_url,
            alias_address,
        }
    }

    /// Returns the foundry ID.
    pub fn foundry_id(&self) -> &FoundryId {
        &self.foundry_id
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

    /// Returns the number of minted tokens.
    pub fn circulating_tokens(&self) -> u64 {
        self.circulating_tokens
    }

    /// Returns the maximum supply.
    pub fn maximum_supply(&self) -> u64 {
        self.maximum_supply
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
    pub fn alias_address(&self) -> &AliasAddress {
        &self.alias_address
    }
}

impl TryFrom<FoundryOutput> for NativeTokenPackageData {
    type Error = StardustError;
    fn try_from(output: FoundryOutput) -> Result<Self, StardustError> {
        let metadata =
            output
                .features()
                .metadata()
                .ok_or(StardustError::FoundryConversionError {
                    foundry_id: output.id(),
                    err: anyhow::anyhow!("metadata not found"),
                })?;
        let irc_30_metadata: Irc30Metadata =
            serde_json::from_slice(metadata.data()).map_err(|e| {
                StardustError::FoundryConversionError {
                    foundry_id: output.id(),
                    err: e.into(),
                }
            })?;

        let symbol = check_identifier(irc_30_metadata.symbol().to_string().to_ascii_lowercase());

        let decimals = u8::try_from(*irc_30_metadata.decimals()).map_err(|e| {
            StardustError::FoundryConversionError {
                foundry_id: output.id(),
                err: e.into(),
            }
        })?;

        let maximum_supply = output.token_scheme().as_simple().maximum_supply();
        if maximum_supply.bits() > 64 {
            return Err(StardustError::FoundryConversionError {
                foundry_id: output.id(),
                err: anyhow::anyhow!("maximum supply exceeds u64"),
            });
        }

        let native_token_data = NativeTokenPackageData {
            move_toml: MoveTomlManifest {
                package_name: symbol.to_lowercase(),
            },
            module: NativeTokenModuleData {
                foundry_id: output.id(),
                module_name: symbol.to_lowercase(),
                otw_name: symbol.clone().to_ascii_uppercase(),
                decimals,
                symbol,
                circulating_tokens: output.token_scheme().as_simple().minted_tokens().as_u64()
                    - output.token_scheme().as_simple().melted_tokens().as_u64(), //we know that "Melted Tokens must not be greater than Minted Tokens"
                maximum_supply: maximum_supply.as_u64(),
                coin_name: irc_30_metadata.name().to_owned(),
                coin_description: irc_30_metadata.description().clone().unwrap_or_default(),
                icon_url: irc_30_metadata.url().clone(),
                alias_address: *output.alias_address(),
            },
        };

        Ok(native_token_data)
    }
}

impl fmt::Display for NativeTokenPackageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NativeTokenPackageData: {:?}", self)
    }
}

fn check_identifier(identifier: String) -> String {
    // If it is a valid Move idenfier return it
    if move_core_types::identifier::is_valid(&identifier) {
        identifier
    } else {
        // Else try to remove the unvalid characters
        let re: Regex = Regex::new(ALLOWED_NO_SELF_IDENTIFIERS).unwrap();
        if let Some(mat) = re.find(&identifier) {
            mat.as_str().to_owned()
        } else {
            // Generate a new valid random identifier
            let mut rng = rand::thread_rng();
            let gen = rand_regex::Regex::compile(ALLOWED_NO_SELF_IDENTIFIERS, 100).unwrap();
            let res: String = (&mut rng).sample(&gen);
            if res.len() > 7 {
                res[..7].to_string()
            } else {
                res
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::stardust::native_token::package_builder::PackageBuilder;
    use iota_sdk::types::block::address::AliasAddress;
    use iota_sdk::types::block::output::feature::MetadataFeature;
    use iota_sdk::types::block::output::unlock_condition::ImmutableAliasAddressUnlockCondition;
    use iota_sdk::types::block::output::{
        AliasId, Feature, FoundryOutputBuilder, SimpleTokenScheme, TokenScheme,
    };
    use iota_sdk::U256;

    use super::*;

    #[test]
    fn test_foundry_output_with_default_metadata() -> Result<()> {
        // Step 1: Create a FoundryOutput with an IRC30Metadata feature
        let token_scheme = SimpleTokenScheme::new(
            U256::from(100_000_000),
            U256::from(0),
            U256::from(100_000_000),
        )
        .unwrap();

        let irc_30_metadata = Irc30Metadata::new("Dogecoin", "DOGE❤", 0);

        let alias_id = AliasId::new([0; AliasId::LENGTH]);
        let builder = FoundryOutputBuilder::new_with_amount(
            100_000_000_000,
            1,
            TokenScheme::Simple(token_scheme),
        )
        .add_unlock_condition(ImmutableAliasAddressUnlockCondition::new(
            AliasAddress::new(alias_id),
        ))
        .add_feature(Feature::Metadata(
            MetadataFeature::new(irc_30_metadata).unwrap(),
        ));
        let output = builder.finish().unwrap();

        // Step 2: Convert the FoundryOutput to NativeTokenPackageData
        let native_token_data = NativeTokenPackageData::try_from(output)?;

        // Step 3: Verify the conversion
        let package_builder = PackageBuilder;
        assert!(package_builder.build_and_compile(native_token_data).is_ok());

        Ok(())
    }

    #[test]
    fn test_foundry_output_with_additional_metadata() -> Result<()> {
        // Step 1: Create a FoundryOutput with an IRC30Metadata feature
        let token_scheme = SimpleTokenScheme::new(
            U256::from(100_000_000),
            U256::from(0),
            U256::from(100_000_000),
        )
        .unwrap();

        let irc_30_metadata = Irc30Metadata::new("Dogecoin", "DOGE", 0)
            .with_description("Much wow")
            .with_url(Url::parse("https://dogecoin.com").unwrap())
            .with_logo_url(Url::parse("https://dogecoin.com/logo.png").unwrap())
            .with_logo("0x54654");

        let alias_id = AliasId::new([0; AliasId::LENGTH]);
        let builder = FoundryOutputBuilder::new_with_amount(
            100_000_000_000,
            1,
            TokenScheme::Simple(token_scheme),
        )
        .add_unlock_condition(ImmutableAliasAddressUnlockCondition::new(
            AliasAddress::new(alias_id),
        ))
        .add_feature(Feature::Metadata(
            MetadataFeature::new(irc_30_metadata).unwrap(),
        ));
        let output = builder.finish().unwrap();

        // Step 2: Convert the FoundryOutput to NativeTokenPackageData
        let native_token_data = NativeTokenPackageData::try_from(output)?;

        // Step 3: Verify the conversion
        let package_builder = PackageBuilder;
        assert!(package_builder.build_and_compile(native_token_data).is_ok());

        Ok(())
    }
}
