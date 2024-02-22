// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import mitt from 'mitt';

type AccountSourcesEvents = {
	accountSourcesChanged: void;
	accountSourceStatusUpdated: { accountSourceID: string };
};

export const accountSourcesEvents = mitt<AccountSourcesEvents>();
