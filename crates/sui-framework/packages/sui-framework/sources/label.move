// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// A library provides implementation for working with labels.
/// Any object which implements the `key` ability can be tagged with labels.
module sui::label {

    use std::string::String;

    use sui::dynamic_field;
    use sui::vec_set::{Self, VecSet};

    /// The user-defined custom labels dynamic field name.
    const LABELS_NAME: vector<u8> = b"labels";
    /// The system-defined labels dynamic field name.
    const SYSTEM_LABELS_NAME: vector<u8> = b"system_labels";

    /// Sender is not @0x0 the system address.
    const ENotSystemAddress: u64 = 0;

    /// The capability allows to work with system labels.
    public struct SystemLabelerCap has key {
        id: UID,
    }

    /// Create and transfer a `SystemLabelerCap` object to an address.
    /// This function is called exactly once, during genesis.
    public fun assign_system_labeler_cap(to: address, ctx: &mut TxContext) {
        assert!(ctx.sender() == @0x0, ENotSystemAddress);

        // Create a new capability.
        let cap = SystemLabelerCap {
            id: object::new(ctx),
        };

        // Transfer the capability to the specified address.
        transfer::transfer(cap, to);
    }

    /// Add a user-defined custom label.
    public fun add(object: &mut UID, label: String) {
        add_impl(object, LABELS_NAME, label);
    }

    /// Add a system-defined label.
    /// Can by call only by a `SystemLabelerCap` owner.
    public fun add_system(_: &SystemLabelerCap, object: &mut UID, label: String) {
        add_impl(object, SYSTEM_LABELS_NAME, label);
    }

    /// Remove a user-defined custom label.
    public fun remove(object: &mut UID, label: &String) {
        remove_impl(object, LABELS_NAME, label);
    }

    /// Remove a system-defined label.
    /// Can by call only by a `SystemLabelerCap` owner.
    public fun remove_system(_: &SystemLabelerCap, object: &mut UID, label: &String) {
        remove_impl(object, SYSTEM_LABELS_NAME, label);
    }

    /// Check if an object is tagged with a user-defined custom label.
    public fun has(object: &mut UID, label: &String): bool {
        has_impl(object, LABELS_NAME, label)
    }

    /// Check if an object is tagged with a system-defined label.
    public fun has_system(object: &mut UID, label: &String): bool {
        has_impl(object, SYSTEM_LABELS_NAME, label)
    }

    /// Check if an object is tagged with an any label.
    public fun has_any(object: &mut UID, label: &String): bool {
        has_impl(object, LABELS_NAME, label) || has_impl(object, SYSTEM_LABELS_NAME, label)
    }

    /// Add label internal utility function.
    fun add_impl(object: &mut UID, df_name: vector<u8>, label: String) {
        // Check if a labels collection exists.
        if (dynamic_field::exists_(object, df_name)) {
            // Borrow the related labels collection.
            let labels = dynamic_field::borrow_mut<vector<u8>, VecSet<String>>(object, df_name);

            // Insert the label into the collection.
            labels.insert(label);
        } else {
            // Create a new labels collection.
            let mut labels = vec_set::empty();

            // Insert the label into the collection.
            labels.insert(label);

            // Add the created collection as a dynamic field to the object.
            dynamic_field::add(object, df_name, labels);
        }
    }

    /// Remove label internal utility function.
    fun remove_impl(object: &mut UID, df_name: vector<u8>, label: &String) {
        // Need to check if this variable is required.
        let mut need_remove_collection = false;

        // Check if a labels collection exists.
        if (dynamic_field::exists_(object, df_name)) {
            // Borrow the related labels collection.
            let labels = dynamic_field::borrow_mut<vector<u8>, VecSet<String>>(object, df_name);

            // Check if the labels collection contains the label.
            if (labels.contains(label)) {
                // Remove the label.
                labels.remove(label);

                // Remove the labels collection if it is empty.
                if (labels.is_empty()) {
                    need_remove_collection = true;
                };
            };
        };

        // Remove the related labels collection.
        if (need_remove_collection) {
            dynamic_field::remove<vector<u8>, VecSet<String>>(object, df_name);
        }
    }

    /// Utility function for checking if an object is tagged with a label.
    fun has_impl(object: &UID, df_name: vector<u8>, label: &String): bool {
        // The label can not exist if there is no a label collection.
        if (!dynamic_field::exists_(object, df_name)) {
            return false
        };

        // Borrow the related labels collection.
        let labels = dynamic_field::borrow<vector<u8>, VecSet<String>>(object, df_name);

        // Check if an object is tagged with a label.
        labels.contains(label)
    }
}
