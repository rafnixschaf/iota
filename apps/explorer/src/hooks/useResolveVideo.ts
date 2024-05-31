// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { type IOTAObjectResponse } from '@iota/iota.js/client';

import { useRecognizedPackages } from './useRecognizedPackages';

export function useResolveVideo(object: IOTAObjectResponse) {
    const recognizedPackages = useRecognizedPackages();
    const objectType =
        object.data?.type ?? object?.data?.content?.dataType === 'package'
            ? 'package'
            : object?.data?.content?.type;
    const isRecognized = objectType && recognizedPackages.includes(objectType.split('::')[0]);

    if (!isRecognized) return null;

    const display = object.data?.display?.data;

    return display?.video_url;
}
