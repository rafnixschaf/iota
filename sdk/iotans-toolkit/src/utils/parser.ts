// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { IOTAMoveObject, IOTAObjectData, IOTAObjectResponse } from '@iota/iota.js/client';
import { normalizeIOTAAddress } from '@iota/iota.js/utils';

export const camelCase = (string: string) => string.replace(/(_\w)/g, (g) => g[1].toUpperCase());

export const parseObjectDataResponse = (response: IOTAObjectResponse | undefined) =>
	((response?.data as IOTAObjectData)?.content as IOTAMoveObject)?.fields as Record<string, any>;

export const parseRegistryResponse = (response: IOTAObjectResponse | undefined): any => {
	const fields = parseObjectDataResponse(response)?.value?.fields || {};

	const object = Object.fromEntries(
		Object.entries({ ...fields }).map(([key, val]) => [camelCase(key), val]),
	);

	if (response?.data?.objectId) {
		object.id = response.data.objectId;
	}

	delete object.data;

	const data = (fields.data?.fields.contents || []).reduce(
		(acc: Record<string, any>, c: Record<string, any>) => {
			const key = c.fields.key;
			const value = c.fields.value;

			return {
				...acc,
				[camelCase(key)]:
					c.type.includes('Address') || key === 'addr' ? normalizeIOTAAddress(value) : value,
			};
		},
		{},
	);

	return { ...object, ...data };
};
