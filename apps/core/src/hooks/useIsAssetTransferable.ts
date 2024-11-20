// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { useEffect, useState } from 'react';
import { useIotaClient } from '@iota/dapp-kit';
import { IotaMoveNormalizedStruct, IotaObjectData } from '@iota/iota-sdk/client';

export function useIsAssetTransferable(obj: IotaObjectData | null | undefined) {
    const client = useIotaClient();

    const [isAssetTransferable, setIsAssetTransferable] = useState<boolean>(false);
    const [isCheckingAssetTransferability, setIsCheckingAssetTransferability] =
        useState<boolean>(true);

    useEffect(() => {
        (async () => {
            setIsCheckingAssetTransferability(true);

            if (!obj) {
                return false;
            }

            const objectType =
                obj?.type ??
                (obj?.content?.dataType === 'package' ? 'package' : obj?.content?.type) ??
                null;

            const [packageId, moduleName, functionName] =
                objectType?.split('<')[0]?.split('::') || [];

            const normalizedStruct: IotaMoveNormalizedStruct = await client.getNormalizedMoveStruct(
                {
                    package: packageId,
                    module: moduleName,
                    struct: functionName,
                },
            );

            if (!normalizedStruct) {
                return false;
            }

            const structAbilities = normalizedStruct?.abilities?.abilities ?? null;

            if (structAbilities) {
                // The object is transferable if it has the 'Store' ability.
                return structAbilities.includes('Store');
            }

            return false;
        })()
            .then((isTransferable) => {
                setIsAssetTransferable(isTransferable);
            })
            .catch(() => {
                setIsAssetTransferable(false);
            })
            .finally(() => {
                setIsCheckingAssetTransferability(false);
            });
    }, [client, obj]);

    return [isAssetTransferable, isCheckingAssetTransferability];
}
