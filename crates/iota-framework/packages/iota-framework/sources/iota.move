// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Coin<IOTA> is the token used to pay for gas in Iota.
/// It has 9 decimals, and the smallest unit (10^-9) is called "micros".
module iota::iota {
    use iota::balance::Balance;
    use iota::coin::{Self, Coin, TreasuryCap};

    const EAlreadyMinted: u64 = 0;
    /// Sender is not @0x0 the system address.
    const ENotSystemAddress: u64 = 1;

    #[allow(unused_const)]
    /// The amount of Micros per Iota token based on the fact that micros is
    /// 10^-9 of a Iota token
    const MICROS_PER_IOTA: u64 = 1_000_000_000;

    #[allow(unused_const)]
    /// The total supply of Iota denominated in whole Iota tokens (10 Billion)
    const TOTAL_SUPPLY_IOTA: u64 = 10_000_000_000;

    #[allow(unused_const)]
    /// The total supply of Iota denominated in Micros (10 Billion * 10^9)
    const TOTAL_SUPPLY_MICROS: u64 = 10_000_000_000_000_000_000;

    /// Name of the coin
    public struct IOTA has drop {}

    /// The IOTA token treasury capability.
    /// Protects the token from unauthorized changes.
    public struct IotaTreasuryCap has store {
        inner: TreasuryCap<IOTA>,
    }

    #[allow(unused_function)]
    /// Register the `IOTA` Coin to acquire `IotaTreasuryCap`.
    /// This should be called only once during genesis creation.
    fun new(ctx: &mut TxContext): IotaTreasuryCap {
        assert!(ctx.sender() == @0x0, ENotSystemAddress);
        assert!(ctx.epoch() == 0, EAlreadyMinted);

        let (treasury, metadata) = coin::create_currency(
            IOTA {},
            9,
            b"IOTA",
            b"Iota",
            // TODO: add appropriate description and logo url
            b"",
            option::none(),
            ctx
        );

        transfer::public_freeze_object(metadata);

        IotaTreasuryCap {
            inner: treasury,
        }
    }

    public entry fun transfer(c: coin::Coin<IOTA>, recipient: address) {
        transfer::public_transfer(c, recipient)
    }

    /// Create an IOTA coin worth `value` and increase the total supply in `cap` accordingly.
    public fun mint(cap: &mut IotaTreasuryCap, value: u64, ctx: &mut TxContext): Coin<IOTA> {
        assert!(ctx.sender() == @0x0, ENotSystemAddress);

        cap.inner.mint(value, ctx)
    }

    /// Mint some amount of IOTA as a `Balance` and increase the total supply in `cap` accordingly.
    /// Aborts if `value` + `cap.inner.total_supply` >= U64_MAX
    public fun mint_balance(cap: &mut IotaTreasuryCap, value: u64, ctx: &TxContext): Balance<IOTA> {
        assert!(ctx.sender() == @0x0, ENotSystemAddress);

        cap.inner.mint_balance(value)
    }

    /// Destroy the IOTA coin `c` and decrease the total supply in `cap` accordingly.
    public fun burn(cap: &mut IotaTreasuryCap, c: Coin<IOTA>, ctx: &TxContext): u64 {
        assert!(ctx.sender() == @0x0, ENotSystemAddress);

        cap.inner.burn(c)
    }

    /// Destroy the IOTA balance `b` and decrease the total supply in `cap` accordingly.
    public fun burn_balance(cap: &mut IotaTreasuryCap, b: Balance<IOTA>, ctx: &TxContext): u64 {
        assert!(ctx.sender() == @0x0, ENotSystemAddress);

        cap.inner.supply_mut().decrease_supply(b)
    }

    /// Return the total number of IOTA's in circulation.
    public fun total_supply(cap: &IotaTreasuryCap): u64 {
        cap.inner.total_supply()
    }

    #[allow(unused_function)]
    /// Increase the IOTA supply.
    /// This should be called only once during genesis creation.
    fun mint_genesis_supply(cap: &mut IotaTreasuryCap, value: u64, ctx: &TxContext): Balance<IOTA> {
        assert!(ctx.epoch() == 0, EAlreadyMinted);

        cap.mint_balance(value, ctx)
    }

    #[test_only]
    public fun create_for_testing(ctx: &mut TxContext): IotaTreasuryCap {
        // The `new` function must be called here to be sure that the test function
        // contains all the important checks.
        new(ctx)
    }
}
