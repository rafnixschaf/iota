// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { type DelegatedStake } from '@iota/iota.js/client';

// Get Stake IOTA by stakeIOTAId
export const getStakeIOTAByIOTAId = (allDelegation: DelegatedStake[], stakeIOTAId?: string | null) => {
    return (
        allDelegation.reduce((acc, curr) => {
            const total = BigInt(
                curr.stakes.find(({ stakedIOTAId }) => stakedIOTAId === stakeIOTAId)?.principal || 0,
            );
            return total + acc;
        }, 0n) || 0n
    );
};
