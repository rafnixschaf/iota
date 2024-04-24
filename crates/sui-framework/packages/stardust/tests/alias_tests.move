module stardust::alias_tests{
    use std::{
        option::{some},
        type_name::{get}
    };
    use sui::{
        sui::{SUI},
        balance::{Self},
        bag::{Self},
        coin::{Self},
    };
    use stardust::{
        alias::{Self},
        utilities::{Self as utils},
    };
        
    const ENativeTokenBagEmpty: u64 = 0;
    const ENativeTokenBagNonEmpty: u64 = 1;
    
    // one time witness for a coin used in tests
    // we can not declare these inside the test function, and there is no constructor for Basic so we can't test it in a test module
    public struct TEST_A has drop {}
    public struct TEST_B has drop {}

    // demonstration on how to claim the assets from a basic output with all unlock conditions inside one PTB
    #[test]
    fun demonstrate_claiming_ptb() {
        let initial_iota_in_output = 10000;
        let initial_testA_in_output = 100;
        let initial_testB_in_output = 100;

        let owner = @0xA;
        let migrate_to = @0xD; 

        // create a new tx context
        let mut ctx = tx_context::new(
            // sender
            @0xA,
            // tx)hash
            x"3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532",
            // epoch
            1,
            // epoch ts in ms (10 in seconds)
            10000,
            // ids created
            0,
        );

        // mint some tokens
        let sui_balance = balance::create_for_testing<SUI>(initial_iota_in_output);
        let test_a_balance = balance::create_for_testing<TEST_A>(initial_testA_in_output);
        let test_b_balance = balance::create_for_testing<TEST_B>(initial_testB_in_output);

        let mut token_bag = bag::new(&mut ctx);
        // add the native token balances to the bag
        token_bag.add(get<TEST_A>().into_string(), test_a_balance);
        token_bag.add(get<TEST_B>().into_string(), test_b_balance);

        let (mut output, governor_cap, state_cap) = alias::create_for_testing(
            sui_balance,
            // state cap version
            0,
            // state index
            0,
            // state metadata
            some(b"state metadata content"),
            // sender feature
            some(owner),
            // metadata feature
            some(b"metadata content"),
            // issuer feature
            some(owner),
            // immutable metadata
            some(b"immutable metadata content"),
            &mut ctx,
        );
        output.attach_tokens(token_bag);

        // command 1: extract the base token and native token bag
        let (extracted_base_token, mut native_token_bag_option) = output.extract_assets(&state_cap);
        assert!(native_token_bag_option.is_some(), ENativeTokenBagEmpty);
        let mut native_token_bag = native_token_bag_option.extract();
        option::destroy_none(native_token_bag_option);

        // command 2: extract asset A and send to user
        utils::extract_and_send_to<TEST_A>(&mut native_token_bag, migrate_to, &mut ctx);

        // command 3: extract asset B and send to user
        utils::extract_and_send_to<TEST_B>(&mut native_token_bag, migrate_to, &mut ctx);
        assert!(native_token_bag.is_empty(), ENativeTokenBagNonEmpty);
        
        // command 4: delete the bag
        native_token_bag.destroy_empty();

        // comand 5: create coin from the extracted iota balance
        let iota_coin = coin::from_balance(extracted_base_token, &mut ctx);

        // command 6: send back the base token coin to the user
        transfer::public_transfer(iota_coin, migrate_to);

        // command 7: destroy state cap
        output.destroy_state_cap(state_cap);

        // command 8: destroy the alias output
        output.destroy(governor_cap);

        // !!! migration complete !!! 
    }
}