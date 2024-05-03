// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[allow(unused_const)]
module sui_system::timelocked_staked_sui {
    use sui::balance::Balance;
    use sui::sui::SUI;

    /// TimelockedStakedSui objects cannot be split to below this amount.
    const MIN_STAKING_THRESHOLD: u64 = 1_000_000_000; // 1 SUI

    const EInsufficientSuiTokenBalance: u64 = 3;
    const EIncompatibleStakedSui: u64 = 12;
    const EStakedSuiBelowThreshold: u64 = 18;

    /// A self-custodial object holding the timelocked staked SUI tokens.
    public struct TimelockedStakedSui has key, store {
        id: UID,
        /// ID of the staking pool we are staking with.
        pool_id: ID,
        /// The epoch at which the stake becomes active.
        stake_activation_epoch: u64,
        /// The staked SUI tokens.
        principal: Balance<SUI>,
        /// This is the epoch time stamp of when the lock expires.
        expire_timestamp_ms: u64,
    }

    public(package) fun create(
        pool_id: ID,
        stake_activation_epoch: u64,
        principal: Balance<SUI>,
        expire_timestamp_ms: u64,
        ctx: &mut TxContext
    ): TimelockedStakedSui {
        TimelockedStakedSui {
            id: object::new(ctx),
            pool_id,
            stake_activation_epoch,
            principal,
            expire_timestamp_ms
        }
    }

    public(package) fun unwrap_timelocked_staked_sui(staked_sui: TimelockedStakedSui): (ID, u64, Balance<SUI>, u64) {
        let TimelockedStakedSui {
            id,
            pool_id,
            stake_activation_epoch,
            principal,
            expire_timestamp_ms,
        } = staked_sui;
        object::delete(id);
        (pool_id, stake_activation_epoch, principal, expire_timestamp_ms)
    }

    // public fun into_balance(staked_sui: TimelockedStakedSui): Balance<SUI> {
    //     let (_, _, principal, _) = unwrap_timelocked_staked_sui(staked_sui);
    //     principal
    // }

    // ==== getters and misc utility functions ====

    public fun pool_id(staked_sui: &TimelockedStakedSui): ID { staked_sui.pool_id }

    public fun staked_sui_amount(staked_sui: &TimelockedStakedSui): u64 { staked_sui.principal.value() }

    /// Allows calling `.amount()` on `StakedSui` to invoke `staked_sui_amount`
    public use fun staked_sui_amount as TimelockedStakedSui.amount;

    public fun stake_activation_epoch(staked_sui: &TimelockedStakedSui): u64 {
        staked_sui.stake_activation_epoch
    }

    public fun expire_timestamp_ms(staked_sui: &TimelockedStakedSui): u64 {
        staked_sui.expire_timestamp_ms
    }

    /// Split TimelockedStakedSui `self` to two parts, one with principal `split_amount`,
    /// and the remaining principal is left in `self`.
    /// All the other parameters of the TimelockedStakedSui like `stake_activation_epoch` or `pool_id` remain the same.
    public fun split(self: &mut TimelockedStakedSui, split_amount: u64, ctx: &mut TxContext): TimelockedStakedSui {
        let original_amount = self.principal.value();
        assert!(split_amount <= original_amount, EInsufficientSuiTokenBalance);
        let remaining_amount = original_amount - split_amount;
        // Both resulting parts should have at least MIN_STAKING_THRESHOLD.
        assert!(remaining_amount >= MIN_STAKING_THRESHOLD, EStakedSuiBelowThreshold);
        assert!(split_amount >= MIN_STAKING_THRESHOLD, EStakedSuiBelowThreshold);
        TimelockedStakedSui {
            id: object::new(ctx),
            pool_id: self.pool_id,
            stake_activation_epoch: self.stake_activation_epoch,
            principal: self.principal.split(split_amount),
            expire_timestamp_ms: self.expire_timestamp_ms,
        }
    }

    /// Split the given TimelockedStakedSui to the two parts, one with principal `split_amount`,
    /// transfer the newly split part to the sender address.
    public entry fun split_staked_sui(stake: &mut TimelockedStakedSui, split_amount: u64, ctx: &mut TxContext) {
        transfer::transfer(split(stake, split_amount, ctx), ctx.sender());
    }

    /// Allows calling `.split_to_sender()` on `StakedSui` to invoke `split_staked_sui`
    public use fun split_staked_sui as TimelockedStakedSui.split_to_sender;

    /// Consume the staked sui `other` and add its value to `self`.
    /// Aborts if some of the staking parameters are incompatible (pool id, stake activation epoch, etc.)
    public entry fun join_staked_sui(self: &mut TimelockedStakedSui, other: TimelockedStakedSui) {
        assert!(is_equal_staking_metadata(self, &other), EIncompatibleStakedSui);
        let TimelockedStakedSui {
            id,
            pool_id: _,
            stake_activation_epoch: _,
            principal,
            expire_timestamp_ms: _,
        } = other;

        id.delete();
        self.principal.join(principal);
    }

    /// Allows calling `.join()` on `TimelockedStakedSui` to invoke `join_staked_sui`
    public use fun join_staked_sui as TimelockedStakedSui.join;

    /// Returns true if all the staking parameters of the staked sui except the principal are identical
    public fun is_equal_staking_metadata(self: &TimelockedStakedSui, other: &TimelockedStakedSui): bool {
        (self.pool_id == other.pool_id) &&
        (self.stake_activation_epoch == other.stake_activation_epoch) &&
        (self.expire_timestamp_ms == other.expire_timestamp_ms)
    }
}
