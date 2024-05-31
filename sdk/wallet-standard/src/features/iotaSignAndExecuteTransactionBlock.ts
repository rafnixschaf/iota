// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type {
	ExecuteTransactionRequestType,
	IOTATransactionBlockResponse,
	IOTATransactionBlockResponseOptions,
} from '@iota/iota.js/client';

import type { IOTASignTransactionBlockInput } from './iotaSignTransactionBlock.js';

/** The latest API version of the signAndExecuteTransactionBlock API. */
export type IOTASignAndExecuteTransactionBlockVersion = '1.0.0';

/**
 * A Wallet Standard feature for signing a transaction, and submitting it to the
 * network. The wallet is expected to submit the transaction to the network via RPC,
 * and return the transaction response.
 */
export type IOTASignAndExecuteTransactionBlockFeature = {
	/** Namespace for the feature. */
	'iota:signAndExecuteTransactionBlock': {
		/** Version of the feature API. */
		version: IOTASignAndExecuteTransactionBlockVersion;
		signAndExecuteTransactionBlock: IOTASignAndExecuteTransactionBlockMethod;
	};
};

export type IOTASignAndExecuteTransactionBlockMethod = (
	input: IOTASignAndExecuteTransactionBlockInput,
) => Promise<IOTASignAndExecuteTransactionBlockOutput>;

/** Input for signing and sending transactions. */
export interface IOTASignAndExecuteTransactionBlockInput extends IOTASignTransactionBlockInput {
	/**
	 * `WaitForEffectsCert` or `WaitForLocalExecution`, see details in `ExecuteTransactionRequestType`.
	 * Defaults to `WaitForLocalExecution` if options.showEffects or options.showEvents is true
	 */
	requestType?: ExecuteTransactionRequestType;
	/** specify which fields to return (e.g., transaction, effects, events, etc). By default, only the transaction digest will be returned. */
	options?: IOTATransactionBlockResponseOptions;
}

/** Output of signing and sending transactions. */
export interface IOTASignAndExecuteTransactionBlockOutput extends IOTATransactionBlockResponse {}
