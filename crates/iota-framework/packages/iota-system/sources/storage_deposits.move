// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

module iota_system::storage_deposits {
    use iota::balance::Balance;
    use iota::iota::IOTA;

    /* friend iota_system::iota_system_state_inner; */

    /// Struct representing the storage deposits, containing one `Balance`:
    /// - `storage_balance` has the invariant that it's the sum of `storage_rebate` of
    ///    all objects currently stored on-chain. To maintain this invariant, the only inflow of this
    ///    balance is storage charges collected from transactions, and the only outflow is storage rebates
    ///    of transactions.
    public struct StorageDeposits has store {
        storage_balance: Balance<IOTA>,
    }

    /// Called by `iota_system` at genesis time.
    public(package) fun new(initial_balance: Balance<IOTA>) : StorageDeposits {
        StorageDeposits {
            // At the beginning there's no object in the storage yet
            storage_balance: initial_balance,
        }
    }

    /// Called by `iota_system` at epoch change times to process the inflows and outflows of storage deposits.
    public(package) fun advance_epoch(
        self: &mut StorageDeposits,
        storage_charges: Balance<IOTA>,
        storage_rebate_amount: u64,
        //TODO: try the way to configure
        _non_refundable_storage_fee_amount: u64,
    ) : Balance<IOTA> {
        // The storage charges for the epoch come from the storage rebate of the new objects created
        // and the new storage rebates of the objects modified during the epoch so we put the charges
        // into `storage_balance`.
        self.storage_balance.join(storage_charges);

        // `storage_rebates` include the already refunded rebates of deleted objects and old rebates of modified objects and
        // should be taken out of the `storage_balance`.
        let storage_rebate = self.storage_balance.split(storage_rebate_amount);

        // The storage rebate has already been returned to individual transaction senders' gas coins
        // so we return the balance to be burnt at the very end of epoch change.
        storage_rebate
    }

    public fun total_balance(self: &StorageDeposits): u64 {
        self.storage_balance.value()
    }
}
