// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import React, { useState } from 'react';
import { EnterValuesFormView, ReviewValuesFormView } from './views';
import { CoinBalance } from '@iota/iota-sdk/client';
import { useSendCoinTransaction, useNotifications } from '@/hooks';
import { useSignAndExecuteTransaction } from '@iota/dapp-kit';
import { NotificationType } from '@/stores/notificationStore';
import { useGetAllCoins } from '@iota/core';
import { Dialog, DialogBody, DialogContent, DialogPosition, Header } from '@iota/apps-ui-kit';

export interface FormDataValues {
    amount: string;
    to: string;
    isPayAllIota: boolean;
    gasBudgetEst: string;
}

export const INITIAL_VALUES: FormDataValues = {
    to: '',
    amount: '',
    isPayAllIota: false,
    gasBudgetEst: '',
};

interface SendCoinPopupProps {
    coin: CoinBalance;
    activeAddress: string;
    setOpen: (bool: boolean) => void;
    open: boolean;
}

enum FormStep {
    EnterValues,
    ReviewValues,
}

function SendTokenDialogBody({
    coin,
    activeAddress,
    setOpen,
    open,
}: SendCoinPopupProps): React.JSX.Element {
    const [step, setStep] = useState<FormStep>(FormStep.EnterValues);
    const [selectedCoin, setSelectedCoin] = useState<CoinBalance>(coin);
    const [formData, setFormData] = useState<FormDataValues>(INITIAL_VALUES);
    const { addNotification } = useNotifications();

    const { data: coinsData } = useGetAllCoins(selectedCoin?.coinType, activeAddress);

    const {
        mutateAsync: signAndExecuteTransaction,
        error,
        isPending,
    } = useSignAndExecuteTransaction();

    const { data: sendCoinData } = useSendCoinTransaction(
        coinsData || [],
        selectedCoin?.coinType,
        activeAddress,
        formData.to,
        formData.amount,
        selectedCoin?.totalBalance === formData.amount,
    );

    function handleTransfer() {
        if (!sendCoinData?.transaction) {
            addNotification('There was an error with the transaction', NotificationType.Error);
            return;
        } else {
            signAndExecuteTransaction({
                transaction: sendCoinData.transaction,
            })
                .then(() => {
                    setOpen(false);
                    addNotification('Transfer transaction has been sent');
                })
                .catch(() => {
                    addNotification('Transfer transaction was not sent', NotificationType.Error);
                });
        }
    }

    function onNext(): void {
        setStep(FormStep.ReviewValues);
    }

    function onBack(): void {
        setStep(FormStep.EnterValues);
    }

    return (
        <>
            <Header
                title={step === FormStep.EnterValues ? 'Send' : 'Review & Send'}
                onClose={() => setOpen(false)}
            />
            <div className="h-full">
                <DialogBody>
                    {step === FormStep.EnterValues && (
                        <EnterValuesFormView
                            coin={selectedCoin}
                            activeAddress={activeAddress}
                            gasBudget={sendCoinData?.gasBudget?.toString() || '--'}
                            setSelectedCoin={setSelectedCoin}
                            onNext={onNext}
                            setFormData={setFormData}
                        />
                    )}
                    {step === FormStep.ReviewValues && (
                        <ReviewValuesFormView
                            formData={formData}
                            onBack={onBack}
                            executeTransfer={handleTransfer}
                            senderAddress={activeAddress}
                            gasBudget={sendCoinData?.gasBudget?.toString() || '--'}
                            error={error?.message}
                            isPending={isPending}
                        />
                    )}
                </DialogBody>
            </div>
        </>
    );
}

function SendTokenDialog(props: SendCoinPopupProps): React.JSX.Element {
    return (
        <Dialog open={props.open} onOpenChange={props.setOpen}>
            <DialogContent containerId="overlay-portal-container" position={DialogPosition.Right}>
                <SendTokenDialogBody {...props} />
            </DialogContent>
        </Dialog>
    );
}

export default SendTokenDialog;
