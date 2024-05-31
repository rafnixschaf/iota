// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { useIOTAClient } from '@iota/dapp-kit';
import { IOTAClient } from '@iota/iota.js/client';
import { useMemo } from 'react';

import { useNetwork } from '~/context';
import { Network } from '~/utils/api/DefaultRpcClient';

// TODO: Use enhanced RPC locally by default
export function useEnhancedRpcClient() {
    const [network] = useNetwork();
    const client = useIOTAClient();
    const enhancedRpc = useMemo(() => {
        if (network === Network.Local) {
            return new IOTAClient({ url: 'http://localhost:9124' });
        }

        return client;
    }, [network, client]);

    return enhancedRpc;
}
