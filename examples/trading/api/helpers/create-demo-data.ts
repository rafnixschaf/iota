// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { TransactionBlock } from '@iota/iota-sdk/transactions';

import { CONFIG } from '../config';
import { ACTIVE_NETWORK, getActiveAddress, signAndExecute } from '../iota-utils';

// a simple example of objects by creating N amount of bears.
const createDemoLockedObjects = async (totalBears: number) => {
	if (totalBears < 5) throw new Error('Please create at least 5 bears to run this script.');
	const txb = new TransactionBlock();
	const toTransfer = [];

	const DEMO_BEAR_TYPE = `${CONFIG.DEMO_CONTRACT.packageId}::demo_bear::DemoBear`;

	for (let i = 0; i < totalBears; i++) {
		const bear = txb.moveCall({
			target: `${CONFIG.DEMO_CONTRACT.packageId}::demo_bear::new`,
			arguments: [txb.pure.string(`A happy bear`)],
		});

		// Let's keep a significant amount of bears to play with escrows.
		if (i < totalBears / 3) {
			toTransfer.push(bear);
			continue;
		}

		const [locked, key] = txb.moveCall({
			target: `${CONFIG.SWAP_CONTRACT.packageId}::lock::lock`,
			arguments: [bear],
			typeArguments: [DEMO_BEAR_TYPE],
		});

		// Let's unlock half of them, to catch some destroy events on our API.
		if (i % 2 === 0) {
			const item = txb.moveCall({
				target: `${CONFIG.SWAP_CONTRACT.packageId}::lock::unlock`,
				arguments: [locked, key],
				typeArguments: [DEMO_BEAR_TYPE],
			});
			toTransfer.push(item);
			continue;
		}

		toTransfer.push(locked);
		toTransfer.push(key);
	}

	txb.transferObjects(toTransfer, txb.pure.address(getActiveAddress()));

	const res = await signAndExecute(txb, ACTIVE_NETWORK);

	if (!res.objectChanges || res.objectChanges.length === 0)
		throw new Error('Something went wrong while creating demo bears & locked objects.');

	console.log('Successfully created demo bears & locked objects.');
};

createDemoLockedObjects(30);
