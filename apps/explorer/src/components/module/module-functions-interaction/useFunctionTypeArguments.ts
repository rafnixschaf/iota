// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { useMemo } from 'react';

import type { IOTAMoveAbilitySet } from '@iota/iota.js/client';

export function useFunctionTypeArguments(typeArguments: IOTAMoveAbilitySet[]) {
    return useMemo(
        () =>
            typeArguments.map(
                (aTypeArgument, index) =>
                    `T${index}${
                        aTypeArgument.abilities.length
                            ? `: ${aTypeArgument.abilities.join(' + ')}`
                            : ''
                    }`,
            ),
        [typeArguments],
    );
}
