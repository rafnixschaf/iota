// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { UseMutationOptions, UseMutationResult } from '@tanstack/react-query';
import { useMutation } from '@tanstack/react-query';

import { useIOTAClientContext } from './useIOTAClient.js';
import type { IOTARpcMethods } from './useIOTAClientQuery.js';

export type UseIOTAClientMutationOptions<T extends keyof IOTARpcMethods> = Omit<
	UseMutationOptions<IOTARpcMethods[T]['result'], Error, IOTARpcMethods[T]['params'], unknown[]>,
	'mutationFn'
>;

export function useIOTAClientMutation<T extends keyof IOTARpcMethods>(
	method: T,
	options: UseIOTAClientMutationOptions<T> = {},
): UseMutationResult<IOTARpcMethods[T]['result'], Error, IOTARpcMethods[T]['params'], unknown[]> {
	const iotaContext = useIOTAClientContext();

	return useMutation({
		...options,
		mutationFn: async (params) => {
			return await iotaContext.client[method](params as never);
		},
	});
}
