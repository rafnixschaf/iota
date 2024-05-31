// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import networkEnv from '_src/background/NetworkEnv';
import { type NetworkEnvType } from '_src/shared/api-env';
import { SentryHttpTransport } from '@iota/core';
import { getNetwork, Network, IOTAClient, IOTAHTTPTransport } from '@iota/iota.js/client';

const iotaClientPerNetwork = new Map<string, IOTAClient>();
const SENTRY_MONITORED_ENVS = [Network.Custom];

export function getIOTAClient({ network, customRpcUrl }: NetworkEnvType): IOTAClient {
    const key = `${network}_${customRpcUrl}`;
    if (!iotaClientPerNetwork.has(key)) {
        const connection = getNetwork(network)?.url ?? customRpcUrl;
        if (!connection) {
            throw new Error(`API url not found for network ${network} ${customRpcUrl}`);
        }
        iotaClientPerNetwork.set(
            key,
            new IOTAClient({
                transport:
                    !customRpcUrl && SENTRY_MONITORED_ENVS.includes(network)
                        ? new SentryHttpTransport(connection)
                        : new IOTAHTTPTransport({ url: connection }),
            }),
        );
    }
    return iotaClientPerNetwork.get(key)!;
}

export async function getActiveNetworkIOTAClient(): Promise<IOTAClient> {
    return getIOTAClient(await networkEnv.getActiveNetwork());
}
