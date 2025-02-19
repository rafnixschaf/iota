// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_macros::sim_test;
use iota_rest_api::{Client, ExecuteTransactionQueryParameters, client::BalanceChange};
use iota_test_transaction_builder::make_transfer_iota_transaction;
use iota_types::{
    base_types::IotaAddress, effects::TransactionEffectsAPI, transaction::TransactionDataAPI,
};
use test_cluster::TestClusterBuilder;

#[sim_test]
async fn execute_transaction_transfer() {
    let test_cluster = TestClusterBuilder::new().build().await;

    let client = Client::new(test_cluster.rpc_url());
    let address = IotaAddress::random_for_testing_only();
    let amount = 9;

    let txn =
        make_transfer_iota_transaction(&test_cluster.wallet, Some(address), Some(amount)).await;
    let sender = txn.transaction_data().sender();

    let request = ExecuteTransactionQueryParameters {
        events: false,
        balance_changes: true,
        input_objects: true,
        output_objects: true,
    };

    let response = client.execute_transaction(&request, &txn).await.unwrap();

    let gas = response.effects.gas_cost_summary().net_gas_usage();

    let mut expected = vec![
        BalanceChange {
            address: sender,
            coin_type: iota_types::gas_coin::GAS::type_tag(),
            amount: -(amount as i128 + gas as i128),
        },
        BalanceChange {
            address,
            coin_type: iota_types::gas_coin::GAS::type_tag(),
            amount: amount as i128,
        },
    ];
    expected.sort_by_key(|e| e.address);

    let mut actual = response.balance_changes.unwrap();
    actual.sort_by_key(|e| e.address);

    assert_eq!(actual, expected);
}
