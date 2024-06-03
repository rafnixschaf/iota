// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module timelock::timelocked_balance_tests {

    use std::string;

    use sui::balance;
    use sui::sui::SUI;
    use sui::test_scenario;
    use sui::vec_set;

    use timelock::timelock::{Self, LabelerCap};
    use timelock::timelocked_balance;

    #[test]
    fun test_join_timelocked_balances() {
        // Set up a test environment.
        let sender = @0xA;
        let mut scenario = test_scenario::begin(sender);

        // Minting some IOTA.
        let iota1 = balance::create_for_testing<SUI>(10);
        let iota2 = balance::create_for_testing<SUI>(15);

        // Lock the IOTA balances.
        let mut timelock1 = timelock::lock(iota1, 100, scenario.ctx());
        let timelock2 = timelock::lock(iota2, 100, scenario.ctx());

        // Join the timelocks.
        timelocked_balance::join(&mut timelock1, timelock2);
    
        // Check the joined timelock.
        assert!(timelock1.expiration_timestamp_ms() == 100, 1);
        assert!(timelock1.locked().value() == 25, 2);

        // Cleanup.
        let (balance, _, _) = timelock::unpack(timelock1);

        balance::destroy_for_testing(balance);

        scenario.end();
    }

    #[test]
    #[expected_failure(abort_code = timelocked_balance::EDifferentExpirationTime)]
    fun test_join_timelocked_balances_with_different_exp_time() {
        // Set up a test environment.
        let sender = @0xA;
        let mut scenario = test_scenario::begin(sender);

        // Minting some IOTA.
        let iota1 = balance::create_for_testing<SUI>(10);
        let iota2 = balance::create_for_testing<SUI>(15);

        // Lock the IOTA balances.
        let mut timelock1 = timelock::lock(iota1, 100, scenario.ctx());
        let timelock2 = timelock::lock(iota2, 200, scenario.ctx());

        // Join the timelocks.
        timelocked_balance::join(&mut timelock1, timelock2);

        // Cleanup.
        let (balance, _, _) = timelock::unpack(timelock1);

        balance::destroy_for_testing(balance);

        scenario.end();
    }

    #[test]
    fun test_join_vec_timelocked_balances() {
        // Set up a test environment.
        let sender = @0xA;
        let mut scenario = test_scenario::begin(sender);

        // Minting some IOTA.
        let iota1 = balance::create_for_testing<SUI>(10);
        let iota2 = balance::create_for_testing<SUI>(15);
        let iota3 = balance::create_for_testing<SUI>(20);
        let iota4 = balance::create_for_testing<SUI>(25);

        // Lock the IOTA balances.
        let mut timelock1 = timelock::lock(iota1, 100, scenario.ctx());

        let mut others = vector[];

        others.push_back(timelock::lock(iota2, 100, scenario.ctx()));
        others.push_back(timelock::lock(iota3, 100, scenario.ctx()));
        others.push_back(timelock::lock(iota4, 100, scenario.ctx()));

        // Join the timelocks.
        timelocked_balance::join_vec(&mut timelock1, others);
    
        // Check the joined timelock.
        assert!(timelock1.expiration_timestamp_ms() == 100, 1);
        assert!(timelock1.locked().value() == 70, 2);

        // Cleanup.
        let (balance, _, _) = timelock::unpack(timelock1);

        balance::destroy_for_testing(balance);

        scenario.end();
    }

    #[test]
    fun test_join_empty_vec_timelocked_balances() {
        // Set up a test environment.
        let sender = @0xA;
        let mut scenario = test_scenario::begin(sender);

        // Minting some IOTA.
        let iota = balance::create_for_testing<SUI>(10);

        // Lock the IOTA balance.
        let mut timelock = timelock::lock(iota, 100, scenario.ctx());
        let others = vector[];

        // Join the timelocks.
        timelocked_balance::join_vec(&mut timelock, others);
    
        // Check the joined timelock.
        assert!(timelock.expiration_timestamp_ms() == 100, 1);
        assert!(timelock.locked().value() == 10, 2);

        // Cleanup.
        let (balance, _, _) = timelock::unpack(timelock);

        balance::destroy_for_testing(balance);

        scenario.end();
    }

    #[test]
    #[expected_failure(abort_code = timelocked_balance::EDifferentExpirationTime)]
    fun test_join_vec_timelocked_balances_with_different_exp_time() {
        // Set up a test environment.
        let sender = @0xA;
        let mut scenario = test_scenario::begin(sender);

        // Minting some IOTA.
        let iota1 = balance::create_for_testing<SUI>(10);
        let iota2 = balance::create_for_testing<SUI>(15);
        let iota3 = balance::create_for_testing<SUI>(20);
        let iota4 = balance::create_for_testing<SUI>(25);

        // Lock the IOTA balances.
        let mut timelock1 = timelock::lock(iota1, 100, scenario.ctx());

        let mut others = vector[];

        others.push_back(timelock::lock(iota2, 100, scenario.ctx()));
        others.push_back(timelock::lock(iota3, 200, scenario.ctx()));
        others.push_back(timelock::lock(iota4, 100, scenario.ctx()));

        // Join the timelocks.
        timelocked_balance::join_vec(&mut timelock1, others);

        // Cleanup.
        let (balance, _, _) = timelock::unpack(timelock1);

        balance::destroy_for_testing(balance);

        scenario.end();
    }

    #[test]
    fun test_split_timelocked_balances() {
        // Set up a test environment.
        let sender = @0xA;
        let mut scenario = test_scenario::begin(sender);

        // Minting some IOTA.
        let iota = balance::create_for_testing<SUI>(10);

        // Lock the IOTA balance.
        let mut original = timelock::lock(iota, 100, scenario.ctx());

        // Split the timelock.
        let splitted = timelocked_balance::split(&mut original, 3, scenario.ctx());
    
        // Check the original timelock.
        assert!(original.expiration_timestamp_ms() == 100, 1);
        assert!(original.locked().value() == 7, 2);

        // Check the splitted timelock.
        assert!(splitted.expiration_timestamp_ms() == 100, 3);
        assert!(splitted.locked().value() == 3, 4);

        // Cleanup.
        let (balance, _, _) = timelock::unpack(original);
        balance::destroy_for_testing(balance);

        let (balance, _, _) = timelock::unpack(splitted);
        balance::destroy_for_testing(balance);

        scenario.end();
    }

    #[test]
    fun test_split_zero_value_from_timelocked_balances() {
        // Set up a test environment.
        let sender = @0xA;
        let mut scenario = test_scenario::begin(sender);

        // Minting some IOTA.
        let iota = balance::create_for_testing<SUI>(10);

        // Lock the IOTA balance.
        let mut original = timelock::lock(iota, 100, scenario.ctx());

        // Split the timelock.
        let splitted = timelocked_balance::split(&mut original, 0, scenario.ctx());
    
        // Check the original timelock.
        assert!(original.expiration_timestamp_ms() == 100, 1);
        assert!(original.locked().value() == 10, 2);

        // Check the splitted timelock.
        assert!(splitted.expiration_timestamp_ms() == 100, 3);
        assert!(splitted.locked().value() == 0, 4);

        // Cleanup.
        let (balance, _, _) = timelock::unpack(original);
        balance::destroy_for_testing(balance);

        let (balance, _, _) = timelock::unpack(splitted);
        balance::destroy_for_testing(balance);

        scenario.end();
    }

    #[test]
    fun test_split_same_value_from_timelocked_balances() {
        // Set up a test environment.
        let sender = @0xA;
        let mut scenario = test_scenario::begin(sender);

        // Minting some IOTA.
        let iota = balance::create_for_testing<SUI>(10);

        // Lock the IOTA balance.
        let mut original = timelock::lock(iota, 100, scenario.ctx());

        // Split the timelock.
        let splitted = timelocked_balance::split(&mut original, 10, scenario.ctx());

        // Check the original timelock.
        assert!(original.expiration_timestamp_ms() == 100, 0);
        assert!(original.locked().value() == 0, 1);

        // Check the splitted timelock.
        assert!(splitted.expiration_timestamp_ms() == 100, 2);
        assert!(splitted.locked().value() == 10, 3);

        // Cleanup.
        let (balance, _, _) = timelock::unpack(original);
        balance::destroy_for_testing(balance);

        let (balance, _, _) = timelock::unpack(splitted);
        balance::destroy_for_testing(balance);

        scenario.end();
    }

    #[test]
    #[expected_failure(abort_code = balance::ENotEnough)]
    fun test_split_bigger_value_from_timelocked_balances() {
        // Set up a test environment.
        let sender = @0xA;
        let mut scenario = test_scenario::begin(sender);

        // Minting some IOTA.
        let iota = balance::create_for_testing<SUI>(10);

        // Lock the IOTA balance.
        let mut original = timelock::lock(iota, 100, scenario.ctx());

        // Split the timelock.
        let splitted = timelocked_balance::split(&mut original, 11, scenario.ctx());

        // Cleanup.
        let (balance, _, _) = timelock::unpack(original);
        balance::destroy_for_testing(balance);

        let (balance, _, _) = timelock::unpack(splitted);
        balance::destroy_for_testing(balance);

        scenario.end();
    }

    #[test]
    fun test_join_labeled_timelocked_balances() {
        // Set up a test environment.
        let system = @0x0;
        let sender = @0xA;
        let mut scenario = test_scenario::begin(system);

        // Initialize a LabelerCap.
        timelock::assign_labeler_cap(sender, scenario.ctx());

        // Switch to sender.
        scenario.next_tx(sender);

        // Take the capability.
        let cap = scenario.take_from_sender<LabelerCap>();

        // Minting some IOTA.
        let iota1 = balance::create_for_testing<SUI>(10);
        let iota2 = balance::create_for_testing<SUI>(15);

        // Lock the IOTA balances.
        let label1 = string::utf8(b"label1");
        let label2 = string::utf8(b"label2");

        let mut labels = vec_set::empty();

        labels.insert(label1);
        labels.insert(label2);

        let mut timelock1 = timelock::lock_labeled(&cap, iota1, 100, labels, scenario.ctx());
        let timelock2 = timelock::lock_labeled(&cap, iota2, 100, labels, scenario.ctx());

        // Join the timelocks.
        timelocked_balance::join(&mut timelock1, timelock2);
    
        // Check the joined timelock.
        assert!(timelock1.locked().value() == 25, 0);
        assert!(timelock1.expiration_timestamp_ms() == 100, 1);
        assert!(timelock1.is_labeled_with(&label1), 2);
        assert!(timelock1.is_labeled_with(&label2), 3);
        assert!(timelock1.labels().borrow().keys() == labels.keys(), 4);

        // Cleanup.
        let (balance, _, _) = timelock::unpack(timelock1);

        balance::destroy_for_testing(balance);

        scenario.return_to_sender(cap);

        scenario.end();
    }

    #[test]
    #[expected_failure(abort_code = timelocked_balance::EDifferentLabels)]
    fun test_join_labeled_timelocked_balances_with_different_labels() {
        // Set up a test environment.
        let system = @0x0;
        let sender = @0xA;
        let mut scenario = test_scenario::begin(system);

        // Initialize a LabelerCap.
        timelock::assign_labeler_cap(sender, scenario.ctx());

        // Switch to sender.
        scenario.next_tx(sender);

        // Take the capability.
        let cap = scenario.take_from_sender<LabelerCap>();

        // Minting some IOTA.
        let iota1 = balance::create_for_testing<SUI>(10);
        let iota2 = balance::create_for_testing<SUI>(15);

        // Lock the IOTA balance.
        let label1 = string::utf8(b"label1");
        let label2 = string::utf8(b"label2");

        let mut labels1 = vec_set::empty();
        labels1.insert(label1);

        let mut labels2 = vec_set::empty();
        labels2.insert(label2);

        let mut timelock1 = timelock::lock_labeled(&cap, iota1, 100, labels1, scenario.ctx());
        let timelock2 = timelock::lock_labeled(&cap, iota2, 100, labels2, scenario.ctx());

        // Join the timelocks.
        timelocked_balance::join(&mut timelock1, timelock2);

        // Cleanup.
        let (balance, _, _) = timelock::unpack(timelock1);

        balance::destroy_for_testing(balance);

        scenario.return_to_sender(cap);

        scenario.end();
    }

    #[test]
    fun test_split_labeled_timelocked_balances() {
        // Set up a test environment.
        let system = @0x0;
        let sender = @0xA;
        let mut scenario = test_scenario::begin(system);

        // Initialize a LabelerCap.
        timelock::assign_labeler_cap(sender, scenario.ctx());

        // Switch to sender.
        scenario.next_tx(sender);

        // Take the capability.
        let cap = scenario.take_from_sender<LabelerCap>();

        // Minting some IOTA.
        let iota = balance::create_for_testing<SUI>(10);

        // Lock the IOTA balance.
        let label1 = string::utf8(b"label1");
        let label2 = string::utf8(b"label2");

        let mut labels = vec_set::empty();

        labels.insert(label1);
        labels.insert(label2);

        let mut original = timelock::lock_labeled(&cap, iota, 100, labels, scenario.ctx());

        // Split the timelock.
        let splitted = timelocked_balance::split(&mut original, 3, scenario.ctx());
    
        // Check the original timelock.
        assert!(original.locked().value() == 7, 0);
        assert!(original.expiration_timestamp_ms() == 100, 1);
        assert!(original.is_labeled_with(&label1), 2);
        assert!(original.is_labeled_with(&label2), 3);
        assert!(original.labels().borrow().keys() == labels.keys(), 4);

        // Check the splitted timelock.
        assert!(splitted.locked().value() == 3, 5);
        assert!(splitted.expiration_timestamp_ms() == 100, 6);
        assert!(splitted.is_labeled_with(&label1), 7);
        assert!(splitted.is_labeled_with(&label2), 8);
        assert!(original.labels() == option::some(labels), 9);

        // Cleanup.
        let (balance, _, _) = timelock::unpack(original);
        balance::destroy_for_testing(balance);

        let (balance, _, _) = timelock::unpack(splitted);
        balance::destroy_for_testing(balance);

        scenario.return_to_sender(cap);

        scenario.end();
    }
}
