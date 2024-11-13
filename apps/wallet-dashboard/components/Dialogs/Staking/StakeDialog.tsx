// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import React, { useState } from 'react';
import { EnterAmountView, SelectValidatorView } from './views';
import {
    useNotifications,
    useNewStakeTransaction,
    useGetCurrentEpochStartTimestamp,
} from '@/hooks';
import {
    ExtendedDelegatedStake,
    GroupedTimelockObject,
    parseAmount,
    TIMELOCK_IOTA_TYPE,
    useCoinMetadata,
    useGetAllOwnedObjects,
    useGetValidatorsApy,
} from '@iota/core';
import { useCurrentAccount, useSignAndExecuteTransaction } from '@iota/dapp-kit';
import { IOTA_TYPE_ARG } from '@iota/iota-sdk/utils';
import { NotificationType } from '@/stores/notificationStore';
import { prepareObjectsForTimelockedStakingTransaction } from '@/lib/utils';
import { Dialog } from '@iota/apps-ui-kit';
import { DetailsView, UnstakeView } from './views';

export enum StakeDialogView {
    Details,
    SelectValidator,
    EnterAmount,
    Unstake,
}

interface StakeDialogProps {
    isTimelockedStaking?: boolean;
    onSuccess?: (digest: string) => void;
    isOpen: boolean;
    handleClose: () => void;
    view: StakeDialogView;
    setView: (view: StakeDialogView) => void;
    stakedDetails?: ExtendedDelegatedStake | null;
}

function StakeDialog({
    onSuccess,
    isTimelockedStaking,
    isOpen,
    handleClose,
    view,
    setView,
    stakedDetails,
}: StakeDialogProps): JSX.Element {
    const [selectedValidator, setSelectedValidator] = useState<string>('');
    const [amount, setAmount] = useState<string>('');
    const account = useCurrentAccount();
    const senderAddress = account?.address ?? '';

    const { data: metadata } = useCoinMetadata(IOTA_TYPE_ARG);
    const coinDecimals = metadata?.decimals ?? 0;
    const amountWithoutDecimals = parseAmount(amount, coinDecimals);
    const { data: currentEpochMs } = useGetCurrentEpochStartTimestamp();
    const { data: timelockedObjects } = useGetAllOwnedObjects(senderAddress, {
        StructType: TIMELOCK_IOTA_TYPE,
    });

    let groupedTimelockObjects: GroupedTimelockObject[] = [];
    if (isTimelockedStaking && timelockedObjects && currentEpochMs) {
        groupedTimelockObjects = prepareObjectsForTimelockedStakingTransaction(
            timelockedObjects,
            amountWithoutDecimals,
            currentEpochMs,
        );
    }

    const { data: newStakeData } = useNewStakeTransaction(
        selectedValidator,
        amountWithoutDecimals,
        senderAddress,
        isTimelockedStaking,
        groupedTimelockObjects,
    );

    const { mutateAsync: signAndExecuteTransaction } = useSignAndExecuteTransaction();
    const { addNotification } = useNotifications();
    const { data: rollingAverageApys } = useGetValidatorsApy();

    const validators = Object.keys(rollingAverageApys ?? {}) ?? [];

    function handleBack(): void {
        setView(StakeDialogView.SelectValidator);
    }

    function handleValidatorSelect(validator: string): void {
        setSelectedValidator(validator);
    }

    function selectValidatorHandleNext(): void {
        if (selectedValidator) {
            setView(StakeDialogView.EnterAmount);
        }
    }

    function detailsHandleUnstake() {
        setView(StakeDialogView.Unstake);
    }

    function detailsHandleStake() {
        setView(StakeDialogView.SelectValidator);
    }

    function handleStake(): void {
        if (isTimelockedStaking && groupedTimelockObjects.length === 0) {
            addNotification('Invalid stake amount. Please try again.', NotificationType.Error);
            return;
        }
        if (!newStakeData?.transaction) {
            addNotification('Stake transaction was not created', NotificationType.Error);
            return;
        }
        signAndExecuteTransaction(
            {
                transaction: newStakeData?.transaction,
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
                addNotification('Stake transaction has been sent');
            })
            .catch(() => {
                addNotification('Stake transaction was not sent', NotificationType.Error);
            });
    }

    return (
        <Dialog open={isOpen} onOpenChange={() => handleClose()}>
            {view === StakeDialogView.Details && stakedDetails && (
                <DetailsView
                    handleStake={detailsHandleStake}
                    handleUnstake={detailsHandleUnstake}
                    stakedDetails={stakedDetails}
                    handleClose={handleClose}
                />
            )}
            {view === StakeDialogView.SelectValidator && (
                <SelectValidatorView
                    selectedValidator={selectedValidator}
                    handleClose={handleClose}
                    validators={validators}
                    onSelect={handleValidatorSelect}
                    onNext={selectValidatorHandleNext}
                />
            )}
            {view === StakeDialogView.EnterAmount && (
                <EnterAmountView
                    selectedValidator={selectedValidator}
                    amount={amount}
                    handleClose={handleClose}
                    onChange={(e) => setAmount(e.target.value)}
                    onBack={handleBack}
                    onStake={handleStake}
                />
            )}
            {view === StakeDialogView.Unstake && stakedDetails && (
                <UnstakeView
                    extendedStake={stakedDetails}
                    handleClose={handleClose}
                    showActiveStatus
                />
            )}
        </Dialog>
    );
}

export default StakeDialog;
