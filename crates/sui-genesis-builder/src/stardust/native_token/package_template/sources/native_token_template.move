// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module 0x0::$MODULE_NAME {
    use sui::coin;
    use sui::transfer;
    use sui::tx_context::{Self, TxContext};
    use sui::url::Url;
    use stardust::capped_coin;

    /// The type identifier of coin. The coin will have a type
    /// tag of kind: `Coin<package_object::$MODULE_NAME::$OTW`
    /// Make sure that the name of the type matches the module's name.
    public struct $OTW has drop {}

    /// Module initializer is called once on module publish. A treasury
    /// cap is sent to the publisher, who then controls minting and burning
    fun init(witness: $OTW, ctx: &mut TxContext) {
        let icon_url = $ICON_URL;

        // Create the currency
        let (treasury_cap, metadata) = coin::create_currency<$OTW>(
            witness,
            $COIN_DECIMALS,
            b"$COIN_SYMBOL",
            b"$COIN_NAME",
            b"$COIN_DESCRIPTION",
            icon_url,
            ctx
        );

        // Create the max supply policy
        let policy = capped_coin::create_max_supply_policy(treasury_cap, $MAXIMUM_SUPPLY, ctx);

        // Freeze the coin metadata
        transfer::public_freeze_object(metadata);

        // Transfer the policy as a cap to the alias address
        transfer::public_transfer(policy, @alias);
    }

}