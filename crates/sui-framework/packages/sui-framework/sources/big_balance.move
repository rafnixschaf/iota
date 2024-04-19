// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// A storable handler for Balances in general. Is used in the `Coin`
/// module to allow balance operations and can be used to implement
/// custom coins with `Supply` and `Balance`s.
module sui::big_balance {

    /// Allows calling `.into_coin()` on a `Balance` to turn it into a coin.
    public use fun sui::big_coin::from_balance as BigBalance.into_coin;

    /* friend sui::sui; */

    /// For when trying to destroy a non-zero balance.
    const ENonZero: u64 = 0;
    /// For when an overflow is happening on Supply operations.
    const EOverflow: u64 = 1;
    /// For when trying to withdraw more than there is.
    const ENotEnough: u64 = 2;

    /// A Supply of T. Used for minting and burning.
    /// Wrapped into a `TreasuryCap` in the `Coin` module.
    public struct BigSupply<phantom T> has store {
        value: u256
    }

    /// Storable balance - an inner struct of a Coin type.
    /// Can be used to store coins which don't need the key ability.
    public struct BigBalance<phantom T> has store {
        value: u256
    }

    /// Get the amount stored in a `Balance`.
    public fun value<T>(self: &BigBalance<T>): u256 {
        self.value
    }

    /// Get the `Supply` value.
    public fun supply_value<T>(supply: &BigSupply<T>): u256 {
        supply.value
    }

    /// Create a new supply for type T.
    public fun create_supply<T: drop>(_: T): BigSupply<T> {
        BigSupply { value: 0 }
    }

    /// Increase supply by `value` and create a new `BigBalance<T>` with this value.
    public fun increase_supply<T>(self: &mut BigSupply<T>, value: u256): BigBalance<T> {
        assert!(value < (115792089237316195423570985008687907853269984665640564039457584007913129639935u256 - self.value), EOverflow);
        self.value = self.value + value;
        BigBalance { value }
    }

    /// Burn a BigBalance<T> and decrease BigSupply<T>.
    public fun decrease_supply<T>(self: &mut BigSupply<T>, balance: BigBalance<T>): u256 {
        let BigBalance { value } = balance;
        assert!(self.value >= value, EOverflow);
        self.value = self.value - value;
        value
    }

    /// Create a zero `BigBalance` for type `T`.
    public fun zero<T>(): BigBalance<T> {
        BigBalance { value: 0 }
    }

    /// Join two balances together.
    public fun join<T>(self: &mut BigBalance<T>, balance: BigBalance<T>): u256 {
        let BigBalance { value } = balance;
        self.value = self.value + value;
        self.value
    }

    /// Split a `BigBalance` and take a sub balance from it.
    public fun split<T>(self: &mut BigBalance<T>, value: u256): BigBalance<T> {
        assert!(self.value >= value, ENotEnough);
        self.value = self.value - value;
        BigBalance { value }
    }

    /// Withdraw all balance. After this the remaining balance must be 0.
    public fun withdraw_all<T>(self: &mut BigBalance<T>): BigBalance<T> {
        let value = self.value;
        split(self, value)
    }

    /// Destroy a zero `BigBalance`.
    public fun destroy_zero<T>(balance: BigBalance<T>) {
        assert!(balance.value == 0, ENonZero);
        let BigBalance { value: _ } = balance;
    }

    /// Destroy a `Supply` preventing any further minting and burning.
    public(package) fun destroy_supply<T>(self: BigSupply<T>): u256 {
        let BigSupply { value } = self;
        value
    }

    #[test_only]
    /// Create a `Balance` of any coin for testing purposes.
    public fun create_for_testing<T>(value: u256): BigBalance<T> {
        BigBalance { value }
    }

    #[test_only]
    /// Destroy a `Balance` of any coin for testing purposes.
    public fun destroy_for_testing<T>(self: BigBalance<T>): u256 {
        let BigBalance { value } = self;
        value
    }

    #[test_only]
    /// Create a `Supply` of any coin for testing purposes.
    public fun create_supply_for_testing<T>(): BigSupply<T> {
        BigSupply { value: 0 }
    }
}

#[test_only]
module sui::big_balance_tests {
    use sui::big_balance;
    use sui::sui::SUI;
    use sui::test_utils;

    #[test]
    fun test_big_balance() {
        let mut balance = big_balance::zero<SUI>();
        let another = big_balance::create_for_testing(1000);

        balance.join(another);

        assert!(balance.value() == 1000, 0);

        let balance1 = balance.split(333);
        let balance2 = balance.split(333);
        let balance3 = balance.split(334);

        balance.destroy_zero();

        assert!(balance1.value() == 333, 1);
        assert!(balance2.value() == 333, 2);
        assert!(balance3.value() == 334, 3);

        test_utils::destroy(balance1);
        test_utils::destroy(balance2);
        test_utils::destroy(balance3);
    }
}
