// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { useIOTAClient } from '@iota/dapp-kit';
import { useQuery } from '@tanstack/react-query';

export function useGetNetworkMetrics() {
    const client = useIOTAClient();
    return useQuery({
        queryKey: ['home', 'metrics'],
        queryFn: () => client.getNetworkMetrics(),
        gcTime: 24 * 60 * 60 * 1000,
        staleTime: Infinity,
        retry: 5,
    });
}
