// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

module iota_system::system_display {

    use std::string::{Self, String};

    use iota::display::{Self, Display};

    use iota_system::iota_system::IotaSystemState;

    /// Create an empty `Display` object with `IotaSystemAdminCap`.
    public(package) fun new<T: key>(
        iota_system: &mut IotaSystemState,
        ctx: &mut TxContext
    ): Display<T> {
        // Load the `IotaSystemAdminCap` instance.
        let sys_admin_cap = iota_system.load_iota_system_admin_cap();

        // Create a `Display` object.
        display::system_new<T>(sys_admin_cap, ctx)
    }

    /// Create a new `Display<T>` object with a set of fields using `IotaSystemAdminCap`.
    public(package) fun new_with_fields<T: key>(
        iota_system: &mut IotaSystemState,
        fields: vector<String>,
        values: vector<String>,
        ctx: &mut TxContext
    ): Display<T> {
        // Load the `IotaSystemAdminCap` instance.
        let sys_admin_cap = iota_system.load_iota_system_admin_cap();

        // Create a `Display` object with fields.
        display::system_new_with_fields<T>(sys_admin_cap, fields, values, ctx)
    }

    /// Add a display object to the system state store.
    public(package) fun add_display_object<T: key>(
        iota_system: &mut IotaSystemState,
        display: Display<T>
    ) {
        // Get a display object unique key.
        let key = display_object_key<T>();

        // Store the display object.
        iota_system.add_extra_field(key, display);
    }

    /// Borrow an immutable display object from the system state store.
    public(package) fun borrow_display_object<T: key>(iota_system: &mut IotaSystemState): &Display<T> {
        // Get a display object unique key.
        let key = display_object_key<T>();

        // Borrow the display object.
        iota_system.borrow_extra_field(key)
    }

    /// Borrow a mutable display object from the system state store.
    public(package) fun borrow_display_object_mut<T: key>(iota_system: &mut IotaSystemState): &mut Display<T> {
        // Get a display object unique key.
        let key = display_object_key<T>();

        // Borrow the display object.
        iota_system.borrow_extra_field_mut(key)
    }

    /// Return a fully qualified type name with the original package IDs
    /// that is used as a display object key.
    fun display_object_key<T>(): String {
        string::from_ascii(std::type_name::get_with_original_ids<T>().into_string())
    }
}
