// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { IOTAClient } from '@iota/iota.js/client';
import { useContext } from 'react';

import { IOTAClientContext } from '../components/IOTAClientProvider.js';

export function useIOTAClientContext() {
	const iotaClient = useContext(IOTAClientContext);

	if (!iotaClient) {
		throw new Error(
			'Could not find IOTAClientContext. Ensure that you have set up the IOTAClientProvider',
		);
	}

	return iotaClient;
}

export function useIOTAClient(): IOTAClient {
	return useIOTAClientContext().client;
}
