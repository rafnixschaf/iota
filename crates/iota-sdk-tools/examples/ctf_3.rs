// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use iota_sdk::{IotaClientBuilder, IOTA_LOCAL_NETWORK_URL};
use iota_sdk_tools::{
    query::{Query, Queryable as _},
    Client,
};
use iota_types::base_types::IotaAddress;
use tracing_subscriber::{fmt::format::FmtSpan, prelude::*, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    set_up_logging()?;

    let mut args = std::env::args().skip(1);
    let address = args.next().expect("no address specified");
    let network = args
        .next()
        .unwrap_or_else(|| IOTA_LOCAL_NETWORK_URL.to_owned());

    let client = Client::new(
        IotaClientBuilder::default().build(network).await?,
        IotaAddress::from_str(&address)?,
    )?;

    // Publish a module
    let (published, txn) = client
        .publish("crates/iota-sdk-tools/examples/challenge_3")
        .execute()
        .await?;
    let counter = txn
        .created
        .first_matching(Query::new().type_name("Counter").shared())?
        .object_id;
    let capability = txn
        .created
        .first_matching(Query::new().treasury_cap(true).shared())?
        .object_id;

    let mint_coin = || async {
        client
            .call_move(published.package_id, "mintcoin", "mint_coin")
            .params(capability)
            .execute()
            .await
            .and_then(|res| {
                Ok(res
                    .created
                    .first_matching(Query::new().coin(true))
                    .map_err(|e| anyhow::anyhow!(e))?
                    .object_id)
            })
    };
    // Call mint three times
    let coin_1 = mint_coin().await?;
    let coin_2 = mint_coin().await?;
    let coin_3 = mint_coin().await?;

    // Merge the coins into coin 1
    client.merge_coins(coin_1, coin_2).execute().await?.digest;

    client.merge_coins(coin_1, coin_3).execute().await?.digest;

    // Split coin 1 so it has the correct value of 5
    client.split_coin(coin_1, [1]).execute().await?.digest;

    let digest = client
        .call_move(published.package_id, "mintcoin", "get_flag")
        .params((counter, coin_1))
        .execute()
        .await?
        .digest;
    println!("Challenge 3: {digest:#?}");
    Ok(())
}

fn set_up_logging() -> anyhow::Result<()> {
    std::panic::set_hook(Box::new(|p| {
        tracing::error!("{}", p);
    }));

    let registry = tracing_subscriber::registry();

    let registry = {
        registry
            .with(EnvFilter::from_default_env())
            .with(tracing_subscriber::fmt::layer().with_span_events(FmtSpan::CLOSE))
    };

    registry.init();
    Ok(())
}
