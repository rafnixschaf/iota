// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module timelock::label_tests {

    use std::string;

    use sui::test_scenario;
    use sui::test_utils::assert_eq;

    use timelock::label::{Self, LabelerCap};

    use timelock::test_label_one::{Self, TEST_LABEL_ONE};
    use timelock::test_label_two::{Self, TEST_LABEL_TWO};

    public struct FAKE_WITNESS has drop {}

    #[test]
    fun test_create_labels() {
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

        // Build the labels.
        let label_one = label::from_type<TEST_LABEL_ONE>(&labeler_one);
        let label_two = label::from_type<TEST_LABEL_TWO>(&labeler_two);

        // Check the labels.
        assert_eq(label_one.is_type<TEST_LABEL_ONE>(), true);
        assert_eq(label_one.is_type<TEST_LABEL_TWO>(), false);

        assert_eq(label_two.is_type<TEST_LABEL_ONE>(), false);
        assert_eq(label_two.is_type<TEST_LABEL_TWO>(), true);

        assert_eq(&label_one == &label_one, true);
        assert_eq(&label_two == &label_two, true);
        assert_eq(&label_one == &label_two, false);

        // Cleanup.
        label::destroy(label_one);
        label::destroy(label_two);

        label::destroy_labeler_cap(labeler_one);
        label::destroy_labeler_cap(labeler_two);

        scenario.end();
    }

    #[test]
    fun test_label_value() {
        // Set up a test environment.
        let sender = @0xA;
        let mut scenario = test_scenario::begin(sender);

        // Initialize a LabelerCap instance.
        test_label_one::assign_labeler_cap(sender, scenario.ctx());

        // Advance the scenario to a new transaction.
        scenario.next_tx(sender);

        // Take the capability.
        let labeler_one = scenario.take_from_sender<LabelerCap<TEST_LABEL_ONE>>();

        // Build a label.
        let label_one = label::from_type<TEST_LABEL_ONE>(&labeler_one);

        // Check the label.
        assert_eq(*label_one.value(), string::utf8(b"00000000000000000000000000000000000000000000000000000000000010cf::test_label_one::TEST_LABEL_ONE"));

        // Cleanup.
        label::destroy(label_one);

        label::destroy_labeler_cap(labeler_one);

        scenario.end();
    }

    #[test]
    #[expected_failure(abort_code = label::ENotOneTimeWitness)]
    fun test_create_cap_with_fake_witness() {
        // Set up a test environment.
        let sender = @0xA;
        let mut scenario = test_scenario::begin(sender);

        // Fake one time witness.
        let witness = FAKE_WITNESS{};

        // Create a new capability.
        let cap = label::create_labeler_cap<FAKE_WITNESS>(witness, scenario.ctx());

        // Cleanup.
        label::destroy_labeler_cap(cap);

        scenario.end();
    }
}
