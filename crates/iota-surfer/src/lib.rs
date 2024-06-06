// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{path::PathBuf, sync::Arc, time::Duration};

use futures::future::join_all;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use iota_swarm_config::genesis_config::{AccountConfig, DEFAULT_GAS_AMOUNT};
use surf_strategy::SurfStrategy;
use test_cluster::TestClusterBuilder;
use tokio::sync::watch;
use tracing::info;

use crate::{surfer_state::SurfStatistics, surfer_task::SurferTask};

pub mod default_surf_strategy;
mod surf_strategy;
mod surfer_state;
mod surfer_task;

const VALIDATOR_COUNT: usize = 7;

const ACCOUNT_NUM: usize = 20;
const GAS_OBJECT_COUNT: usize = 3;

pub async fn run<S: SurfStrategy + Default>(
    run_duration: Duration,
    epoch_duration: Duration,
    package_paths: Vec<PathBuf>,
) -> SurfStatistics {
    let cluster = Arc::new(
        TestClusterBuilder::new()
            .with_num_validators(VALIDATOR_COUNT)
            .with_epoch_duration_ms(epoch_duration.as_millis() as u64)
            .with_accounts(vec![
                AccountConfig {
                    address: None,
                    gas_amounts: vec![DEFAULT_GAS_AMOUNT; GAS_OBJECT_COUNT],
                };
                ACCOUNT_NUM
            ])
            .build()
            .await,
    );
    info!(
        "Started cluster with {} validators and epoch duration of {:?}ms",
        VALIDATOR_COUNT,
        epoch_duration.as_millis()
    );

    let seed = rand::thread_rng().gen::<u64>();
    info!("Initial Seed: {:?}", seed);
    let mut rng = StdRng::seed_from_u64(seed);
    let (exit_sender, exit_rcv) = watch::channel(());

    let mut tasks =
        SurferTask::create_surfer_tasks::<S>(cluster.clone(), rng.gen::<u64>(), exit_rcv).await;
    info!("Created {} surfer tasks", tasks.len());

    for path in package_paths {
        tasks
            .choose_mut(&mut rng)
            .unwrap()
            .state
            .publish_package(path)
            .await;
    }

    let mut handles = vec![];
    for task in tasks {
        handles.push(tokio::task::spawn(task.surf()));
    }
    tokio::time::sleep(run_duration).await;
    exit_sender.send(()).unwrap();
    let all_stats: Result<Vec<_>, _> = join_all(handles).await.into_iter().collect();
    SurfStatistics::aggregate(all_stats.unwrap())

    // TODO: Right now it will panic here complaining about dropping a tokio
    // runtime inside of another tokio runtime. Reason unclear.
}
