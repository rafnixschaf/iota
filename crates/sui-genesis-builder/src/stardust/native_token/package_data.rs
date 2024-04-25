// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::stardust::native_token::module_data::NativeTokenModuleData;

pub struct NativeTokenPackageData {
    move_toml: MoveTomlManifest,
    module: NativeTokenModuleData,
}

pub struct MoveTomlManifest {
    package_name: String,
}

impl MoveTomlManifest {
    pub fn new(package_name: String) -> Self {
        Self { package_name }
    }

    pub fn package_name(&self) -> &str {
        &self.package_name
    }
}

impl NativeTokenPackageData {
    pub fn new(cargo_toml_manifest: MoveTomlManifest, module: NativeTokenModuleData) -> Self {
        Self {
            move_toml: cargo_toml_manifest,
            module,
        }
    }

    pub fn move_toml(&self) -> &MoveTomlManifest {
        &self.move_toml
    }

    pub fn module(&self) -> &NativeTokenModuleData {
        &self.module
    }
}
