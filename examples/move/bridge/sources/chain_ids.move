// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

module bridge::chain_ids {

    use std::vector;

    // Chain IDs
    const IOTAMainnet: u8 = 0;
    const IOTATestnet: u8 = 1;
    const IOTADevnet: u8 = 2;
    const IOTALocalTest: u8 = 3;

    const EthMainnet: u8 = 10;
    const EthSepolia: u8 = 11;
    const EthLocalTest: u8 = 12;

    struct BridgeRoute has drop {
        source: u8,
        destination: u8,
    }

    public fun iota_mainnet(): u8 {
        IOTAMainnet
    }

    public fun iota_testnet(): u8 {
        IOTATestnet
    }

    public fun iota_devnet(): u8 {
        IOTADevnet
    }

    public fun iota_local_test(): u8 {
        IOTALocalTest
    }

    public fun eth_mainnet(): u8 {
        EthMainnet
    }

    public fun eth_sepolia(): u8 {
        EthSepolia
    }

    public fun eth_local_test(): u8 {
        EthLocalTest
    }

    public fun valid_routes(): vector<BridgeRoute> {
        vector[
            BridgeRoute { source: IOTAMainnet, destination: EthMainnet },
            BridgeRoute { source: IOTADevnet, destination: EthSepolia },
            BridgeRoute { source: IOTATestnet, destination: EthSepolia },
            BridgeRoute { source: IOTALocalTest, destination: EthLocalTest },
            BridgeRoute { source: EthMainnet, destination: IOTAMainnet },
            BridgeRoute { source: EthSepolia, destination: IOTADevnet },
            BridgeRoute { source: EthSepolia, destination: IOTATestnet },
            BridgeRoute { source: EthLocalTest, destination: IOTALocalTest }]
    }

    public fun is_valid_route(source: u8, destination: u8): bool {
        let route = BridgeRoute { source, destination };
        return vector::contains(&valid_routes(), &route)
    }
}
