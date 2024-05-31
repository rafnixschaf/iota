// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { IOTAClient } from '@iota/iota.js/client';
import type { UseQueryOptions, UseQueryResult } from '@tanstack/react-query';
import { useQuery } from '@tanstack/react-query';

import type { PartialBy } from '../types/utilityTypes.js';
import { useIOTAClientContext } from './useIOTAClient.js';

export type IOTARpcMethodName = {
	[K in keyof IOTAClient]: IOTAClient[K] extends ((input: any) => Promise<any>) | (() => Promise<any>)
		? K
		: never;
}[keyof IOTAClient];

export type IOTARpcMethods = {
	[K in IOTARpcMethodName]: IOTAClient[K] extends (input: infer P) => Promise<infer R>
		? {
				name: K;
				result: R;
				params: P;
		  }
		: IOTAClient[K] extends () => Promise<infer R>
		? {
				name: K;
				result: R;
				params: undefined | object;
		  }
		: never;
};

export type UseIOTAClientQueryOptions<T extends keyof IOTARpcMethods, TData> = PartialBy<
	Omit<UseQueryOptions<IOTARpcMethods[T]['result'], Error, TData, unknown[]>, 'queryFn'>,
	'queryKey'
>;

export function useIOTAClientQuery<
	T extends keyof IOTARpcMethods,
	TData = IOTARpcMethods[T]['result'],
>(
	...args: undefined extends IOTARpcMethods[T]['params']
		? [method: T, params?: IOTARpcMethods[T]['params'], options?: UseIOTAClientQueryOptions<T, TData>]
		: [method: T, params: IOTARpcMethods[T]['params'], options?: UseIOTAClientQueryOptions<T, TData>]
): UseQueryResult<TData, Error> {
	const [method, params, { queryKey = [], ...options } = {}] = args as [
		method: T,
		params?: IOTARpcMethods[T]['params'],
		options?: UseIOTAClientQueryOptions<T, TData>,
	];

	const iotaContext = useIOTAClientContext();

	return useQuery({
		...options,
		queryKey: [iotaContext.network, method, params, ...queryKey],
		queryFn: async () => {
			return await iotaContext.client[method](params as never);
		},
	});
}
