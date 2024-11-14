// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import React, { useState, useMemo } from 'react';
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
    useBalance,
    createValidationSchema,
} from '@iota/core';
import { Formik } from 'formik';
import type { FormikHelpers } from 'formik';
import { useCurrentAccount, useSignAndExecuteTransaction } from '@iota/dapp-kit';
import { IOTA_TYPE_ARG } from '@iota/iota-sdk/utils';
import { NotificationType } from '@/stores/notificationStore';
import { prepareObjectsForTimelockedStakingTransaction } from '@/lib/utils';
import { Dialog } from '@iota/apps-ui-kit';
import { DetailsView, UnstakeView } from './views';
import { FormValues } from './views/EnterAmountView';

export const MIN_NUMBER_IOTA_TO_STAKE = 1;

export enum StakeDialogView {
    Details,
    SelectValidator,
    EnterAmount,
    Unstake,
}

const INITIAL_VALUES = {
    amount: '',
};

interface StakeDialogProps {
    isTimelockedStaking?: boolean;
    onSuccess?: (digest: string) => void;
    isOpen: boolean;
    handleClose: () => void;
    view: StakeDialogView;
    setView?: (view: StakeDialogView) => void;
    stakedDetails?: ExtendedDelegatedStake | null;

    selectedValidator?: string;
    setSelectedValidator?: (validator: string) => void;
}

export function StakeDialog({
    onSuccess,
    isTimelockedStaking,
    isOpen,
    handleClose,
    view,
    setView,
    stakedDetails,
    selectedValidator = '',
    setSelectedValidator,
}: StakeDialogProps): JSX.Element {
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
    const { data: iotaBalance } = useBalance(senderAddress!);
    const coinBalance = BigInt(iotaBalance?.totalBalance || 0);
    const minimumStake = parseAmount(MIN_NUMBER_IOTA_TO_STAKE.toString(), coinDecimals);
    const coinSymbol = metadata?.symbol ?? '';

    const validators = Object.keys(rollingAverageApys ?? {}) ?? [];

    const validationSchema = useMemo(
        () =>
            createValidationSchema(
                coinBalance,
                coinSymbol,
                coinDecimals,
                view === StakeDialogView.Unstake,
                minimumStake,
            ),
        [coinBalance, coinSymbol, coinDecimals, view, minimumStake],
    );

    function handleBack(): void {
        setView?.(StakeDialogView.SelectValidator);
    }

    function handleValidatorSelect(validator: string): void {
        setSelectedValidator?.(validator);
    }

    function selectValidatorHandleNext(): void {
        if (selectedValidator) {
            setView?.(StakeDialogView.EnterAmount);
        }
    }

    function detailsHandleUnstake() {
        setView?.(StakeDialogView.Unstake);
    }

    function detailsHandleStake() {
        setView?.(StakeDialogView.SelectValidator);
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
                handleClose();
                addNotification('Stake transaction has been sent');
            })
            .catch(() => {
                addNotification('Stake transaction was not sent', NotificationType.Error);
            });
    }

    function onSubmit(values: FormValues, { resetForm }: FormikHelpers<FormValues>) {
        handleStake();
        resetForm();
    }

    return (
        <Dialog open={isOpen} onOpenChange={() => handleClose()}>
            <Formik
                initialValues={INITIAL_VALUES}
                validationSchema={validationSchema}
                onSubmit={onSubmit}
                validateOnMount
            >
                <>
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
                            handleClose={handleClose}
                            setAmount={setAmount}
                            onBack={handleBack}
                            onStake={handleStake}
                            gasBudget={newStakeData?.gasBudget}
                        />
                    )}
                    {view === StakeDialogView.Unstake && stakedDetails && (
                        <UnstakeView
                            extendedStake={stakedDetails}
                            handleClose={handleClose}
                            showActiveStatus
                        />
                    )}
                </>
            </Formik>
        </Dialog>
    );
}
