module stardust::address_unlock_condition_tests{
    use std::{
        option::{some},
    };
    use sui::{
        sui::{SUI},
        balance::{Self},
        bag::{Self},
        coin::{Self},
    };
    use stardust::{
        alias::{Self},
        alias_output::{Self},
        basic::{Self},
        expiration_unlock_condition::{Self as expiration},
        storage_deposit_return_unlock_condition::{Self as sdruc},
        timelock_unlock_condition::{Self as timelock},
        utilities::{Self as utils},
    };
        
    const ENativeTokenBagNonEmpty: u64 = 1;
    const EIotaBlanceMismatch: u64 = 3;
    
    // one time witness for a coin used in tests
    // we can not declare these inside the test function, and there is no constructor for Basic so we can't test it in a test module
    public struct TEST_A has drop {}
    public struct TEST_B has drop {}

    // demonstration on how to claim the assets from a basic alias_output with all unlock conditions inside one PTB
    #[test]
    fun demonstrate_alias_address_unlocking() {
        let initial_iota_in_output = 10000;

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

        let mut alias_output = alias_output::create_for_testing(
            // iota
            balance::create_for_testing<SUI>(initial_iota_in_output),
            // tokens
            bag::new(&mut ctx),
            &mut ctx,
        );
        let alias = alias::create_for_testing(
            // legacy state controller
            some(owner),
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
        alias_output.attach_alias(alias);

        // Basic Output owned by the alias
        let basic_sui_balance = balance::create_for_testing<SUI>(initial_iota_in_output);
        let timelocked_until = 5;
        let expiration_after = 20;
        let sdruc_return_address = @0xB;
        let sdruc_return_amount = 1000;
        let expiration_return_address = @0xC;
        let basic_output = basic::create_for_testing(
            basic_sui_balance,
            bag::new(&mut ctx),
            some(sdruc::create_for_testing(sdruc_return_address, sdruc_return_amount)),
            some(timelock::create_for_testing(timelocked_until)),
            some(expiration::create_for_testing(owner, expiration_return_address, expiration_after)),
            // metadata feature
            some(b"metadata content"),
            // tag feature
            some(b"tag content"),
            // sender feature
            some(owner),
            &mut ctx,
        );

        // command 1: unlock the basic token
        // TODO: is it possible to create a Receiving object?
        // transfer::transfer(basic_output,alias.id().uid_to_address());
        // let basic_output = unlock_alias_address_owned_basic(&mut alias,basic_output);

        // command 2: extract the base token and native token bag
        let (extracted_base_token_option, native_token_bag) = basic_output.extract_assets(&mut ctx);
        
        // command 3: delete the bag
        native_token_bag.destroy_empty();

        // comand 4: create coin from the extracted iota balance
        let iota_coin = utils::create_coin_from_option_balance(extracted_base_token_option, &mut ctx);
        // we should have `initial_iota_in_output` - `sdruc_return_amount` left in the coin
        assert!(iota_coin.value() == (initial_iota_in_output - sdruc_return_amount), EIotaBlanceMismatch);

        // command 6: send back the base token coin to the user
        transfer::public_transfer(iota_coin, migrate_to);

        // command 7: extract the base token and native token bag
        let (extracted_base_token, native_token_bag, extracted_alias) = alias_output.extract_assets();

        // command 8: delete the bag
        assert!(native_token_bag.is_empty(), ENativeTokenBagNonEmpty);
        native_token_bag.destroy_empty();

        // comand 9: create coin from the extracted iota balance
        let iota_coin = coin::from_balance(extracted_base_token, &mut ctx);

        // command 10: send back the base token coin to the user
        transfer::public_transfer(iota_coin, migrate_to);

        // command 10: destroy alias
        extracted_alias.destroy();

        // !!! migration complete !!! 
    }
}