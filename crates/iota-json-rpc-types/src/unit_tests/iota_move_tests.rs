// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_enum_compat_util::*;

use crate::{IOTAMoveStruct, IOTAMoveValue};

#[test]
fn enforce_order_test() {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(["tests", "staged", "iota_move_struct.yaml"]);
    check_enum_compat_order::<IOTAMoveStruct>(path);

    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(["tests", "staged", "iota_move_value.yaml"]);
    check_enum_compat_order::<IOTAMoveValue>(path);
}
