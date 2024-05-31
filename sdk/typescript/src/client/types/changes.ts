// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { IOTAObjectChange } from './generated.js';

export type IOTAObjectChangePublished = Extract<IOTAObjectChange, { type: 'published' }>;
export type IOTAObjectChangeTransferred = Extract<IOTAObjectChange, { type: 'transferred' }>;
export type IOTAObjectChangeMutated = Extract<IOTAObjectChange, { type: 'mutated' }>;
export type IOTAObjectChangeDeleted = Extract<IOTAObjectChange, { type: 'deleted' }>;
export type IOTAObjectChangeWrapped = Extract<IOTAObjectChange, { type: 'wrapped' }>;
export type IOTAObjectChangeCreated = Extract<IOTAObjectChange, { type: 'created' }>;
