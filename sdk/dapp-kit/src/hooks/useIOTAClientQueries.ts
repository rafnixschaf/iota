// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { UseQueryResult } from '@tanstack/react-query';
import { useQueries } from '@tanstack/react-query';

import { useIOTAClientContext } from './useIOTAClient.js';
import type { IOTARpcMethods, UseIOTAClientQueryOptions } from './useIOTAClientQuery.js';

type IOTAClientQueryOptions = IOTARpcMethods[keyof IOTARpcMethods] extends infer Method
	? Method extends {
			name: infer M extends keyof IOTARpcMethods;
			params?: infer P;
	  }
		? undefined extends P
			? {
					method: M;
					params?: P;
					options?: UseIOTAClientQueryOptions<M, unknown>;
			  }
			: {
					method: M;
					params: P;
					options?: UseIOTAClientQueryOptions<M, unknown>;
			  }
		: never
	: never;

export type UseIOTAClientQueriesResults<Args extends readonly IOTAClientQueryOptions[]> = {
	-readonly [K in keyof Args]: Args[K] extends {
		method: infer M extends keyof IOTARpcMethods;
		readonly options?:
			| {
					select?: (...args: any[]) => infer R;
			  }
			| object;
	}
		? UseQueryResult<unknown extends R ? IOTARpcMethods[M]['result'] : R>
		: never;
};

export function useIOTAClientQueries<
	const Queries extends readonly IOTAClientQueryOptions[],
	Results = UseIOTAClientQueriesResults<Queries>,
>({
	queries,
	combine,
}: {
	queries: Queries;
	combine?: (results: UseIOTAClientQueriesResults<Queries>) => Results;
}): Results {
	const iotaContext = useIOTAClientContext();

	return useQueries({
		combine: combine as never,
		queries: queries.map((query) => {
			const { method, params, options: { queryKey = [], ...restOptions } = {} } = query;

			return {
				...restOptions,
				queryKey: [iotaContext.network, method, params, ...queryKey],
				queryFn: async () => {
					return await iotaContext.client[method](params as never);
				},
			};
		}) as [],
	});
}
