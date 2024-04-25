// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::stardust::native_token::module::NativeTokenModule;
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::{fs, io};
use sui_move_build::{BuildConfig, CompiledPackage};
use tracing::warn;

pub struct NativeTokenPackage {
    move_toml: MoveTomlManifest,
    module: NativeTokenModule,
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

impl NativeTokenPackage {
    pub fn new(cargo_toml_manifest: MoveTomlManifest, module: NativeTokenModule) -> Self {
        Self {
            move_toml: cargo_toml_manifest,
            module,
        }
    }

    pub fn cargo_toml_manifest(&self) -> &MoveTomlManifest {
        &self.move_toml
    }

    pub fn module(&self) -> &NativeTokenModule {
        &self.module
    }

    pub fn build_and_compile(&self) -> Result<CompiledPackage> {
        // Step 1: Copy the template package directory
        let package_path = self.copy_template_dir()?;

        // Step 2: Adjust the Move.toml file
        self.adjust_move_toml(&package_path)?;

        // Step 3: Replace template variables in the .move file
        self.adjust_native_token_module(&package_path)?;

        // Step 4: Compile the package
        let compiled_package = BuildConfig::default().build(package_path)?;

        Ok(compiled_package)
    }

    fn copy_template_dir(&self) -> Result<PathBuf> {
        // Step 1: Create a new folder for the package and copy the contents
        let package_template_path =
            Path::new("crates/sui-genesis-builder/src/stardust/native_token/package_template");
        let new_package_name = format!("native_token_package_{}", self.module.native_token_id());
        let new_package_path = Path::new("crates/sui-genesis-builder/src/stardust/native_token")
            .join(&new_package_name);
        if new_package_path.exists() {
            warn!(
                "Package with name {} already exists. Deleting the existing package.",
                new_package_name
            );
            fs::remove_dir_all(&new_package_path)?;
        }
        fs::create_dir_all(&new_package_path.join("sources"))?;

        // Recursive copy function to handle directories and files
        fn recursive_copy(src: &Path, dst: &Path) -> io::Result<()> {
            if src.is_dir() {
                if !dst.exists() {
                    fs::create_dir(dst)?;
                }
                for entry in fs::read_dir(src)? {
                    let entry = entry?;
                    let file_type = entry.file_type()?;
                    if file_type.is_dir() {
                        recursive_copy(&entry.path(), &dst.join(entry.file_name()))?;
                    } else {
                        fs::copy(&entry.path(), dst.join(entry.file_name()))?;
                    }
                }
            }
            Ok(())
        }
        recursive_copy(&package_template_path, &new_package_path)?;

        Ok(new_package_path)
    }

    fn adjust_move_toml(&self, package_path: &PathBuf) -> Result<()> {
        let cargo_toml_path = package_path.join("Move.toml");
        let contents = fs::read_to_string(&cargo_toml_path)?;
        let new_contents = contents
            .replace("$PACKAGE_NAME", &self.move_toml.package_name())
            .replace("$ALIAS", &self.module.alias_address());
        fs::write(&cargo_toml_path, new_contents)?;

        Ok(())
    }

    fn adjust_native_token_module(&self, package_path: &PathBuf) -> Result<()> {
        let old_move_file_path = package_path.join("sources/native_token_template.move");
        let new_move_file_name = format!("{}.move", self.module.module_name());
        let new_move_file_path = package_path.join("sources").join(&new_move_file_name);

        // Rename the template .move file
        fs::rename(&old_move_file_path, &new_move_file_path)?;

        let contents = fs::read_to_string(&new_move_file_path)?;

        let icon_url = match &self.module.icon_url() {
            Some(url) => format!(
                "option::some<Url>(sui::url::new_unsafe_from_bytes(b\"{}\"))",
                url
            ),
            None => "option::none<Url>()".to_string(),
        };

        let new_contents = contents
            .replace("$MODULE_NAME", &self.module.module_name())
            .replace("$OTW", &self.module.otw_name())
            .replace("$COIN_DECIMALS", &self.module.decimals().to_string())
            .replace("$COIN_SYMBOL", &self.module.symbol())
            .replace("$COIN_NAME", &self.module.coin_name())
            .replace("$COIN_DESCRIPTION", &self.module.coin_description())
            .replace("$ICON_URL", &icon_url)
            .replace("$ALIAS_ADDRESS", &self.module.alias_address());

        fs::write(&new_move_file_path, new_contents)?;

        Ok(())
    }
}
