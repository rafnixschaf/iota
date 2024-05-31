// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { IOTAClient } from '@iota/iota.js/client';
import type {
	InfiniteData,
	UseInfiniteQueryOptions,
	UseInfiniteQueryResult,
} from '@tanstack/react-query';
import { useInfiniteQuery } from '@tanstack/react-query';

import type { PartialBy } from '../types/utilityTypes.js';
import { useIOTAClientContext } from './useIOTAClient.js';

interface PaginatedResult {
	data?: unknown;
	nextCursor?: unknown;
	hasNextPage: boolean;
}

export type IOTARpcPaginatedMethodName = {
	[K in keyof IOTAClient]: IOTAClient[K] extends (input: any) => Promise<PaginatedResult> ? K : never;
}[keyof IOTAClient];

export type IOTARpcPaginatedMethods = {
	[K in IOTARpcPaginatedMethodName]: IOTAClient[K] extends (
		input: infer Params,
	) => Promise<
		infer Result extends { hasNextPage?: boolean | null; nextCursor?: infer Cursor | null }
	>
		? {
				name: K;
				result: Result;
				params: Params;
				cursor: Cursor;
		  }
		: never;
};

export type UseIOTAClientInfiniteQueryOptions<
	T extends keyof IOTARpcPaginatedMethods,
	TData,
> = PartialBy<
	Omit<
		UseInfiniteQueryOptions<
			IOTARpcPaginatedMethods[T]['result'],
			Error,
			TData,
			IOTARpcPaginatedMethods[T]['result'],
			unknown[]
		>,
		'queryFn' | 'initialPageParam' | 'getNextPageParam'
	>,
	'queryKey'
>;

export function useIOTAClientInfiniteQuery<
	T extends keyof IOTARpcPaginatedMethods,
	TData = InfiniteData<IOTARpcPaginatedMethods[T]['result']>,
>(
	method: T,
	params: IOTARpcPaginatedMethods[T]['params'],
	{
		queryKey = [],
		enabled = !!params,
		...options
	}: UseIOTAClientInfiniteQueryOptions<T, TData> = {},
): UseInfiniteQueryResult<TData, Error> {
	const iotaContext = useIOTAClientContext();

	return useInfiniteQuery({
		...options,
		initialPageParam: null,
		queryKey: [iotaContext.network, method, params, ...queryKey],
		enabled,
		queryFn: ({ pageParam }) =>
			iotaContext.client[method]({
				...(params ?? {}),
				cursor: pageParam,
			} as never),
		getNextPageParam: (lastPage) => (lastPage.hasNextPage ? lastPage.nextCursor ?? null : null),
	});
}
