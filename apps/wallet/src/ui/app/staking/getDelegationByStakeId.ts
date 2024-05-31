// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type { DelegatedStake } from '@iota/iota.js/client';

// Helper function to get the delegation by stakedIOTAId
export const getDelegationDataByStakeId = (
    delegationsStake: DelegatedStake[],
    stakeIOTAId: string,
) => {
    let stake = null;
    for (const { stakes } of delegationsStake) {
        stake = stakes.find(({ stakedIOTAId }) => stakedIOTAId === stakeIOTAId) || null;
        if (stake) return stake;
    }

    return stake;
};
