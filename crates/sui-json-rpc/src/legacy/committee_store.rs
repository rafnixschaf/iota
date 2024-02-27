// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use rocksdb::Options;

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use sui_types::{
    committee::{Committee, EpochId},
    error::SuiResult,
};

pub struct CommitteeStore {}

impl CommitteeStore {
    pub fn new(
        _path: PathBuf,
        _genesis_committee: &Committee,
        _db_options: Option<Options>,
    ) -> Self {
        Self {}
    }

    pub fn new_for_testing(_genesis_committee: &Committee) -> Self {
        Self {}
    }

    pub fn init_genesis_committee(&self, _genesis_committee: Committee) -> SuiResult {
        unimplemented!()
    }

    pub fn insert_new_committee(&self, _new_committee: &Committee) -> SuiResult {
        unimplemented!()
    }

    pub fn get_committee(&self, _epoch_id: &EpochId) -> SuiResult<Option<Arc<Committee>>> {
        unimplemented!()
    }

    // todo - make use of cache or remove this method
    pub fn get_latest_committee(&self) -> Committee {
        unimplemented!()
    }
    /// Return the committee specified by `epoch`. If `epoch` is `None`, return the latest committee.
    // todo - make use of cache or remove this method
    pub fn get_or_latest_committee(&self, _epoch: Option<EpochId>) -> SuiResult<Committee> {
        unimplemented!()
    }

    pub fn checkpoint_db(&self, _path: &Path) -> SuiResult {
        unimplemented!()
    }
}
