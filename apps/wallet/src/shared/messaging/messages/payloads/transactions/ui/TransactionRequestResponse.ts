// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { isBasePayload } from '_payloads';
import type { BasePayload, Payload } from '_payloads';
import { type SignedTransaction } from '_src/ui/app/WalletSigner';
import type { IOTATransactionBlockResponse } from '@iota/iota.js/client';
import { type IOTASignMessageOutput } from '@iota/wallet-standard';

export interface TransactionRequestResponse extends BasePayload {
    type: 'transaction-request-response';
    txID: string;
    approved: boolean;
    txResult?: IOTATransactionBlockResponse | IOTASignMessageOutput;
    txResultError?: string;
    txSigned?: SignedTransaction;
}

export function isTransactionRequestResponse(
    payload: Payload,
): payload is TransactionRequestResponse {
    return isBasePayload(payload) && payload.type === 'transaction-request-response';
}
