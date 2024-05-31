// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { formatAddress, formatDigest } from './format.js';
import {
	isValidIOTAAddress,
	isValidIOTAObjectId,
	isValidTransactionDigest,
	normalizeStructTag,
	normalizeIOTAAddress,
	normalizeIOTAObjectId,
	parseStructTag,
	IOTA_ADDRESS_LENGTH,
} from './iota-types.js';

export { fromB64, toB64, fromHEX, toHEX } from '@iota/bcs';
export { is, assert } from 'superstruct';

export {
	formatAddress,
	formatDigest,
	isValidIOTAAddress,
	isValidIOTAObjectId,
	isValidTransactionDigest,
	normalizeStructTag,
	normalizeIOTAAddress,
	normalizeIOTAObjectId,
	parseStructTag,
	IOTA_ADDRESS_LENGTH,
};

export const IOTA_DECIMALS = 9;
export const MICROS_PER_IOTA = BigInt(1000000000);

export const MOVE_STDLIB_ADDRESS = '0x1';
export const IOTA_FRAMEWORK_ADDRESS = '0x2';
export const IOTA_SYSTEM_ADDRESS = '0x3';
export const IOTA_CLOCK_OBJECT_ID = normalizeIOTAObjectId('0x6');
export const IOTA_SYSTEM_MODULE_NAME = 'iota_system';
export const IOTA_TYPE_ARG = `${IOTA_FRAMEWORK_ADDRESS}::iota::IOTA`;
export const IOTA_SYSTEM_STATE_OBJECT_ID: string = normalizeIOTAObjectId('0x5');
