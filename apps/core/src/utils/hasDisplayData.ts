// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { IotaObjectData, IotaObjectResponse } from '@iota/iota-sdk/client';

export const hasDisplayData = (input: IotaObjectResponse | IotaObjectData): boolean => {
    if (isIotaObjectResponse(input)) {
        return !!input.data?.display?.data;
    } else if (isIotaObjectData(input)) {
        return !!input?.display?.data;
    }
    return false;
};

// Type guard for IotaObjectResponse
function isIotaObjectResponse(input: unknown): input is IotaObjectResponse {
    return typeof input === 'object' && input !== null && 'data' in input;
}

// Type guard for IotaObjectData
function isIotaObjectData(input: unknown): input is IotaObjectData {
    return typeof input === 'object' && input !== null && 'content' in input;
}
