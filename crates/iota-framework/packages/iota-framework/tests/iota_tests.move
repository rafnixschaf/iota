// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module iota::iota_tests {

    use iota::iota;
    use iota::test_scenario;
    use iota::test_utils::{Self, assert_eq};

    #[test]
    fun test_mint_burn_flow() {
        // Set up a test enviroment.
        let mut scenario = test_scenario::begin(@0x0);
        let ctx = scenario.ctx();

        // Create an IOTA treasury capability.
        let mut iota_treasury_cap = iota::create_for_testing(ctx);

        // Mint some IOTA.
        let iota_coin = iota_treasury_cap.mint(100, ctx);

        assert_eq(iota_treasury_cap.total_supply(), 100);

        let iota_balance = iota_treasury_cap.mint_balance(200, ctx);

        assert_eq(iota_treasury_cap.total_supply(), 300);

        // Burn some IOTA.
        iota_treasury_cap.burn(iota_coin, ctx);

        assert_eq(iota_treasury_cap.total_supply(), 200);

        iota_treasury_cap.burn_balance(iota_balance, ctx);

        assert_eq(iota_treasury_cap.total_supply(), 0);

        // Cleanup.
        test_utils::destroy(iota_treasury_cap);
    
        scenario.end();
    }

    #[test]
    #[expected_failure(abort_code = iota::ENotSystemAddress)]
    fun test_mint_coins_by_custom_address() {
        // Set up a test enviroment.
        let mut scenario = test_scenario::begin(@0xA);

        // Create an IOTA treasury capability.
        let mut iota_treasury_cap = iota::create_for_testing(scenario.ctx());

        // Mint some IOTA coins.
        let iota = iota_treasury_cap.mint(100, scenario.ctx());

        // Cleanup.
        test_utils::destroy(iota);
        test_utils::destroy(iota_treasury_cap);

        scenario.end();
    }

    #[test]
    #[expected_failure(abort_code = iota::ENotSystemAddress)]
    fun test_burn_coins_by_custom_address() {
        // Set up a test enviroment.
        let mut scenario = test_scenario::begin(@0x0);

        // Create an IOTA treasury capability.
        let mut iota_treasury_cap = iota::create_for_testing(scenario.ctx());

        // Mint some IOTA coins.
        let iota = iota_treasury_cap.mint(100, scenario.ctx());

        assert_eq(iota_treasury_cap.total_supply(), 100);

        // Switch to a custom address.
        scenario.next_tx(@0xA);

        // Burn some IOTA coins.
        iota_treasury_cap.burn(iota, scenario.ctx());

        // Cleanup.
        test_utils::destroy(iota_treasury_cap);

        scenario.end();
    }

    #[test]
    #[expected_failure(abort_code = iota::ENotSystemAddress)]
    fun test_mint_balance_by_custom_address() {
        // Set up a test enviroment.
        let mut scenario = test_scenario::begin(@0xA);

        // Create an IOTA treasury capability.
        let mut iota_treasury_cap = iota::create_for_testing(scenario.ctx());

        // Mint some IOTA balance.
        let iota = iota_treasury_cap.mint_balance(100, scenario.ctx());

        // Cleanup.
        test_utils::destroy(iota);
        test_utils::destroy(iota_treasury_cap);

        scenario.end();
    }

    #[test]
    #[expected_failure(abort_code = iota::ENotSystemAddress)]
    fun test_burn_balance_by_custom_address() {
        // Set up a test enviroment.
        let mut scenario = test_scenario::begin(@0x0);

        // Create an IOTA treasury capability.
        let mut iota_treasury_cap = iota::create_for_testing(scenario.ctx());

        // Mint some IOTA balance.
        let iota = iota_treasury_cap.mint_balance(100, scenario.ctx());

        assert_eq(iota_treasury_cap.total_supply(), 100);

        // Switch to a custom address.
        scenario.next_tx(@0xA);

        // Burn some IOTA balance.
        iota_treasury_cap.burn_balance(iota, scenario.ctx());

        // Cleanup.
        test_utils::destroy(iota_treasury_cap);

        scenario.end();
    }
}
