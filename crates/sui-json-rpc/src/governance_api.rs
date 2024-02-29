// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use sui_types::sui_serde::BigInt;
use tracing::instrument;

use sui_json_rpc_api::{GovernanceReadApiServer, JsonRpcMetrics};
use sui_json_rpc_types::DelegatedStake;
use sui_json_rpc_types::{SuiCommittee, ValidatorApys};
use sui_types::base_types::{ObjectID, SuiAddress};
use sui_types::sui_system_state::sui_system_state_summary::SuiSystemStateSummary;

#[derive(Clone)]
pub struct GovernanceReadApi {
    pub metrics: Arc<JsonRpcMetrics>,
}

impl GovernanceReadApi {
    pub fn new(metrics: Arc<JsonRpcMetrics>) -> Self {
        Self { metrics }
    }
}

#[async_trait]
impl GovernanceReadApiServer for GovernanceReadApi {
    #[instrument(skip(self))]
    async fn get_stakes_by_ids(
        &self,
        _staked_sui_ids: Vec<ObjectID>,
    ) -> RpcResult<Vec<DelegatedStake>> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn get_stakes(&self, _owner: SuiAddress) -> RpcResult<Vec<DelegatedStake>> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn get_committee_info(&self, _epoch: Option<BigInt<u64>>) -> RpcResult<SuiCommittee> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn get_latest_sui_system_state(&self) -> RpcResult<SuiSystemStateSummary> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn get_reference_gas_price(&self) -> RpcResult<BigInt<u64>> {
        unimplemented!()
    }

    #[instrument(skip(self))]
    async fn get_validators_apy(&self) -> RpcResult<ValidatorApys> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use itertools::Itertools;
    use jsonrpsee::RpcModule;
    use sui_json_rpc_api::GovernanceReadApiOpenRpc;
    use sui_json_rpc_types::ValidatorApy;
    use sui_open_rpc::Module;
    use sui_types::committee::EpochId;
    use sui_types::sui_system_state::PoolTokenExchangeRate;

    use crate::SuiRpcModule;

    use super::*;

    #[derive(Clone, Debug)]
    pub struct ValidatorExchangeRates {
        pub address: SuiAddress,
        pub pool_id: ObjectID,
        pub active: bool,
        pub rates: Vec<(EpochId, PoolTokenExchangeRate)>,
    }

    impl SuiRpcModule for GovernanceReadApi {
        fn rpc(self) -> RpcModule<Self> {
            self.into_rpc()
        }

        fn rpc_doc_module() -> Module {
            GovernanceReadApiOpenRpc::module_doc()
        }
    }

    // APY_e = (ER_e+1 / ER_e) ^ 365
    fn calculate_apy((rate_e, rate_e_1): (PoolTokenExchangeRate, PoolTokenExchangeRate)) -> f64 {
        (rate_e.rate() / rate_e_1.rate()).powf(365.0) - 1.0
    }

    fn calculate_apys(
        stake_subsidy_start_epoch: u64,
        exchange_rate_table: Vec<ValidatorExchangeRates>,
    ) -> Vec<ValidatorApy> {
        let mut apys = vec![];

        for rates in exchange_rate_table.into_iter().filter(|r| r.active) {
            // we start the apy calculation from the epoch when the stake subsidy starts
            let exchange_rates = rates.rates.into_iter().filter_map(|(epoch, rate)| {
                if epoch >= stake_subsidy_start_epoch {
                    Some(rate)
                } else {
                    None
                }
            });

            // we need at least 2 data points to calculate apy
            let average_apy = if exchange_rates.clone().count() >= 2 {
                // rates are sorted by epoch in descending order.
                let er_e = exchange_rates.clone().dropping(1);
                // rate e+1
                let er_e_1 = exchange_rates.dropping_back(1);
                let apys = er_e
                    .zip(er_e_1)
                    .map(calculate_apy)
                    .filter(|apy| *apy > 0.0 && *apy < 0.1)
                    .take(30)
                    .collect::<Vec<_>>();

                let apy_counts = apys.len() as f64;
                apys.iter().sum::<f64>() / apy_counts
            } else {
                0.0
            };
            apys.push(ValidatorApy {
                address: rates.address,
                apy: average_apy,
            });
        }
        apys
    }

    #[test]
    fn test_apys_calculation_filter_outliers() {
        // staking pool exchange rates extracted from mainnet
        let file =
            std::fs::File::open("src/unit_tests/data/validator_exchange_rate/rates.json").unwrap();
        let rates: BTreeMap<String, Vec<(u64, PoolTokenExchangeRate)>> =
            serde_json::from_reader(file).unwrap();

        let mut address_map = BTreeMap::new();

        let exchange_rates = rates
            .into_iter()
            .map(|(validator, rates)| {
                let address = SuiAddress::random_for_testing_only();
                address_map.insert(address, validator);
                ValidatorExchangeRates {
                    address,
                    pool_id: ObjectID::random(),
                    active: true,
                    rates,
                }
            })
            .collect();

        let apys = calculate_apys(20, exchange_rates);

        for apy in apys {
            println!("{}: {}", address_map[&apy.address], apy.apy);
            assert!(apy.apy < 0.07)
        }
    }
}
