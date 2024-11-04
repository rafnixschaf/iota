// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

module iota_system::iota_system_display {

    use std::string::String;

    use iota::display::{Self, Display};

    use iota_system::iota_system::IotaSystemState;

    /// Create an empty `Display` object with `SystemDisplayCap`.
    public(package) fun system_new<T: key>(
        iota_system: &IotaSystemState,
        ctx: &mut TxContext
    ): Display<T> {
        // Load the `SystemDisplayCap` instance.
        let sys_display_cap = iota_system.load_system_display_cap();

        // Create a `Display` object.
        display::system_new<T>(sys_display_cap, ctx)
    }

    /// Create a new Display<T> object with a set of fields using `SystemDisplayCap`.
    public(package) fun system_new_with_fields<T: key>(
        iota_system: &IotaSystemState,
        fields: vector<String>,
        values: vector<String>,
        ctx: &mut TxContext
    ): Display<T> {
        // Load the `SystemDisplayCap` instance.
        let sys_display_cap = iota_system.load_system_display_cap();

        // Create a `Display` object with fields.
        display::system_new_with_fields<T>(sys_display_cap, fields, values, ctx)
    }
}
