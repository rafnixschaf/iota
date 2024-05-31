// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

module timelock::timelocked_staked_iota {

    use iota_system::staking_pool::StakedIOTA;

    const EIncompatibleTimelockedStakedIOTA: u64 = 0;

    /// A self-custodial object holding the timelocked staked IOTA tokens.
    public struct TimelockedStakedIOTA has key {
        id: UID,
        /// A self-custodial object holding the staked IOTA tokens.
        staked_iota: StakedIOTA,
        /// This is the epoch time stamp of when the lock expires.
        expiration_timestamp_ms: u64,
    }

    /// Create a new instance of `TimelockedStakedIOTA`.
    public(package) fun create(
        staked_iota: StakedIOTA,
        expiration_timestamp_ms: u64,
        ctx: &mut TxContext
    ): TimelockedStakedIOTA {
        TimelockedStakedIOTA {
            id: object::new(ctx),
            staked_iota,
            expiration_timestamp_ms
        }
    }

    /// Function to get the pool id of a `TimelockedStakedIOTA`.
    public fun pool_id(self: &TimelockedStakedIOTA): ID { self.staked_iota.pool_id() }

    /// Function to get the staked iota amount of a `TimelockedStakedIOTA`.
    public fun staked_iota_amount(self: &TimelockedStakedIOTA): u64 { self.staked_iota.staked_iota_amount() }

    /// Allows calling `.amount()` on `TimelockedStakedIOTA` to invoke `staked_iota_amount`
    public use fun staked_iota_amount as TimelockedStakedIOTA.amount;

    /// Function to get the stake activation epoch of a `TimelockedStakedIOTA`.
    public fun stake_activation_epoch(self: &TimelockedStakedIOTA): u64 {
        self.staked_iota.stake_activation_epoch()
    }

    /// Function to get the expiration timestamp of a `TimelockedStakedIOTA`.
    public fun expiration_timestamp_ms(self: &TimelockedStakedIOTA): u64 {
        self.expiration_timestamp_ms
    }

    /// Split `TimelockedStakedIOTA` into two parts, one with principal `split_amount`,
    /// and the remaining principal is left in `self`.
    /// All the other parameters of the `TimelockedStakedIOTA` like `stake_activation_epoch` or `pool_id` remain the same.
    public fun split(self: &mut TimelockedStakedIOTA, split_amount: u64, ctx: &mut TxContext): TimelockedStakedIOTA {
        let splitted_stake = self.staked_iota.split(split_amount, ctx);

        TimelockedStakedIOTA {
            id: object::new(ctx),
            staked_iota: splitted_stake,
            expiration_timestamp_ms: self.expiration_timestamp_ms,
        }
    }

    /// Split the given `TimelockedStakedIOTA` to the two parts, one with principal `split_amount`,
    /// transfer the newly split part to the sender address.
    public entry fun split_staked_iota(stake: &mut TimelockedStakedIOTA, split_amount: u64, ctx: &mut TxContext) {
        transfer::transfer(split(stake, split_amount, ctx), ctx.sender());
    }

    /// Allows calling `.split_to_sender()` on `TimelockedStakedIOTA` to invoke `split_staked_iota`
    public use fun split_staked_iota as TimelockedStakedIOTA.split_to_sender;

    /// Consume the staked iota `other` and add its value to `self`.
    /// Aborts if some of the staking parameters are incompatible (pool id, stake activation epoch, etc.)
    public entry fun join_staked_iota(self: &mut TimelockedStakedIOTA, other: TimelockedStakedIOTA) {
        assert!(self.is_equal_staking_metadata(&other), EIncompatibleTimelockedStakedIOTA);

        let TimelockedStakedIOTA {
            id,
            staked_iota,
            expiration_timestamp_ms: _,
        } = other;

        id.delete();

        self.staked_iota.join(staked_iota);
    }

    /// Allows calling `.join()` on `TimelockedStakedIOTA` to invoke `join_staked_iota`
    public use fun join_staked_iota as TimelockedStakedIOTA.join;

    /// Returns true if all the staking parameters of the staked iota except the principal are identical
    public fun is_equal_staking_metadata(self: &TimelockedStakedIOTA, other: &TimelockedStakedIOTA): bool {
        self.staked_iota.is_equal_staking_metadata(&other.staked_iota) &&
        (self.expiration_timestamp_ms == other.expiration_timestamp_ms)
    }

    /// An utility function to destroy a `TimelockedStakedIOTA`.
    public(package) fun unpack(self: TimelockedStakedIOTA): (StakedIOTA, u64) {
        let TimelockedStakedIOTA {
            id,
            staked_iota,
            expiration_timestamp_ms,
        } = self;

        object::delete(id);

        (staked_iota, expiration_timestamp_ms)
    }

    /// An utility function to transfer a `TimelockedStakedIOTA`.
    public(package) fun transfer(stake: TimelockedStakedIOTA, recipient: address) {
        transfer::transfer(stake, recipient);
    }
}
