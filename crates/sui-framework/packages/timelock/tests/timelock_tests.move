// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module timelock::timelock_tests {

    use sui::balance;
    use sui::sui::SUI;
    use sui::test_scenario;
    use sui::test_utils::{Self, assert_eq};

    use timelock::labels::{Self, LabelerCap};
    use timelock::timelock;

    use timelock::test_label_one::{Self, TEST_LABEL_ONE};
    use timelock::test_label_two::{Self, TEST_LABEL_TWO};

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
        assert_eq(timelock.locked().value(), 10);

        // Check if the timelock is locked.
        assert_eq(timelock.is_locked(scenario.ctx()), true);
        assert_eq(timelock.remaining_time(scenario.ctx()), 100);

        // Check labels.
        assert_eq(timelock.labels().is_empty(), true);

        // Increment epoch timestamp.
        scenario.ctx().increment_epoch_timestamp(10);

        // Check if the timelock is still locked.
        assert_eq(timelock.is_locked(scenario.ctx()), true);
        assert_eq(timelock.remaining_time(scenario.ctx()), 90);

        // Increment epoch timestamp again.
        scenario.ctx().increment_epoch_timestamp(90);

        // Check if the timelock is unlocked.
        assert_eq(timelock.is_locked(scenario.ctx()), false);
        assert_eq(timelock.remaining_time(scenario.ctx()), 0);

        // Unlock the IOTA balance.
        let balance = timelock::unlock(timelock, scenario.ctx());

        // Check the unlocked IOTA balance.
        assert_eq(balance.value(), 10);

        // Cleanup.
        balance::destroy_for_testing(balance);

        scenario.end();
    }

    #[test]
    fun test_lock_unlock_labeled_flow() {
        // Set up a test environment.
        let sender = @0xA;
        let mut scenario = test_scenario::begin(sender);

        // Initialize LabelerCap instances.
        test_label_one::assign_labeler_cap(sender, scenario.ctx());
        test_label_two::assign_labeler_cap(sender, scenario.ctx());

        // Advance the scenario to a new transaction.
        scenario.next_tx(sender);

        // Take the capabilities.
        let labeler_one = scenario.take_from_sender<LabelerCap<TEST_LABEL_ONE>>();
        let labeler_two = scenario.take_from_sender<LabelerCap<TEST_LABEL_TWO>>();

        // Minting some IOTA.
        let iota = balance::create_for_testing<SUI>(10);

        // Lock the IOTA balance.
        let labels = labels::create_builder()
            .with_label<TEST_LABEL_ONE>(&labeler_one)
            .with_label<TEST_LABEL_TWO>(&labeler_two)
            .into_labels();
    
        let timelock = timelock::lock_with_labels(iota, 100, labels, scenario.ctx());

        // Check the locked IOTA.
        assert_eq(timelock.locked().value(), 10);

        // Check if the timelock is locked.
        assert_eq(timelock.is_locked(scenario.ctx()), true);
        assert_eq(timelock.remaining_time(scenario.ctx()), 100);

        // Check the labels.
        assert_eq(timelock.labels().contains<TEST_LABEL_ONE>(), true);
        assert_eq(timelock.labels().contains<TEST_LABEL_TWO>(), true);

        // Increment epoch timestamp.
        scenario.ctx().increment_epoch_timestamp(10);

        // Check if the timelock is still locked.
        assert_eq(timelock.is_locked(scenario.ctx()), true);
        assert_eq(timelock.remaining_time(scenario.ctx()), 90);

        // Increment epoch timestamp again.
        scenario.ctx().increment_epoch_timestamp(90);

        // Check if the timelock is unlocked.
        assert_eq(timelock.is_locked(scenario.ctx()), false);
        assert_eq(timelock.remaining_time(scenario.ctx()), 0);

        // Unlock the IOTA balance.
        let balance = timelock::unlock(timelock, scenario.ctx());

        // Check the unlocked IOTA balance.
        assert_eq(balance.value(), 10);

        // Cleanup.
        balance::destroy_for_testing(balance);

        scenario.return_to_sender(labeler_one);
        scenario.return_to_sender(labeler_two);

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
        test_utils::destroy(timelock);

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
}
