// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//docs::#regulate
module examples::regcoin {
    use iota::coin::{Self, DenyCapV1};
    use iota::deny_list::{DenyList};

    public struct REGCOIN has drop {}

    fun init(witness: REGCOIN, ctx: &mut TxContext) {
        let (treasury, deny_cap, metadata) = coin::create_regulated_currency_v1(
            witness,
            6,
            b"REGCOIN",
            b"",
            b"",
            option::none(),
            false,
            ctx,
        );
        transfer::public_freeze_object(metadata);
        transfer::public_transfer(treasury, ctx.sender());
        transfer::public_transfer(deny_cap, ctx.sender())
    }

    //docs::/#regulate}
    public fun add_addr_from_deny_list(
        denylist: &mut DenyList,
        denycap: &mut DenyCapV1<REGCOIN>,
        denyaddy: address,
        ctx: &mut TxContext,
    ) {
        coin::deny_list_v1_add(denylist, denycap, denyaddy, ctx);
    }

    public fun remove_addr_from_deny_list(
        denylist: &mut DenyList,
        denycap: &mut DenyCapV1<REGCOIN>,
        denyaddy: address,
        ctx: &mut TxContext,
    ) {
        coin::deny_list_v1_remove(denylist, denycap, denyaddy, ctx);
    }
}
