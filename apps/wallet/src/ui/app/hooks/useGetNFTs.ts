// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    hasDisplayData,
    isKioskOwnerToken,
    useGetAllOwnedObjects,
    useKioskClient,
} from '@iota/core';
import { type IotaObjectData } from '@iota/iota-sdk/client';
import { useMemo } from 'react';
import { useHiddenAssets } from '../pages/home/assets/HiddenAssetsProvider';

type OwnedAssets = {
    visual: IotaObjectData[];
    other: IotaObjectData[];
    hidden: IotaObjectData[];
};

export enum AssetFilterTypes {
    Visual = 'visual',
    Other = 'other',
}

export function useGetNFTs(address?: string | null) {
    const kioskClient = useKioskClient();
    const { data, isPending, error, isError, isLoading } = useGetAllOwnedObjects(address ?? '', {
        MatchNone: [{ StructType: '0x2::coin::Coin' }],
    });
    const { hiddenAssets } = useHiddenAssets();

    const assets = useMemo(() => {
        const ownedAssets: OwnedAssets = {
            visual: [],
            other: [],
            hidden: [],
        };

        if (hiddenAssets.type === 'loading') {
            return ownedAssets;
        } else {
            const groupedAssets = data?.reduce((acc, curr) => {
                if (curr.objectId && hiddenAssets.assetIds.includes(curr.objectId))
                    acc.hidden.push(curr);
                else if (hasDisplayData(curr) || isKioskOwnerToken(kioskClient.network, curr))
                    acc.visual.push(curr);
                else if (!hasDisplayData(curr)) acc.other.push(curr);
                return acc;
            }, ownedAssets);
            return groupedAssets;
        }
    }, [hiddenAssets, data, kioskClient.network]);

    return {
        data: assets,
        isLoading,
        isPending: isPending,
        isError: isError,
        error,
    };
}
