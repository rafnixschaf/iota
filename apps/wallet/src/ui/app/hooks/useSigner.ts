// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { type SerializedUIAccount } from '_src/background/accounts/Account';
import { isLedgerAccountSerializedUI } from '_src/background/accounts/LedgerAccount';
import { isQredoAccountSerializedUI } from '_src/background/accounts/QredoAccount';
import { useIotaClient } from '@mysten/dapp-kit';

import { walletApiProvider } from '../ApiProvider';
import { useIotaLedgerClient } from '../components/ledger/IotaLedgerClientProvider';
import { LedgerSigner } from '../LedgerSigner';
import { QredoSigner } from '../QredoSigner';
import { type WalletSigner } from '../WalletSigner';
import useAppSelector from './useAppSelector';
import { useBackgroundClient } from './useBackgroundClient';
import { useQredoAPI } from './useQredoAPI';

export function useSigner(account: SerializedUIAccount | null): WalletSigner | null {
	const { connectToLedger } = useIotaLedgerClient();
	const api = useIotaClient();
	const background = useBackgroundClient();
	const [qredoAPI] = useQredoAPI(
		account && !account?.isLocked && isQredoAccountSerializedUI(account)
			? account.sourceID
			: undefined,
	);
	const network = useAppSelector(({ app: { network } }) => network);
	if (!account) {
		return null;
	}
	if (isLedgerAccountSerializedUI(account)) {
		return new LedgerSigner(connectToLedger, account.derivationPath, api);
	}
	if (isQredoAccountSerializedUI(account)) {
		return qredoAPI ? new QredoSigner(api, account, qredoAPI, network) : null;
	}
	return walletApiProvider.getSignerInstance(account, background);
}
