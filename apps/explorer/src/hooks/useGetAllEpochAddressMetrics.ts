// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { useIOTAClient } from '@iota/dapp-kit';
import { type IOTAClient } from '@iota/iota.js/client';
import { useQuery } from '@tanstack/react-query';

export function useGetAllEpochAddressMetrics(
    ...input: Parameters<IOTAClient['getAllEpochAddressMetrics']>
) {
    const client = useIOTAClient();
    return useQuery({
        queryKey: ['get', 'all', 'epoch', 'addresses', ...input],
        queryFn: () => client.getAllEpochAddressMetrics(...input),
    });
}
