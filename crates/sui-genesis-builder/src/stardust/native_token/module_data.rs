// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::Url;

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

    pub fn native_token_id(&self) -> &str {
        &self.native_token_id
    }

    pub fn module_name(&self) -> &str {
        &self.module_name
    }

    pub fn otw_name(&self) -> &str {
        &self.otw_name
    }

    pub fn decimals(&self) -> u8 {
        self.decimals
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn coin_name(&self) -> &str {
        &self.coin_name
    }

    pub fn coin_description(&self) -> &str {
        &self.coin_description
    }

    pub fn icon_url(&self) -> &Option<Url> {
        &self.icon_url
    }

    pub fn alias_address(&self) -> &str {
        &self.alias_address
    }
}
