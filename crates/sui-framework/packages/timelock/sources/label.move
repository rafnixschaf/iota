// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

module timelock::label {

    use std::string::{Self, String};

    use sui::types;

    // === Error codes ===

    /// Error code for when a type passed to the `create_labeler_cap` function is not a one-time witness.
    const ENotOneTimeWitness: u64 = 0;

    // === Structs ===

    /// `LabelerCap` allows to create labels of the specific type `L`.
    /// Can be publicly transferred.
    public struct LabelerCap<phantom L> has key, store {
        id: UID,
    }

    /// `Label` is an immutable label representation.
    public struct Label has store {
        /// A fully qualified type name with the original package IDs.
        value: String,
    }
    // === `LabelerCap` functions ===

    /// Create a `LabelerCap` instance.
    /// Can be created only by consuming a one time witness.
    public fun create_labeler_cap<L: drop>(witness: L, ctx: &mut TxContext): LabelerCap<L> {
        assert!(types::is_one_time_witness(&witness), ENotOneTimeWitness);

        LabelerCap<L> {
            id: object::new(ctx),
        }
    }

    /// Delete a `LabelerCap` instance.
    /// If a capability is destroyed, it is impossible to add the related labels.
    public fun destroy_labeler_cap<L: drop>(cap: LabelerCap<L>) {
        let LabelerCap<L> {
            id,
        } = cap;

        object::delete(id);
    }

    // === `Label` functions ===

    /// Create a `Label` instance.
    /// The created label holds a fully qualified type name with the original package IDs.
    /// Can be called only by the related `LabelerCap` owner.
    public fun from_type<L>(_: &LabelerCap<L>): Label {
        Label {
            value: type_name<L>(),
        }
    }

    /// Check if a `Label` represents the type `L`.
    public fun is_type<L>(self: &Label): bool {
        self.value == type_name<L>()
    }

    /// Destroy a `Label` instance.
    public fun destroy(self: Label) {
        let Label {
            value: _,
        } = self;
    }

    /// Destroy an optional `Label` instance.
    public fun destroy_opt(self: Option<Label>) {
        if (self.is_some()) {
            destroy(option::destroy_some(self));
        }
        else {
            option::destroy_none(self);
        };
    }

    /// Return a fully qualified type name with the original package IDs.
    fun type_name<L>(): String {
        string::from_ascii(std::type_name::get_with_original_ids<L>().into_string())
    }

    /// Clone a `Label` instance.
    /// It is a protected utility function, it should be impossible to clone `Label`
    /// because it leads to unauthorized labeled objects creation.
    public(package) fun clone(self: &Label): Label {
        Label {
            value: self.value,
        }
    }

    /// Clone an optional `Label` instance.
    /// It is a protected utility function, it should be impossible to clone `Label`
    /// because it leads to unauthorized labeled objects creation.
    public(package) fun clone_opt(self: &Option<Label>): Option<Label> {
        if (self.is_some()) {
            option::some(clone(self.borrow()))
        }
        else {
            option::none()
        }
    }

    #[test_only]
    public fun value(self: &Label): &String {
        &self.value
    }
}
