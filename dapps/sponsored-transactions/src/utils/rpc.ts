// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { getFullnodeUrl, IOTAClient } from '@iota/iota.js/client';

export const client = new IOTAClient({ url: getFullnodeUrl('testnet') });
