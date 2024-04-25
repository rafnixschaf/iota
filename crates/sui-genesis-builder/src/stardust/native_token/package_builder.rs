// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! The [`package_builder`] module provides the [`PackageBuilder`] struct, which is responsible for building and compiling Stardust native token packages.
use std::path::{Path, PathBuf};
use std::{fs, io};

use anyhow::Result;
use tracing::warn;

use sui_move_build::{BuildConfig, CompiledPackage};

use crate::stardust::native_token::package_data::NativeTokenPackageData;

/// The [`PackageBuilder`] struct is responsible for building and compiling Stardust native token packages.
pub struct PackageBuilder;
impl PackageBuilder {

    /// Builds and compiles a Stardust native token package.
    pub fn build_and_compile(&self, package: NativeTokenPackageData) -> Result<CompiledPackage> {
        let package_template_path =
            Path::new("crates/sui-genesis-builder/src/stardust/native_token/package_template");

        let new_package_path = Path::new("crates/sui-genesis-builder/src/stardust/native_token")
            .join(format!(
                "native_token_package_{}",
                package.module().native_token_id()
            ));

        // Step 1: Copy the template package directory
        Self::copy_template_dir(package_template_path, &new_package_path)?;

        // Step 2: Adjust the Move.toml file
        Self::adjust_move_toml(&new_package_path, &package)?;

        // Step 3: Replace template variables in the .move file
        Self::adjust_native_token_module(&new_package_path, &package)?;

        // Step 4: Compile the package
        let compiled_package = BuildConfig::default().build(new_package_path)?;

        Ok(compiled_package)
    }

    // Copies the contents of a directory to a new location.
    fn copy_template_dir(from: &Path, to: &PathBuf) -> Result<()> {
        // Step 1: Create a new folder for the package and copy the contents
        if to.exists() {
            warn!(
                "Path {} already exists. Deleting the existing directory.",
                to.display()
            );
            fs::remove_dir_all(to)?;
        }
        fs::create_dir_all(to.join("sources"))?;

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
        recursive_copy(from, to)?;

        Ok(())
    }

    // Adjusts the Move.toml file with the package name and alias address.
    fn adjust_move_toml(package_path: &Path, package: &NativeTokenPackageData) -> Result<()> {
        let cargo_toml_path = package_path.join("Move.toml");
        let contents = fs::read_to_string(&cargo_toml_path)?;
        let new_contents = contents
            .replace("$PACKAGE_NAME", package.move_toml().package_name())
            .replace("$ALIAS", package.module().alias_address());
        fs::write(&cargo_toml_path, new_contents)?;

        Ok(())
    }

    // Replaces template variables in the .move file with the actual values.
    fn adjust_native_token_module(
        package_path: &Path,
        package: &NativeTokenPackageData,
    ) -> Result<()> {
        let old_move_file_path = package_path.join("sources/native_token_template.move");
        let new_move_file_name = format!("{}.move", package.module().module_name());
        let new_move_file_path = package_path.join("sources").join(new_move_file_name);

        // Rename the template .move file
        fs::rename(old_move_file_path, &new_move_file_path)?;

        let contents = fs::read_to_string(&new_move_file_path)?;

        let icon_url = match &package.module().icon_url() {
            Some(url) => format!(
                "option::some<Url>(sui::url::new_unsafe_from_bytes(b\"{}\"))",
                url
            ),
            None => "option::none<Url>()".to_string(),
        };

        let new_contents = contents
            .replace("$MODULE_NAME", package.module().module_name())
            .replace("$OTW", package.module().otw_name())
            .replace("$COIN_DECIMALS", &package.module().decimals().to_string())
            .replace("$COIN_SYMBOL", package.module().symbol())
            .replace("$COIN_NAME", package.module().coin_name())
            .replace("$COIN_DESCRIPTION", package.module().coin_description())
            .replace("$ICON_URL", &icon_url)
            .replace("$ALIAS_ADDRESS", package.module().alias_address());

        fs::write(&new_move_file_path, new_contents)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::io::Write;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_copy_template_dir_success() {
        // Set up a temporary directory as the environment for the test
        let tmp_dir = tempdir().unwrap();
        let test_package_path = tmp_dir.path().join("package_template");
        fs::create_dir_all(&test_package_path).unwrap();

        // Simulate existing files and directories that the function expects to copy
        let src_dir = test_package_path.join("sources");
        fs::create_dir_all(&src_dir).unwrap();
        let test_file_path = src_dir.join("test.move");
        let mut file = File::create(test_file_path).unwrap();
        writeln!(file, "0x0::test {{}}").unwrap();

        // Define the target directory for the files to be copied
        let target_dir = tmp_dir.path().join("target_package");

        // Copy the files
        let result = PackageBuilder::copy_template_dir(test_package_path.as_path(), &target_dir);

        assert!(result.is_ok());
        assert!(target_dir.exists());
        assert!(target_dir.join("sources").join("test.move").exists());
    }
}
