// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { WalletAccount } from '@wallet-standard/core';

/**
 * The latest API version of the signMessage API.
 * @deprecated Wallets can still implement this method for compatibility, but this has been replaced by the `iota:signPersonalMessage` feature
 */
export type IOTASignMessageVersion = '1.0.0';

/**
 * A Wallet Standard feature for signing a personal message, and returning the
 * message bytes that were signed, and message signature.
 *
 * @deprecated Wallets can still implement this method for compatibility, but this has been replaced by the `iota:signPersonalMessage` feature
 */
export type IOTASignMessageFeature = {
	/** Namespace for the feature. */
	'iota:signMessage': {
		/** Version of the feature API. */
		version: IOTASignMessageVersion;
		signMessage: IOTASignMessageMethod;
	};
};

/** @deprecated Wallets can still implement this method for compatibility, but this has been replaced by the `iota:signPersonalMessage` feature */
export type IOTASignMessageMethod = (input: IOTASignMessageInput) => Promise<IOTASignMessageOutput>;

/**
 * Input for signing messages.
 * @deprecated Wallets can still implement this method for compatibility, but this has been replaced by the `iota:signPersonalMessage` feature
 */
export interface IOTASignMessageInput {
	message: Uint8Array;
	account: WalletAccount;
}

/**
 * Output of signing messages.
 * @deprecated Wallets can still implement this method for compatibility, but this has been replaced by the `iota:signPersonalMessage` feature
 */
export interface IOTASignMessageOutput {
	messageBytes: string;
	signature: string;
}
