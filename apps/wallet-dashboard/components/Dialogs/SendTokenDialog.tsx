// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    InfoBox,
    InfoBoxStyle,
    InfoBoxType,
    ButtonType,
    ButtonHtmlType,
    Button,
    Dialog,
    DialogContent,
    DialogBody,
    Header,
    DialogPosition,
} from '@iota/apps-ui-kit';
import { parseAmount, useCoinMetadata, useGetAllCoins, useIotaAddressValidation } from '@iota/core';
import { CoinStruct } from '@iota/iota-sdk/client';
import { IOTA_TYPE_ARG } from '@iota/iota-sdk/utils';
import { Exclamation } from '@iota/ui-icons';
import { Field, Form, Formik, useFormikContext } from 'formik';
import Input from '../Input';
import { ChangeEventHandler, useCallback } from 'react';

const INITIAL_VALUES = {
    to: '',
    amount: '',
    isPayAllIota: false,
    gasBudgetEst: '',
};

export type FormValues = typeof INITIAL_VALUES;

export type SubmitProps = {
    to: string;
    amount: string;
    isPayAllIota: boolean;
    coinIds: string[];
    coins: CoinStruct[];
    gasBudgetEst: string;
};

export type SendTokenFormProps = {
    coinType: string;
    activeAddress: string;
    setOpen: (bool: boolean) => void;
    open: boolean;
};

function totalBalance(coins: CoinStruct[]): bigint {
    return coins.reduce((partialSum, c) => partialSum + getBalanceFromCoinStruct(c), BigInt(0));
}
function getBalanceFromCoinStruct(coin: CoinStruct): bigint {
    return BigInt(coin.balance);
}

export function SendTokenDialog({
    coinType,
    activeAddress,
    setOpen,
    open,
}: SendTokenFormProps): React.JSX.Element {
    const { data: coinsData } = useGetAllCoins(coinType, activeAddress!);
    const { setFieldValue, validateField } = useFormikContext();
    const iotaAddressValidation = useIotaAddressValidation();

    const { data: iotaCoinsData } = useGetAllCoins(IOTA_TYPE_ARG, activeAddress!);

    const iotaCoins = iotaCoinsData;
    const coins = coinsData;
    const coinBalance = totalBalance(coins || []);
    const iotaBalance = totalBalance(iotaCoins || []);

    const coinMetadata = useCoinMetadata(coinType);
    const coinDecimals = coinMetadata.data?.decimals ?? 0;

    // const validationSchemaStepOne = useMemo(
    //     () => createValidationSchemaStepOne(coinBalance, symbol, coinDecimals),
    //     [client, coinBalance, symbol, coinDecimals],
    // );

    // remove the comma from the token balance
    const initAmountBig = parseAmount('0', coinDecimals);
    // const initAmountBig = parseAmount(initialAmount, coinDecimals);

    const handleAddressChange = useCallback<ChangeEventHandler<HTMLInputElement>>(
        (e) => {
            const address = e.currentTarget.value;
            setFieldValue(activeAddress, iotaAddressValidation.cast(address)).then(() => {
                validateField(activeAddress);
            });
        },
        [setFieldValue, activeAddress, iotaAddressValidation],
    );

    async function handleFormSubmit({ to, amount, isPayAllIota, gasBudgetEst }: FormValues) {
        if (!coins || !iotaCoins) return;
        const coinsIDs = [...coins]
            .sort((a, b) => Number(b.balance) - Number(a.balance))
            .map(({ coinObjectId }) => coinObjectId);

        const data = {
            to,
            amount,
            isPayAllIota,
            coins,
            coinIds: coinsIDs,
            gasBudgetEst,
        };
        console.log('data', data);

        // onSubmit(data);
    }

    return (
        <Dialog open={open} onOpenChange={setOpen}>
            <DialogContent containerId="overlay-portal-container" position={DialogPosition.Right}>
                <Header title="Send" onClose={() => setOpen(false)} />
                <DialogBody>
                    <Formik
                        initialValues={{
                            amount: '0',
                            to: 'initialTo',
                            // amount: initialAmount,
                            // to: initialTo,
                            isPayAllIota:
                                !!initAmountBig &&
                                initAmountBig === coinBalance &&
                                coinType === IOTA_TYPE_ARG,
                            gasBudgetEst: '',
                        }}
                        // validationSchema={validationSchemaStepOne}
                        enableReinitialize
                        validateOnChange={false}
                        validateOnBlur={false}
                        onSubmit={handleFormSubmit}
                    >
                        {({ isValid, isSubmitting, setFieldValue, values, submitForm }) => {
                            const newPayIotaAll =
                                parseAmount(values.amount, coinDecimals) === coinBalance &&
                                coinType === IOTA_TYPE_ARG;
                            if (values.isPayAllIota !== newPayIotaAll) {
                                setFieldValue('isPayAllIota', newPayIotaAll);
                            }

                            const hasEnoughBalance =
                                values.isPayAllIota ||
                                iotaBalance >
                                    parseAmount(values.gasBudgetEst, coinDecimals) +
                                        parseAmount(
                                            coinType === IOTA_TYPE_ARG ? values.amount : '0',
                                            coinDecimals,
                                        );

                            return (
                                <div className="flex h-full w-full flex-col">
                                    <Form autoComplete="off" noValidate className="flex-1">
                                        <div className="flex h-full w-full flex-col gap-md">
                                            {!hasEnoughBalance ? (
                                                <InfoBox
                                                    type={InfoBoxType.Error}
                                                    supportingText="Insufficient IOTA to cover transaction"
                                                    style={InfoBoxStyle.Elevated}
                                                    icon={<Exclamation />}
                                                />
                                            ) : null}

                                            {/* <SendTokenFormInput
                                                coinDecimals={coinDecimals}
                                                symbol={symbol}
                                                coins={coins}
                                                values={values}
                                                onActionClick={onMaxTokenButtonClick}
                                                isActionButtonDisabled={isMaxActionDisabled}
                                            /> */}
                                            <Field
                                                component={
                                                    <Input
                                                        type="text"
                                                        value={activeAddress}
                                                        placeholder="Enter Address"
                                                        onChange={(e) => handleAddressChange(e)}
                                                        label="Enter recipient address"
                                                    />
                                                }
                                                allowNegative={false}
                                                name="to"
                                                placeholder="Enter Address"
                                            />
                                        </div>
                                    </Form>

                                    <div className="pt-xs">
                                        <Button
                                            onClick={submitForm}
                                            htmlType={ButtonHtmlType.Submit}
                                            type={ButtonType.Primary}
                                            disabled={
                                                !isValid ||
                                                isSubmitting ||
                                                !hasEnoughBalance ||
                                                values.gasBudgetEst === ''
                                            }
                                            text="Review"
                                            fullWidth
                                        />
                                    </div>
                                </div>
                            );
                        }}
                    </Formik>
                </DialogBody>
            </DialogContent>
        </Dialog>
    );
}
