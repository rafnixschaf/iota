// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// A timelock implementation.
module timelock::timelock {

    use sui::vec_set::VecSet;

    /// Error code for when the expire timestamp of the lock is in the past.
    const EExpireEpochIsPast: u64 = 0;
    /// Error code for when the lock has not expired yet.
    const ENotExpiredYet: u64 = 1;
    /// Error code for when the sender is not @0x0, the system address.
    const ENotSystemAddress: u64 = 2;
    /// Error code for when the labels collection of the lock is empty.
    const EEmptyLabelsCollection: u64 = 3;
    /// Error code for when the labels collection of the lock contains an empty label.
    const EEmptyLabel: u64 = 4;

    /// The capability allows to work with labels.
    public struct LabelerCap has key {
        id: UID,
    }

    /// `TimeLock` struct that holds a locked object.
    public struct TimeLock<T: store> has key {
        id: UID,
        /// The locked object.
        locked: T,
        /// This is the epoch time stamp of when the lock expires.
        expiration_timestamp_ms: u64,
        /// Timelock related labels.
        labels: Option<VecSet<vector<u8>>>,
    }

    /// Create and transfer a `LabelerCap` object to an authority address.
    /// This function is called exactly once, during genesis.
    public fun assign_labeler_cap(to: address, ctx: &mut TxContext) {
        assert!(ctx.sender() == @0x0, ENotSystemAddress);

        // Create a new capability.
        let cap = LabelerCap {
            id: object::new(ctx),
        };

        // Transfer the capability to the specified address.
        transfer::transfer(cap, to);
    }

    /// Function to lock an object till a unix timestamp in milliseconds.
    public fun lock<T: store>(locked: T, expiration_timestamp_ms: u64, ctx: &mut TxContext): TimeLock<T> {
        // Check that `expiration_timestamp_ms` is valid.
        check_expiration_timestamp_ms(expiration_timestamp_ms, ctx);

        // Create a timelock.
        pack(locked, expiration_timestamp_ms, option::none(), ctx)
    }

    /// Function to lock a labeled object till a unix timestamp in milliseconds.
    public fun lock_labeled<T: store>(
        _: &LabelerCap,
        locked: T,
        expiration_timestamp_ms: u64,
        labels: VecSet<vector<u8>>,
        ctx: &mut TxContext
    ): TimeLock<T> {
        // Check that `expiration_timestamp_ms` is valid.
        check_expiration_timestamp_ms(expiration_timestamp_ms, ctx);

        // Check that the `labels` value is valid.
        assert!(!labels.is_empty(), EEmptyLabelsCollection);
        assert!(!labels.contains(&vector::empty()), EEmptyLabel);

        // Create a labeled timelock.
        pack(locked, expiration_timestamp_ms, option::some(labels), ctx)
    }

    /// Function to unlock the object from a `TimeLock`.
    public fun unlock<T: store>(self: TimeLock<T>, ctx: &TxContext): T {
        // Unpack the timelock. 
        let (locked, expiration_timestamp_ms, labels) = unpack(self);

        // Check if the lock has expired.
        assert!(expiration_timestamp_ms <= ctx.epoch_timestamp_ms(), ENotExpiredYet);

        // Delete the labels.
        if (labels.is_some()) {
            option::destroy_some(labels);
        }
        else {
            option::destroy_none(labels);
        };

        locked
    }

    /// Function to get the expiration timestamp of a `TimeLock`.
    public fun expiration_timestamp_ms<T: store>(self: &TimeLock<T>): u64 {
        self.expiration_timestamp_ms
    }

    /// Function to check if a `TimeLock` is locked.
    public fun is_locked<T: store>(self: &TimeLock<T>, ctx: &TxContext): bool {
        self.remaining_time(ctx) > 0
    }

    /// Function to get the remaining time of a `TimeLock`.
    /// Returns 0 if the lock has expired.
    public fun remaining_time<T: store>(self: &TimeLock<T>, ctx: &TxContext): u64 {
        // Get the epoch timestamp.
        let current_timestamp_ms = ctx.epoch_timestamp_ms();

        // Check if the lock has expired.
        if (self.expiration_timestamp_ms < current_timestamp_ms) {
            return 0
        };

        // Calculate the remaining time.
        self.expiration_timestamp_ms - current_timestamp_ms
    }

    /// Function to get the locked object of a `TimeLock`.
    public fun locked<T: store>(self: &TimeLock<T>): &T {
        &self.locked
    }

    /// Function to get a mutable reference to the locked object of a `TimeLock`.
    public(package) fun locked_mut<T: store>(self: &mut TimeLock<T>): &mut T {
        &mut self.locked
    }

    /// Function to check if a `TimeLock` labeled with a label.
    public fun is_labeled_with<T: store>(self: &TimeLock<T>, label: &vector<u8>): bool {
        // Check if the labels member are initialized, return `false` if it is not.
        if (self.labels.is_some()) {
            return self.labels.borrow().contains(label)
        }
        else {
            return false
        }
    }

    /// Function to get the labels of a `TimeLock`.
    public fun labels<T: store>(self: &TimeLock<T>): &Option<VecSet<vector<u8>>> {
        &self.labels
    }

    /// An utility function to pack a `TimeLock`.
    public(package) fun pack<T: store>(
        locked: T,
        expiration_timestamp_ms: u64,
        labels: Option<VecSet<vector<u8>>>,
        ctx: &mut TxContext): TimeLock<T>
    {
        // Create a timelock.
        TimeLock {
            id: object::new(ctx),
            locked,
            expiration_timestamp_ms,
            labels,
        }
    }

    /// An utility function to unpack a `TimeLock`.
    public(package) fun unpack<T: store>(lock: TimeLock<T>): (T, u64, Option<VecSet<vector<u8>>>) {
        // Unpack the timelock.
        let TimeLock {
            id,
            locked,
            expiration_timestamp_ms,
            labels,
        } = lock;

        // Delete the timelock. 
        object::delete(id);

        (locked, expiration_timestamp_ms, labels)
    }

    /// An utility function to transfer a `TimeLock`.
    public(package) fun transfer<T: store>(lock: TimeLock<T>, recipient: address) {
        transfer::transfer(lock, recipient);
    }

    /// An utility function to check that the `expiration_timestamp_ms` value is valid.
    fun check_expiration_timestamp_ms(expiration_timestamp_ms: u64, ctx: &TxContext) {
        // Get the epoch timestamp.
        let epoch_timestamp_ms = ctx.epoch_timestamp_ms();

        // Check that `expiration_timestamp_ms` is valid.
        assert!(expiration_timestamp_ms > epoch_timestamp_ms, EExpireEpochIsPast);
    }
}
