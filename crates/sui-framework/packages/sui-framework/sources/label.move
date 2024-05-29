// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// A library provides implementation for working with labels.
/// Any object which implements the `key` ability can be tagged with labels.
module sui::label {

    use sui::dynamic_field;
    use sui::vec_set::{Self, VecSet};

    /// The `LabelsGuard` dynamic field name.
    const LABELS_NAME: vector<u8> = b"labels";

    /// Sender is not @0x0 the system address.
    const ENotSystemAddress: u64 = 0;

    /// The capability allows to work with system labels.
    public struct SystemLabelerCap has key {
        id: UID,
    }

    /// `LabelsGuard` protects the labels from unauthorized access.
    public struct LabelsGuard has store {
        /// User-defined labels.
        labels: VecSet<vector<u8>>,
        /// Protected system-defined labels.
        system_labels: VecSet<vector<u8>>,
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
    public fun add(self: &mut LabelsGuard, label: vector<u8>) {
        self.labels.insert(label);
    }

    /// Remove a user-defined custom label.
    public fun remove(self: &mut LabelsGuard, label: &vector<u8>) {
        self.labels.remove(label);
    }

    /// Check if an object is labeled with a user-defined custom label.
    public fun has(self: &LabelsGuard, label: &vector<u8>): bool {
        self.labels.contains(label)
    }

    /// Add a system-defined label.
    /// Can by call only by a `SystemLabelerCap` owner.
    public fun add_system(self: &mut LabelsGuard, _: &SystemLabelerCap, label: vector<u8>) {
        self.system_labels.insert(label);
    }

    /// Remove a system-defined label.
    /// Can by call only by a `SystemLabelerCap` owner.
    public fun remove_system(self: &mut LabelsGuard, _: &SystemLabelerCap, label: &vector<u8>) {
        self.system_labels.remove(label);
    }

    /// Check if an object is labeled with a system-defined label.
    public fun has_system(self: &LabelsGuard, label: &vector<u8>): bool {
        self.system_labels.contains(label)
    }

    /// Check if an object is labeled with an any label.
    public fun has_any(self: &LabelsGuard, label: &vector<u8>): bool {
        has(self, label) || has_system(self, label)
    }

    /// Immutably borrow the related labels guard.
    /// A `LabelsGuard` instance will be created if it does not exist.
    public fun borrow_labels_guard(object: &mut UID): &LabelsGuard {
        ensure_labels_guard_is_created(object);

        dynamic_field::borrow<vector<u8>, LabelsGuard>(object, LABELS_NAME)
    }

    /// Mutably borrow the related labels guard.
    /// A `LabelsGuard` instance will be created if it does not exist.
    public fun borrow_labels_guard_mut(object: &mut UID): &mut LabelsGuard {
        ensure_labels_guard_is_created(object);

        dynamic_field::borrow_mut<vector<u8>, LabelsGuard>(object, LABELS_NAME)
    }

    /// Remove the related `LabelsGuard` if it exists.
    /// Must be called when the owned object is deleted to avoid memory leaks.
    public fun remove_labels_guard(object: &mut UID) {
        if (dynamic_field::exists_(object, LABELS_NAME)) {
            let LabelsGuard {
                labels: _,
                system_labels: _,
            } = dynamic_field::remove(object, LABELS_NAME);
        };
    }

    /// Create a `LabelsGuard` instance.
    fun create_labels_guard(): LabelsGuard {
        LabelsGuard {
            labels: vec_set::empty(),
            system_labels: vec_set::empty(),
        }
    }

    /// Create a related `LabelsGuard` instance if it does not exist.
    fun ensure_labels_guard_is_created(object: &mut UID) {
        if (!dynamic_field::exists_(object, LABELS_NAME)) {
            dynamic_field::add(object, LABELS_NAME, create_labels_guard());
        };
    }
}
