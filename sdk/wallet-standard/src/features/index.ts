// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type {
	IdentifierRecord,
	StandardConnectFeature,
	StandardDisconnectFeature,
	StandardEventsFeature,
	WalletWithFeatures,
} from '@wallet-standard/core';

import type { IOTASignAndExecuteTransactionBlockFeature } from './iotaSignAndExecuteTransactionBlock.js';
import type { IOTASignMessageFeature } from './iotaSignMessage.js';
import type { IOTASignPersonalMessageFeature } from './iotaSignPersonalMessage.js';
import type { IOTASignTransactionBlockFeature } from './iotaSignTransactionBlock.js';

/**
 * Wallet Standard features that are unique to IOTA, and that all IOTA wallets are expected to implement.
 */
export type IOTAFeatures = IOTASignTransactionBlockFeature &
	IOTASignAndExecuteTransactionBlockFeature &
	IOTASignPersonalMessageFeature &
	// This deprecated feature should be removed once wallets update to the new method:
	Partial<IOTASignMessageFeature>;

export type WalletWithIOTAFeatures = WalletWithFeatures<
	StandardConnectFeature &
		StandardEventsFeature &
		IOTAFeatures &
		// Disconnect is an optional feature:
		Partial<StandardDisconnectFeature>
>;

/**
 * Represents a wallet with the absolute minimum feature set required to function in the IOTA ecosystem.
 */
export type WalletWithRequiredFeatures = WalletWithFeatures<
	MinimallyRequiredFeatures &
		Partial<IOTAFeatures> &
		Partial<StandardDisconnectFeature> &
		IdentifierRecord<unknown>
>;

export type MinimallyRequiredFeatures = StandardConnectFeature & StandardEventsFeature;

export * from './iotaSignMessage.js';
export * from './iotaSignTransactionBlock.js';
export * from './iotaSignAndExecuteTransactionBlock.js';
export * from './iotaSignPersonalMessage.js';
