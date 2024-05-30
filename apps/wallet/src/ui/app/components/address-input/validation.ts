// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { isIotaNSName, useIotaNSEnabled } from '@mysten/core';
import { useIotaClient } from '@mysten/dapp-kit';
import { type IotaClient } from '@mysten/iota.js/client';
import { isValidIotaAddress } from '@mysten/iota.js/utils';
import { useMemo } from 'react';
import * as Yup from 'yup';

export function createIotaAddressValidation(client: IotaClient, iotaNSEnabled: boolean) {
	const resolveCache = new Map<string, boolean>();

	return Yup.string()
		.ensure()
		.trim()
		.required()
		.test('is-iota-address', 'Invalid address. Please check again.', async (value) => {
			if (iotaNSEnabled && isIotaNSName(value)) {
				if (resolveCache.has(value)) {
					return resolveCache.get(value)!;
				}

				const address = await client.resolveNameServiceAddress({
					name: value,
				});

				resolveCache.set(value, !!address);

				return !!address;
			}

			return isValidIotaAddress(value);
		})
		.label("Recipient's address");
}

export function useIotaAddressValidation() {
	const client = useIotaClient();
	const iotaNSEnabled = useIotaNSEnabled();

	return useMemo(() => {
		return createIotaAddressValidation(client, iotaNSEnabled);
	}, [client, iotaNSEnabled]);
}
