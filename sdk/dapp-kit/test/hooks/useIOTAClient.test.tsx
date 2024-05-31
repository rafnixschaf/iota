// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { getFullnodeUrl, IOTAClient } from '@iota/iota.js/client';
import { renderHook } from '@testing-library/react';

import { useIOTAClient } from '../../src/index.js';
import { createIOTAClientContextWrapper } from '../test-utils.js';

describe('useIOTAClient', () => {
	test('throws without a IOTAClientContext', () => {
		expect(() => renderHook(() => useIOTAClient())).toThrowError(
			'Could not find IOTAClientContext. Ensure that you have set up the IOTAClientProvider',
		);
	});

	test('returns a IOTAClient', () => {
		const iotaClient = new IOTAClient({ url: getFullnodeUrl('localnet') });
		const wrapper = createIOTAClientContextWrapper(iotaClient);
		const { result } = renderHook(() => useIOTAClient(), { wrapper });

		expect(result.current).toBe(iotaClient);
	});
});
