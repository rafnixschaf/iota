// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { registerWallet } from '@iota/wallet-standard';

import { IOTAWallet } from './WalletStandardInterface';

registerWallet(new IOTAWallet());
