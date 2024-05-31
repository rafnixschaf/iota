// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SentryHttpTransport } from '@iota/core';
import {
    IOTAClient,
    IOTAHTTPTransport,
    getNetwork,
    Network,
    type NetworkId,
    getAllNetworks,
} from '@iota/iota.js/client';

export { Network } from '@iota/iota.js/client';

export const SupportedNetworks = getAllNetworks();
// The Explorer always shows the Custom RPC input so there is no need to confuse it more by having a Custom Network here
delete SupportedNetworks[Network.Custom];

const defaultClientMap: Map<NetworkId, IOTAClient> = new Map();

// NOTE: This class should not be used directly in React components, prefer to use the useIOTAClient() hook instead
export const createIOTAClient = (network: NetworkId) => {
    const existingClient = defaultClientMap.get(network);
    if (existingClient) return existingClient;

    const supportedNetwork = getNetwork(network);
    // If network is not supported, we use assume we are using a custom RPC
    const networkUrl = supportedNetwork?.url ?? network;

    const client = new IOTAClient({
        transport:
            supportedNetwork && network === Network.Mainnet
                ? new SentryHttpTransport(networkUrl)
                : new IOTAHTTPTransport({ url: networkUrl }),
    });
    defaultClientMap.set(network, client);
    return client;
};
