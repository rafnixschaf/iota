// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { TransactionBlock } from '@iota/iota.js/transactions';
import type { IdentifierString, WalletAccount } from '@wallet-standard/core';

/** The latest API version of the signTransactionBlock API. */
export type IOTASignTransactionBlockVersion = '1.0.0';

/**
 * A Wallet Standard feature for signing a transaction, and returning the
 * serialized transaction and transaction signature.
 */
export type IOTASignTransactionBlockFeature = {
	/** Namespace for the feature. */
	'iota:signTransactionBlock': {
		/** Version of the feature API. */
		version: IOTASignTransactionBlockVersion;
		signTransactionBlock: IOTASignTransactionBlockMethod;
	};
};

export type IOTASignTransactionBlockMethod = (
	input: IOTASignTransactionBlockInput,
) => Promise<IOTASignTransactionBlockOutput>;

/** Input for signing transactions. */
export interface IOTASignTransactionBlockInput {
	transactionBlock: TransactionBlock;
	account: WalletAccount;
	chain: IdentifierString;
}

/** Output of signing transactions. */
export interface IOTASignTransactionBlockOutput extends SignedTransactionBlock {}

export interface SignedTransactionBlock {
	transactionBlockBytes: string;
	signature: string;
}
