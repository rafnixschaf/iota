// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{fs, path::PathBuf};

const SUI_DIR: &str = ".sui";
const SUI_CONFIG_DIR: &str = "config";
/// This is name of the file-based key-store that will be used
/// for persisting keys through the command line.
pub const SUI_KEYSTORE_FILENAME: &str = "sui.keystore";
pub const SUI_CLIENT_CONFIG: &str = "client.yaml";

/// Get the sui configuration directory path, or create
/// a new configuration folder under the home folder
pub fn sui_config_dir() -> Result<PathBuf, anyhow::Error> {
    match std::env::var_os("SUI_CONFIG_DIR") {
        Some(config_env) => Ok(config_env.into()),
        None => match dirs::home_dir() {
            Some(v) => Ok(v.join(SUI_DIR).join(SUI_CONFIG_DIR)),
            None => anyhow::bail!("cannot obtain home directory path"),
        },
    }
    .and_then(|dir| {
        if !dir.exists() {
            fs::create_dir_all(dir.clone())?;
        }
        Ok(dir)
    })
}
