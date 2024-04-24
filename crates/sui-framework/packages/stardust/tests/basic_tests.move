module stardust::basic_tests{
    use std::{
        option::{some},
        type_name::{get}
    };
    use sui::{
        sui::{SUI},
        balance::{Self},
        bag::{Self},
    };
    use stardust::{
        basic::{Self},
        expiration_unlock_condition::{Self as expiration},
        storage_deposit_return_unlock_condition::{Self as sdruc},
        timelock_unlock_condition::{Self as timelock},
        utilities::{Self as utils},
    };
        
    const ENoBaseTokenBalance: u64 = 1;
    const ENativeTokenBagNonEmpty: u64 = 2;
    const EIotaBlanceMismatch: u64 = 3;
    
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
        let sdruc_return_amount = 1000;

        let timelocked_until = 5;
        let expiration_after = 20;
        let owner = @0xA;
        let sdruc_return_address = @0xB;
        let expiration_return_address = @0xC;
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

        let output = basic::create_for_testing(
            sui_balance,
            token_bag,
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

        // ready with the basic output, now we can claim the assets
        // the task is to assemble a PTB like transaction in move that demonstrates how to claim
        // PTB inputs: basic output ID (`basic`) and addredd to migrate to (`migrate_to`)

        // command 1: extract the base token and native token bag
        let (extracted_base_token_option, mut native_token_bag) = output.extract_assets(&mut ctx);
        assert!(extracted_base_token_option.is_some(), ENoBaseTokenBalance);

        // command 2: extract asset A and send to user
        native_token_bag = utils::extract_and_send_to<TEST_A>(native_token_bag, migrate_to, &mut ctx);

        // command 3: extract asset B and send to user
        native_token_bag = utils::extract_and_send_to<TEST_B>(native_token_bag, migrate_to, &mut ctx);
        assert!(native_token_bag.is_empty(), ENativeTokenBagNonEmpty);
        
        // command 4: delete the bag
        native_token_bag.destroy_empty();

        // comand 5: create coin from the extracted iota balance
        let iota_coin = utils::create_coin_from_option_balance(extracted_base_token_option, &mut ctx);
        // we should have `initial_iota_in_output` - `sdruc_return_amount` left in the coin
        assert!(iota_coin.value() == (initial_iota_in_output - sdruc_return_amount), EIotaBlanceMismatch);

        // command 6: send back the base token coin to the user
        // if we sponsored the transaction with our own coins, now is the time to detuct it from the user by taking from `iota_coin` and merging it into the gas token
        // since we can dry run the tx before submission, we know how much to charge the user, or we charge the whole gas budget
        transfer::public_transfer(iota_coin, migrate_to);

        // !!! migration complete !!! 
    }
}