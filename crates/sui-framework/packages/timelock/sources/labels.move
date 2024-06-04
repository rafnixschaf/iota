// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

module timelock::labels {

    use std::string::{Self, String};

    use sui::types;
    use sui::vec_set::{Self, VecSet};

    // === Error codes ===

    /// Error code for when a type passed to the `create_labeler_cap` function is not a one-time witness.
    const ENotOneTimeWitness: u64 = 0;
    /// Error code for when a label already exists in the collection.
    const ELabelAlreadyExists: u64 = 1;

    // === Structs ===

    /// `LabelerCap` allows to insert labels of the specific type `T`.
    /// Can be publicly transferred.
    public struct LabelerCap<phantom T> has key, store {
        id: UID,
    }

    /// `Label` is an internal label representation.
    public struct Label has copy, drop, store {
        /// A fully qualified type name with the original package IDs.
        value: String,
    }

    /// `Labels` is an immutable labels set.
    /// Protects the held labels from being changed and copied.
    public struct Labels has store {
        /// The protected labels collection.
        labels: VecSet<Label>,
    }

    /// `LabelsBuilder` helps to build a `Labels` instance.
    public struct LabelsBuilder {
        /// The labels collection.
        labels: VecSet<Label>,
    }

    // === `LabelerCap` functions ===

    /// Create a `LabelerCap` instance.
    /// Can be created only by consuming a one time witness.
    public fun create_labeler_cap<T: drop>(witness: T, ctx: &mut TxContext): LabelerCap<T> {
        assert!(types::is_one_time_witness(&witness), ENotOneTimeWitness);

        LabelerCap<T> {
            id: object::new(ctx),
        }
    }

    /// Delete a `LabelerCap` instance.
    /// If a capability is destroyed, it is impossible to add the related labels.
    public fun destroy_labeler_cap<T: drop>(cap: LabelerCap<T>) {
        let LabelerCap<T> {
            id,
        } = cap;

        object::delete(id);
    }

    // === `Label` functions ===

    /// Create a `Label` instance.
    /// The created label holds a fully qualified type name with the original package IDs.
    fun from_type<T>(): Label {
        Label {
            value: type_name<T>(),
        }
    }

    /// Return a fully qualified type name with the original package IDs.
    fun type_name<T>(): String {
        string::from_ascii(std::type_name::get_with_original_ids<T>().into_string())
    }

    // === `Labels` functions ===

    /// Return true if `self` has 0 elements, false otherwise.
    public fun is_empty(self: &Labels): bool {
        self.labels.is_empty()
    }

    /// Return true if `self` contains a label, false otherwise.
    public fun contains<T>(self: &Labels): bool {
        self.labels.contains(&from_type<T>())
    }

    /// Destroy a `Labels` instance.
    public fun destroy(self: Labels) {
        let Labels {
            labels: _,
        } = self;
    }

    /// Clone a `Labels` instance.
    /// It is a protected utility function, it should be impossible to clone `Labels`
    /// because it leads to unauthorized labeled objects creation.
    public(package) fun clone(self: &Labels): Labels {
        Labels {
            labels: self.labels,
        }
    }

    // === `LabelsBuilder` functions ===

    /// Create a `LabelsBuilder` instance.
    public fun create_builder(): LabelsBuilder {
        LabelsBuilder {
            labels: vec_set::empty(),
        }
    }

    /// Add a label into `self`. Can be called only by the related `LabelerCap` owner.
    /// Aborts if the label is already present in `self`.
    public fun with_label<T>(mut self: LabelsBuilder, _: &LabelerCap<T>): LabelsBuilder {
        let label = from_type<T>();

        assert!(!self.labels.contains(&label), ELabelAlreadyExists);
    
        self.labels.insert(label);

        self
    }

    /// Transform a `LabelsBuilder` instance into `Labels`.
    public fun into_labels(self: LabelsBuilder): Labels {
        let LabelsBuilder {
            labels,
        } = self;

        Labels {
            labels,
        }
    }
}
