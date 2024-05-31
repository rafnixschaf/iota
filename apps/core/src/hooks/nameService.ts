// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { useFeatureIsOn } from '@growthbook/growthbook-react';
import { useIOTAClient } from '@iota/dapp-kit';
import { useQuery } from '@tanstack/react-query';

const IOTA_NS_FEATURE_FLAG = 'iotans';

// This should align with whatever names we want to be able to resolve.
const IOTA_NS_DOMAINS = ['.iota'];
export function isIOTANSName(name: string) {
	return IOTA_NS_DOMAINS.some((domain) => name.endsWith(domain));
}

export function useIOTANSEnabled() {
	return useFeatureIsOn(IOTA_NS_FEATURE_FLAG);
}

export function useResolveIOTANSAddress(name?: string | null, enabled?: boolean) {
	const client = useIOTAClient();
	const enabledIOTANs = useIOTANSEnabled();

	return useQuery({
		queryKey: ['resolve-iotans-address', name],
		queryFn: async () => {
			return await client.resolveNameServiceAddress({
				name: name!,
			});
		},
		enabled: !!name && enabled && enabledIOTANs,
		refetchOnWindowFocus: false,
		retry: false,
	});
}

export function useResolveIOTANSName(address?: string | null) {
	const client = useIOTAClient();
	const enabled = useIOTANSEnabled();

	return useQuery({
		queryKey: ['resolve-iotans-name', address],
		queryFn: async () => {
			// NOTE: We only fetch 1 here because it's the default name.
			const { data } = await client.resolveNameServiceNames({
				address: address!,
				limit: 1,
			});

			return data[0] || null;
		},
		enabled: !!address && enabled,
		refetchOnWindowFocus: false,
		retry: false,
	});
}
