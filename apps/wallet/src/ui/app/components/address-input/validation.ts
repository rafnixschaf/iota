// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { isIOTANSName, useIOTANSEnabled } from '@iota/core';
import { useIOTAClient } from '@iota/dapp-kit';
import { type IOTAClient } from '@iota/iota.js/client';
import { isValidIOTAAddress } from '@iota/iota.js/utils';
import { useMemo } from 'react';
import * as Yup from 'yup';

export function createIOTAAddressValidation(client: IOTAClient, iotaNSEnabled: boolean) {
    const resolveCache = new Map<string, boolean>();

    return Yup.string()
        .ensure()
        .trim()
        .required()
        .test('is-iota-address', 'Invalid address. Please check again.', async (value) => {
            if (iotaNSEnabled && isIOTANSName(value)) {
                if (resolveCache.has(value)) {
                    return resolveCache.get(value)!;
                }

                const address = await client.resolveNameServiceAddress({
                    name: value,
                });

                resolveCache.set(value, !!address);

                return !!address;
            }

            return isValidIOTAAddress(value);
        })
        .label("Recipient's address");
}

export function useIOTAAddressValidation() {
    const client = useIOTAClient();
    const iotaNSEnabled = useIOTANSEnabled();

    return useMemo(() => {
        return createIOTAAddressValidation(client, iotaNSEnabled);
    }, [client, iotaNSEnabled]);
}
