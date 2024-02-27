// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{collections::HashMap, sync::Arc};

use sui_types::{base_types::SuiAddress, error::SuiResult, TypeTag};

#[derive(Default, Copy, Clone, Debug, Eq, PartialEq)]
pub struct TotalBalance {
    pub balance: i128,
    pub num_coins: i64,
}

/// IndexStore supports creation of various ancillary indexes of state in `SuiDataStore`.
pub struct IndexStore {}

impl IndexStore {
    /// This method first gets the balance from `per_coin_type_balance` cache. On a cache miss, it
    /// gets the balance for passed in `coin_type` from the `all_balance` cache. Only on the second
    /// cache miss, we go to the database (expensive) and update the cache. Notice that db read is
    /// done with `spawn_blocking` as that is expected to block
    pub async fn get_balance(
        &self,
        _owner: SuiAddress,
        _coin_type: TypeTag,
    ) -> SuiResult<TotalBalance> {
        unimplemented!()
    }

    /// This method gets the balance for all coin types from the `all_balance` cache. On a cache miss,
    /// we go to the database (expensive) and update the cache. This cache is dual purpose in the
    /// sense that it not only serves `get_AllBalance()` calls but is also used for serving
    /// `get_Balance()` queries. Notice that db read is performed with `spawn_blocking` as that is
    /// expected to block
    pub async fn get_all_balance(
        &self,
        _owner: SuiAddress,
    ) -> SuiResult<Arc<HashMap<TypeTag, TotalBalance>>> {
        unimplemented!()
    }
}
