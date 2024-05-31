// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import {
	DisplayFieldsResponse,
	IOTAObjectChange,
	IOTAObjectChangeCreated,
	IOTAObjectChangeDeleted,
	IOTAObjectChangeMutated,
	IOTAObjectChangePublished,
	IOTAObjectChangeTransferred,
	IOTAObjectChangeWrapped,
} from '@iota/iota.js/client';

import { groupByOwner } from './groupByOwner';
import { IOTAObjectChangeTypes } from './types';

export type WithDisplayFields<T> = T & { display?: DisplayFieldsResponse };
export type IOTAObjectChangeWithDisplay = WithDisplayFields<IOTAObjectChange>;

export type ObjectChanges = {
	changesWithDisplay: IOTAObjectChangeWithDisplay[];
	changes: IOTAObjectChange[];
	ownerType: string;
};
export type ObjectChangesByOwner = Record<string, ObjectChanges>;

export type ObjectChangeSummary = {
	[K in IOTAObjectChangeTypes]: ObjectChangesByOwner;
};

export const getObjectChangeSummary = (objectChanges: IOTAObjectChangeWithDisplay[]) => {
	if (!objectChanges) return null;

	const mutated = objectChanges.filter(
		(change) => change.type === 'mutated',
	) as IOTAObjectChangeMutated[];

	const created = objectChanges.filter(
		(change) => change.type === 'created',
	) as IOTAObjectChangeCreated[];

	const transferred = objectChanges.filter(
		(change) => change.type === 'transferred',
	) as IOTAObjectChangeTransferred[];

	const published = objectChanges.filter(
		(change) => change.type === 'published',
	) as IOTAObjectChangePublished[];

	const wrapped = objectChanges.filter(
		(change) => change.type === 'wrapped',
	) as IOTAObjectChangeWrapped[];

	const deleted = objectChanges.filter(
		(change) => change.type === 'deleted',
	) as IOTAObjectChangeDeleted[];

	return {
		transferred: groupByOwner(transferred),
		created: groupByOwner(created),
		mutated: groupByOwner(mutated),
		published: groupByOwner(published),
		wrapped: groupByOwner(wrapped),
		deleted: groupByOwner(deleted),
	};
};
