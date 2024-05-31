// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { type SignedTransaction } from '_src/ui/app/WalletSigner';
import type { IOTATransactionBlockResponse } from '@iota/iota.js/client';
import {
    type IOTASignAndExecuteTransactionBlockInput,
    type IOTASignMessageOutput,
} from '@iota/wallet-standard';

export type TransactionDataType = {
    type: 'transaction';
    data: string;
    account: string;
    justSign?: boolean;
    requestType?: IOTASignAndExecuteTransactionBlockInput['requestType'];
    options?: IOTASignAndExecuteTransactionBlockInput['options'];
};

export type SignMessageDataType = {
    type: 'sign-message';
    message: string;
    accountAddress: string;
};

export type ApprovalRequest = {
    id: string;
    approved: boolean | null;
    origin: string;
    originFavIcon?: string;
    txResult?: IOTATransactionBlockResponse | IOTASignMessageOutput;
    txResultError?: string;
    txSigned?: SignedTransaction;
    createdDate: string;
    tx: TransactionDataType | SignMessageDataType;
};

export interface SignMessageApprovalRequest extends Omit<ApprovalRequest, 'txResult' | 'tx'> {
    tx: SignMessageDataType;
    txResult?: IOTASignMessageOutput;
}

export interface TransactionApprovalRequest extends Omit<ApprovalRequest, 'txResult' | 'tx'> {
    tx: TransactionDataType;
    txResult?: IOTATransactionBlockResponse;
}

export function isSignMessageApprovalRequest(
    request: ApprovalRequest,
): request is SignMessageApprovalRequest {
    return request.tx.type === 'sign-message';
}

export function isTransactionApprovalRequest(
    request: ApprovalRequest,
): request is TransactionApprovalRequest {
    return request.tx.type !== 'sign-message';
}
