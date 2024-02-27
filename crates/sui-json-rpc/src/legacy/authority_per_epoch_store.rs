// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use move_binary_format::CompiledModule;
use move_bytecode_utils::module_cache::GetModule;
use move_core_types::language_storage::ModuleId;
use sui_execution::Executor;
use sui_protocol_config::ProtocolConfig;
use sui_types::error::SuiError;

pub struct ModuleCache {}

impl GetModule for ModuleCache {
    type Error = SuiError;
    type Item = CompiledModule;

    fn get_module_by_id(&self, _id: &ModuleId) -> Result<Option<Self::Item>, Self::Error> {
        unimplemented!()
    }
}

pub struct AuthorityPerEpochStore {}

impl AuthorityPerEpochStore {
    pub fn executor(&self) -> &Arc<dyn Executor + Send + Sync> {
        unimplemented!()
    }

    pub fn module_cache(&self) -> &Arc<ModuleCache> {
        unimplemented!()
    }

    pub fn protocol_config(&self) -> &ProtocolConfig {
        unimplemented!()
    }

    pub fn reference_gas_price(&self) -> u64 {
        unimplemented!()
    }
}
