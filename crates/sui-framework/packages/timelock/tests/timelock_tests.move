// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module timelock::timelock_tests {

    use sui::balance;
    use sui::sui::SUI;
    use sui::test_scenario;
    use sui::vec_set;

    use timelock::timelock::{Self, LabelerCap};

    #[test]
    fun test_lock_unlock_flow() {
        // Set up a test environment.
        let sender = @0xA;
        let mut scenario = test_scenario::begin(sender);

        // Minting some IOTA.
        let iota = balance::create_for_testing<SUI>(10);

        // Lock the IOTA balance.
        let timelock = timelock::lock(iota, 100, scenario.ctx());

        // Check the locked IOTA.
        assert!(timelock.locked().value() == 10, 0);

        // Check if the timelock is locked.
        assert!(timelock.is_locked(scenario.ctx()), 1);
        assert!(timelock.remaining_time(scenario.ctx()) == 100, 2);

        // Check labels.
        assert!(timelock.labels().is_none(), 3);

        // Increment epoch timestamp.
        scenario.ctx().increment_epoch_timestamp(10);

        // Check if the timelock is still locked.
        assert!(timelock.is_locked(scenario.ctx()), 4);
        assert!(timelock.remaining_time(scenario.ctx()) == 90, 5);

        // Increment epoch timestamp again.
        scenario.ctx().increment_epoch_timestamp(90);

        // Check if the timelock is unlocked.
        assert!(!timelock.is_locked(scenario.ctx()), 6);
        assert!(timelock.remaining_time(scenario.ctx()) == 0, 7);

        // Unlock the IOTA balance.
        let balance = timelock::unlock(timelock, scenario.ctx());

        // Check the unlocked IOTA balance.
        assert!(balance.value() == 10, 8);

        // Cleanup.
        balance::destroy_for_testing(balance);

        scenario.end();
    }

    #[test]
    fun test_lock_unlock_labeled_flow() {
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
        let label1 = b"label1";
        let label2 = b"label2";

        let mut labels = vec_set::empty();

        labels.insert(label1);
        labels.insert(label2);
    
        let timelock = timelock::lock_labeled(&cap, iota, 100, labels, scenario.ctx());

        // Check the locked IOTA.
        assert!(timelock.locked().value() == 10, 0);

        // Check if the timelock is locked.
        assert!(timelock.is_locked(scenario.ctx()), 1);
        assert!(timelock.remaining_time(scenario.ctx()) == 100, 2);

        // Check labels.
        assert!(timelock.labels() == option::some(labels), 3);
        assert!(timelock.is_labeled_with(&label1), 4);
        assert!(timelock.is_labeled_with(&label2), 5);

        // Increment epoch timestamp.
        scenario.ctx().increment_epoch_timestamp(10);

        // Check if the timelock is still locked.
        assert!(timelock.is_locked(scenario.ctx()), 6);
        assert!(timelock.remaining_time(scenario.ctx()) == 90, 7);

        // Increment epoch timestamp again.
        scenario.ctx().increment_epoch_timestamp(90);

        // Check if the timelock is unlocked.
        assert!(!timelock.is_locked(scenario.ctx()), 8);
        assert!(timelock.remaining_time(scenario.ctx()) == 0, 9);

        // Unlock the IOTA balance.
        let balance = timelock::unlock(timelock, scenario.ctx());

        // Check the unlocked IOTA balance.
        assert!(balance.value() == 10, 10);

        // Cleanup.
        balance::destroy_for_testing(balance);

        scenario.return_to_sender(cap);

        scenario.end();
    }

    #[test]
    #[expected_failure(abort_code = timelock::EExpireEpochIsPast)]
    fun test_expiration_time_is_passed() {
        // Set up a test environment.
        let sender = @0xA;
        let mut scenario = test_scenario::begin(sender);

        // Increment epoch timestamp.
        scenario.ctx().increment_epoch_timestamp(100);

        // Minting some IOTA.
        let iota = balance::create_for_testing<SUI>(10);

        // Lock the IOTA balance with a wrong expiration time.
        let timelock = timelock::lock(iota, 10, scenario.ctx());

        // Cleanup.
        let (balance, _, _) = timelock::unpack(timelock);

        balance::destroy_for_testing(balance);

        scenario.end();
    }

    #[test]
    #[expected_failure(abort_code = timelock::ENotExpiredYet)]
    fun test_unlock_not_expired_object() {
        // Set up a test environment.
        let sender = @0xA;
        let mut scenario = test_scenario::begin(sender);

        // Minting some IOTA.
        let iota = balance::create_for_testing<SUI>(10);

        // Lock the IOTA balance.
        let timelock = timelock::lock(iota, 100, scenario.ctx());

        // Increment epoch timestamp.
        scenario.ctx().increment_epoch_timestamp(10);

        // Unlock the IOTA balance which is not expired.
        let balance = timelock::unlock(timelock, scenario.ctx());

        // Cleanup.
        balance::destroy_for_testing(balance);

        scenario.end();
    }

    #[test]
    #[expected_failure(abort_code = timelock::EEmptyLabelsCollection)]
    fun test_labeled_timelock_with_empty_labels_collection() {
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
        let labels = vec_set::empty();
        let timelock = timelock::lock_labeled(&cap, iota, 100, labels, scenario.ctx());

        // Cleanup.
        let (balance, _, _) = timelock::unpack(timelock);

        balance::destroy_for_testing(balance);

        scenario.return_to_sender(cap);

        scenario.end();
    }

    #[test]
    #[expected_failure(abort_code = timelock::EEmptyLabel)]
    fun test_labeled_timelock_with_empty_label() {
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
        let mut labels = vec_set::empty();

        labels.insert(vector::empty());

        let timelock = timelock::lock_labeled(&cap, iota, 100, labels, scenario.ctx());

        // Cleanup.
        let (balance, _, _) = timelock::unpack(timelock);

        balance::destroy_for_testing(balance);

        scenario.return_to_sender(cap);

        scenario.end();
    }
}
