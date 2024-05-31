// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { IOTAEvent, IOTATransactionBlockKind, TransactionEffects } from '@iota/iota.js/client';

type FormattedBalance = {
    amount?: number | null;
    coinType?: string | null;
    recipientAddress: string;
}[];

export function getAmount(
    _txnData: IOTATransactionBlockKind,
    _txnEffect: TransactionEffects,
    _events: IOTAEvent[],
): FormattedBalance | null {
    // TODO: Support programmable transactions:
    return null;
}
