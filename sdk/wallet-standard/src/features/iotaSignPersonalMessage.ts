// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { WalletAccount } from '@wallet-standard/core';

/** The latest API version of the signPersonalMessage API. */
export type IOTASignPersonalMessageVersion = '1.0.0';

/**
 * A Wallet Standard feature for signing a personal message, and returning the
 * message bytes that were signed, and message signature.
 */
export type IOTASignPersonalMessageFeature = {
	/** Namespace for the feature. */
	'iota:signPersonalMessage': {
		/** Version of the feature API. */
		version: IOTASignPersonalMessageVersion;
		signPersonalMessage: IOTASignPersonalMessageMethod;
	};
};

export type IOTASignPersonalMessageMethod = (
	input: IOTASignPersonalMessageInput,
) => Promise<IOTASignPersonalMessageOutput>;

/** Input for signing personal messages. */
export interface IOTASignPersonalMessageInput {
	message: Uint8Array;
	account: WalletAccount;
}

/** Output of signing personal messages. */
export interface IOTASignPersonalMessageOutput extends SignedPersonalMessage {}

export interface SignedPersonalMessage {
	bytes: string;
	signature: string;
}
