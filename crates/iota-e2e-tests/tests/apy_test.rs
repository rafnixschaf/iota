// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use fastcrypto::ed25519::Ed25519KeyPair;
use iota_json_rpc_api::{CoinReadApiClient, GovernanceReadApiClient};
use iota_keys::keystore::AccountKeystore;
use iota_macros::sim_test;
use iota_swarm_config::genesis_config::{AccountConfig, GenesisConfig, DEFAULT_GAS_AMOUNT};
use iota_test_transaction_builder::TestTransactionBuilder;
use iota_types::{
    crypto::{get_key_pair_from_rng, IotaKeyPair},
    gas_coin::NANOS_PER_IOTA,
};
use test_cluster::TestClusterBuilder;

/// Assuming the following parameters (in IOTAs):
///
/// - VALIDATOR_LOW_STAKE_THRESHOLD_NANOS = 1_500_00
/// - COMMISSION_RATE = 2%
/// - VALIDATOR_TARGET_REWARD = 767_000
/// - SINGLE_VALIDATOR_REWARD = VALIDATOR_TARGET_REWARD / 4
/// - SINGLE_VALIDATOR_COMMISSION = 0.02 * SINGLE_VALIDATOR_REWARD
/// - STAKE = 3_500_000_000
///
/// After epoch 0, we expect the validator to which we staked to have:
/// - An iota_balance of the original staked 1_500_000 plus the distributed
///   rewards of SINGLE_VALIDATOR_REWARD - SINGLE_VALIDATOR_COMMISSION =
///   1_687_915.
/// - The pool should have a total stake of the originally staked 1_500_000 plus
///   the newly staked STAKE plus the entire reward it received (=
///   SINGLE_VALIDATOR_REWARD, since the commission is also staked) =
///   3_501_691_750.
/// - The new pool token balance is then given by
/// (pool_token_epoch_0 * new_stake) / iota_balance_after_rewards = (1_500_000 *
/// 3_501_691_750) / 1_687_915 = 3_111_849_604.
///
/// Finally, this gives us the exchange rate for that pool for epoch 1:
/// iota_amount = 3_501_691_750
/// pool_token = 3_111_849_604
/// rate_epoch1 = pool_token / iota_amount ~= 0.8886703418122358
///
/// After epoch 1, we expect the validator to which we staked to have
/// - An iota_balance of the previously staked 3_501_691_750 plus the
///   distributed rewards of SINGLE_VALIDATOR_REWARD -
///   SINGLE_VALIDATOR_COMMISSION = 3_501_879_665.
/// - The pool should have a total stake of the previously staked 3_501_691_750
///   plus the entire reward it received (= SINGLE_VALIDATOR_REWARD, since the
///   commission is also staked) = 3_501_883_500.
/// - The new pool token balance is then given by
/// (pool_token_epoch_1 * new_stake) / iota_balance_after_rewards =
/// (3_111_849_604 * 3_501_883_500) / 3_501_879_665 = 3_111_853_012.
///
/// Finally, this gives us the exchange rate for that pool for epoch 1:
/// iota_amount = 3_501_883_500
/// pool_token = 3_111_853_012
/// rate_epoch2 = pool_token / iota_amount ~= 0.8886226547118051
///
/// And with those two rates we can calculate the APY:
/// apy = ((rate_epoch1/rate_epoch2)^365) - 1 ~= 0.019779937783565904
#[sim_test]
async fn test_apy() {
    // We need a large stake for low enough APY values such that they are not
    // filtered out by the APY calculation function.
    let stake = 3_500_000_000 * NANOS_PER_IOTA;
    let mut rng = rand::thread_rng();
    let mut genesis_config = GenesisConfig::for_local_testing();
    let (address, keypair): (_, Ed25519KeyPair) = get_key_pair_from_rng(&mut rng);
    genesis_config.accounts.extend([AccountConfig {
        address: Some(address),
        gas_amounts: vec![DEFAULT_GAS_AMOUNT, stake],
    }]);

    let mut test_cluster = TestClusterBuilder::new()
        .set_genesis_config(genesis_config)
        .with_epoch_duration_ms(10_000)
        .build()
        .await;

    test_cluster
        .wallet
        .config
        .keystore
        .add_key(None, IotaKeyPair::Ed25519(keypair))
        .unwrap();

    let ref_gas_price = test_cluster.get_reference_gas_price().await;

    let client = test_cluster.rpc_client();
    let mut coins = client
        .get_coins(address, None, None, None)
        .await
        .unwrap()
        .data
        .into_iter();
    let (gas_coin, stake_coin) = {
        let coin1 = coins.next().expect("there should be at least two coins");
        let coin2 = coins.next().expect("there should be at least two coins");

        if coin1.balance > coin2.balance {
            (coin2, coin1)
        } else {
            (coin1, coin2)
        }
    };

    let validator_address = test_cluster
        .swarm
        .active_validators()
        .next()
        .unwrap()
        .config
        .iota_address();
    let transaction = TestTransactionBuilder::new(address, gas_coin.object_ref(), ref_gas_price)
        .call_staking(stake_coin.object_ref(), validator_address)
        .build();
    test_cluster
        .sign_and_execute_transaction(&transaction)
        .await;

    // Wait for two epochs with the new stake so we get two new exchange rates which
    // are minimally needed to calculate the APY.
    test_cluster.wait_for_epoch(None).await;
    test_cluster.wait_for_epoch(None).await;

    let http_client = test_cluster.rpc_client();

    let apys = http_client
        .get_validators_apy()
        .await
        .expect("call should succeed");

    assert_eq!(apys.epoch, 2);

    let validator_apy = apys
        .apys
        .iter()
        .find(|validator_apy| validator_apy.address == validator_address)
        .unwrap();

    // See description above for the origin of this value.
    assert!((validator_apy.apy - 0.01977).abs() < 0.01);
}
