// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { type SerializedUIAccount } from '_src/background/accounts/Account';
import { isLedgerAccountSerializedUI } from '_src/background/accounts/LedgerAccount';
import { useIOTAClient } from '@iota/dapp-kit';

import { walletApiProvider } from '../ApiProvider';
import { useIOTALedgerClient } from '../components/ledger/IOTALedgerClientProvider';
import { LedgerSigner } from '../LedgerSigner';
import { type WalletSigner } from '../WalletSigner';
import { useBackgroundClient } from './useBackgroundClient';

export function useSigner(account: SerializedUIAccount | null): WalletSigner | null {
    const { connectToLedger } = useIOTALedgerClient();
    const api = useIOTAClient();
    const background = useBackgroundClient();
    if (!account) {
        return null;
    }
    if (isLedgerAccountSerializedUI(account)) {
        return new LedgerSigner(connectToLedger, account.derivationPath, api);
    }
    return walletApiProvider.getSignerInstance(account, background);
}
