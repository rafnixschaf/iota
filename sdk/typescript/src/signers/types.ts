// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { SerializedSignature } from '../cryptography/signature.js';

export type SignedTransaction = {
	transactionBlockBytes: string;
	signature: SerializedSignature;
};

export type SignedMessage = {
	messageBytes: string;
	signature: SerializedSignature;
};
