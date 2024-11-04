// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import React from 'react';
import { VirtualList } from '@/components';
import {
    useCurrentAccount,
    useIotaClientContext,
    useSignAndExecuteTransaction,
} from '@iota/dapp-kit';
import { getNetwork, IotaObjectData } from '@iota/iota-sdk/client';
import { useMigrationTransaction } from '@/hooks/useMigrationTransaction';
import { Button } from '@iota/apps-ui-kit';
import { useNotifications } from '@/hooks';
import { NotificationType } from '@/stores/notificationStore';

interface MigratePopupProps {
    stardustOutputObjects: IotaObjectData[];
    closePopup: () => void;
    onSuccess?: (digest: string) => void;
}

function MigratePopup({
    stardustOutputObjects,
    closePopup,
    onSuccess,
}: MigratePopupProps): JSX.Element {
    const account = useCurrentAccount();
    const { addNotification } = useNotifications();
    const { data: migrateData } = useMigrationTransaction(
        stardustOutputObjects,
        account?.address || '',
    );
    const { network } = useIotaClientContext();
    const { explorer } = getNetwork(network);
    const { mutateAsync: signAndExecuteTransaction, isPending } = useSignAndExecuteTransaction();

    async function handleMigrate(): Promise<void> {
        if (!migrateData) return;
        signAndExecuteTransaction(
            {
                transaction: migrateData.transaction,
            },
            {
                onSuccess: (tx) => {
                    if (onSuccess) {
                        onSuccess(tx.digest);
                    }
                },
            },
        )
            .then(() => {
                closePopup();
                addNotification('Migration transaction has been sent');
            })
            .catch(() => {
                addNotification('Migration transaction was not sent', NotificationType.Error);
            });
    }

    const virtualItem = (asset: IotaObjectData): JSX.Element => (
        <a href={`${explorer}/object/${asset.objectId}`} target="_blank" rel="noreferrer">
            {asset.objectId}
        </a>
    );
    return (
        <div className="flex min-w-[300px] flex-col gap-2">
            <div className="flex flex-col">
                <h1>Migratable Outputs: {stardustOutputObjects.length}</h1>
                <VirtualList
                    items={stardustOutputObjects}
                    estimateSize={() => 30}
                    render={virtualItem}
                />
            </div>
            <p>Gas Fees: {migrateData?.gasBudget?.toString() || '--'}</p>
            <Button text="Migrate" disabled={isPending} onClick={handleMigrate} />
        </div>
    );
}

export default MigratePopup;
