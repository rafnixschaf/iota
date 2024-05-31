// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { createIOTAAddressValidation } from '_components/address-input/validation';
import { createTokenValidation } from '_src/shared/validation';
import { type IOTAClient } from '@iota/iota.js/client';
import * as Yup from 'yup';

export function createValidationSchemaStepOne(
    client: IOTAClient,
    iotaNSEnabled: boolean,
    ...args: Parameters<typeof createTokenValidation>
) {
    return Yup.object({
        to: createIOTAAddressValidation(client, iotaNSEnabled),
        amount: createTokenValidation(...args),
    });
}
