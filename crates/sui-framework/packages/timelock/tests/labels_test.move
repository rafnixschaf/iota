// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module timelock::labels_tests {

    use sui::test_scenario;
    use sui::test_utils::assert_eq;

    use timelock::labels::{Self, LabelerCap};
    use timelock::merge_label::MERGE_LABEL;

    use timelock::test_label_one::{Self, TEST_LABEL_ONE};
    use timelock::test_label_two::{Self, TEST_LABEL_TWO};

    public struct FAKE_WITNESS has drop {}

    #[test]
    fun test_build_labels() {
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
        let labels = labels::create_builder()
            .with_label<TEST_LABEL_ONE>(&labeler_one)
            .with_label<TEST_LABEL_TWO>(&labeler_two)
            .into_labels();

        // Check the labels.
        assert_eq(labels.contains<TEST_LABEL_ONE>(), true);
        assert_eq(labels.contains<TEST_LABEL_TWO>(), true);
        assert_eq(labels.contains<MERGE_LABEL>(), false);
        assert_eq(labels.is_empty(), false);

        // Cleanup.
        labels::destroy(labels);

        labels::destroy_labeler_cap(labeler_one);
        labels::destroy_labeler_cap(labeler_two);

        scenario.end();
    }

    #[test]
    fun test_build_zero_labels() {
        // Set up a test environment.
        let sender = @0xA;
        let scenario = test_scenario::begin(sender);

        // Build the labels.
        let labels = labels::create_builder().into_labels();

        // Check the labels.
        assert_eq(labels.contains<TEST_LABEL_ONE>(), false);
        assert_eq(labels.contains<TEST_LABEL_TWO>(), false);
        assert_eq(labels.is_empty(), true);

        // Cleanup.
        labels::destroy(labels);

        scenario.end();
    }

    #[test]
    #[expected_failure(abort_code = labels::ELabelAlreadyExists)]
    fun test_insert_the_same_label() {
        // Set up a test environment.
        let sender = @0xA;
        let mut scenario = test_scenario::begin(sender);

        // Initialize LabelerCap instances.
        test_label_one::assign_labeler_cap(sender, scenario.ctx());

        // Advance the scenario to a new transaction.
        scenario.next_tx(sender);

        // Take the capabilities.
        let labeler_one = scenario.take_from_sender<LabelerCap<TEST_LABEL_ONE>>();

        // Build the labels.
        let labels = labels::create_builder()
            .with_label<TEST_LABEL_ONE>(&labeler_one)
            .with_label<TEST_LABEL_ONE>(&labeler_one)
            .into_labels();

        // Cleanup.
        labels::destroy(labels);

        labels::destroy_labeler_cap(labeler_one);

        scenario.end();
    }

    #[test]
    #[expected_failure(abort_code = labels::ENotOneTimeWitness)]
    fun test_create_cap_with_fake_witness() {
        // Set up a test environment.
        let sender = @0xA;
        let mut scenario = test_scenario::begin(sender);

        // Fake one time witness.
        let witness = FAKE_WITNESS{};

        // Create a new capability.
        let cap = labels::create_labeler_cap<FAKE_WITNESS>(witness, scenario.ctx());

        // Cleanup.
        labels::destroy_labeler_cap(cap);

        scenario.end();
    }
}
