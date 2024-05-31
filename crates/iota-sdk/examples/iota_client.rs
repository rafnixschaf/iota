// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::IOTAClientBuilder;

// This example shows the few basic ways to connect to a IOTA network.
// There are several in-built methods for connecting to the
// IOTA devnet, tesnet, and localnet (running locally),
// as well as a custom way for connecting to custom URLs.
// The example prints out the API versions of the different networks,
// and finally, it prints the list of available RPC methods
// and the list of subscriptions.
// Note that running this code will fail if there is no IOTA network
// running locally on the default address: 127.0.0.1:9000

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let iota = IOTAClientBuilder::default()
        .build("http://127.0.0.1:9000") // local network address
        .await?;
    println!("IOTA local network version: {}", iota.api_version());

    // local IOTA network, like the above one but using the dedicated function
    let iota_local = IOTAClientBuilder::default().build_localnet().await?;
    println!("IOTA local network version: {}", iota_local.api_version());

    // IOTA devnet -- https://fullnode.devnet.iota.io:443
    let iota_devnet = IOTAClientBuilder::default().build_devnet().await?;
    println!("IOTA devnet version: {}", iota_devnet.api_version());

    // IOTA testnet -- https://fullnode.testnet.iota.io:443
    let iota_testnet = IOTAClientBuilder::default().build_testnet().await?;
    println!("IOTA testnet version: {}", iota_testnet.api_version());

    println!("{:?}", iota_local.available_rpc_methods());
    println!("{:?}", iota_local.available_subscriptions());

    Ok(())
}
