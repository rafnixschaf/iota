// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { CoinBalance, CoinStruct } from '@iota/iota-sdk/client';
import { FormDataValues, INITIAL_VALUES } from '../SendCoinDialog';
import {
    AddressInput,
    CoinFormat,
    COINS_QUERY_REFETCH_INTERVAL,
    COINS_QUERY_STALE_TIME,
    CoinSelector,
    createValidationSchemaSendTokenForm,
    filterAndSortTokenBalances,
    parseAmount,
    SendTokenFormInput,
    useCoinMetadata,
    useFormatCoin,
    useGasBudgetEstimation,
    useGetAllCoins,
} from '@iota/core';
import {
    ButtonHtmlType,
    ButtonType,
    InfoBox,
    InfoBoxType,
    Button,
    InfoBoxStyle,
    LoadingIndicator,
} from '@iota/apps-ui-kit';
import { useIotaClient, useIotaClientQuery } from '@iota/dapp-kit';
import { IOTA_TYPE_ARG } from '@iota/iota-sdk/utils';
import { Field, FieldInputProps, Form, Formik } from 'formik';
import { Exclamation } from '@iota/ui-icons';
import { useMemo } from 'react';

interface EnterValuesFormProps {
    coin: CoinBalance;
    activeAddress: string;
    gasBudget: string;
    setFormData: React.Dispatch<React.SetStateAction<FormDataValues>>;
    setSelectedCoin: React.Dispatch<React.SetStateAction<CoinBalance>>;
    onNext: () => void;
}

function totalBalance(coins: CoinStruct[]): bigint {
    return coins.reduce((partialSum, c) => partialSum + getBalanceFromCoinStruct(c), BigInt(0));
}
function getBalanceFromCoinStruct(coin: CoinStruct): bigint {
    return BigInt(coin.balance);
}

function EnterValuesFormView({
    coin,
    activeAddress,
    setFormData,
    setSelectedCoin,
    onNext,
}: EnterValuesFormProps): JSX.Element {
    const client = useIotaClient();
    // Get all coins of the type
    const { data: coinsData, isPending: coinsIsPending } = useGetAllCoins(
        coin.coinType,
        activeAddress,
    );
    const { data: iotaCoinsData, isPending: iotaCoinsIsPending } = useGetAllCoins(
        IOTA_TYPE_ARG,
        activeAddress,
    );

    const { data: coinsBalance, isPending: coinsBalanceIsPending } = useIotaClientQuery(
        'getAllBalances',
        { owner: activeAddress },
        {
            enabled: !!activeAddress,
            refetchInterval: COINS_QUERY_REFETCH_INTERVAL,
            staleTime: COINS_QUERY_STALE_TIME,
            select: filterAndSortTokenBalances,
        },
    );

    const iotaCoins = iotaCoinsData;
    const coins = coinsData;
    const coinBalance = totalBalance(coins || []);
    const iotaBalance = totalBalance(iotaCoins || []);

    const [tokenBalance, symbol, queryResult] = useFormatCoin(
        coinBalance,
        coin.coinType,
        CoinFormat.FULL,
    );
    
    const coinMetadata = useCoinMetadata(coin.coinType);
    const coinDecimals = coinMetadata.data?.decimals ?? 0;
    
    const validationSchemaStepOne = useMemo(
        () => createValidationSchemaSendTokenForm(coinBalance, symbol, coinDecimals),
        [client, coinBalance, symbol, coinDecimals],
    );

    const formattedTokenBalance = tokenBalance.replace(/,/g, '');
    const initAmountBig = parseAmount('', coinDecimals);

    if (coinsBalanceIsPending || coinsIsPending || iotaCoinsIsPending) {
        return (
            <div className="flex h-full w-full items-center justify-center">
                <LoadingIndicator />
            </div>
        );
    }

    async function handleFormSubmit({ to, amount, isPayAllIota, gasBudgetEst }: FormDataValues) {
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
        setFormData(data);
        onNext();
    }

    return (
        <div>
            <CoinSelector
                activeCoinType={coin.coinType}
                coins={coinsBalance ?? []}
                onClick={(coinType) => {
                    setFormData(INITIAL_VALUES);
                    const coin = coinsBalance?.find((coin) => coin.coinType === coinType);
                    setSelectedCoin(coin!);
                }}
            />

            <Formik
                initialValues={{
                    amount: '',
                    to: '',
                    isPayAllIota:
                        !!initAmountBig &&
                        initAmountBig === coinBalance &&
                        coin.coinType === IOTA_TYPE_ARG,
                    gasBudgetEst: '',
                }}
                validationSchema={validationSchemaStepOne}
                enableReinitialize
                validateOnChange={false}
                validateOnBlur={false}
                onSubmit={handleFormSubmit}
            >
                {({
                    isValid,
                    isSubmitting,
                    setFieldValue,
                    values,
                    submitForm,
                    touched,
                    errors,
                    handleBlur,
                }) => {
                    const newPayIotaAll =
                        parseAmount(values.amount, coinDecimals) === coinBalance &&
                        coin.coinType === IOTA_TYPE_ARG;
                    if (values.isPayAllIota !== newPayIotaAll) {
                        setFieldValue('isPayAllIota', newPayIotaAll);
                    }

                    const hasEnoughBalance =
                        values.isPayAllIota ||
                        iotaBalance >
                            parseAmount(values.gasBudgetEst, coinDecimals) +
                                parseAmount(
                                    coin.coinType === IOTA_TYPE_ARG ? values.amount : '0',
                                    coinDecimals,
                                );

                    async function onMaxTokenButtonClick() {
                        await setFieldValue('amount', formattedTokenBalance);
                    }

                    const isMaxActionDisabled =
                        parseAmount(values?.amount, coinDecimals) === coinBalance ||
                        queryResult.isPending ||
                        !coinBalance;

                    return (
                        <div className="flex h-full w-full flex-col">
                            <Form autoComplete="off" noValidate className="flex-1">
                                <div className="flex h-full w-full flex-col gap-md">
                                    {!hasEnoughBalance && (
                                        <InfoBox
                                            type={InfoBoxType.Error}
                                            supportingText="Insufficient IOTA to cover transaction"
                                            style={InfoBoxStyle.Elevated}
                                            icon={<Exclamation />}
                                        />
                                    )}

                                    <Field name="amount">
                                        {({ field }: { field: FieldInputProps<string> }) => {

                                            // TODO: needs to be updated in fields value
                                            const gasBudgetEstimation = useGasBudgetEstimation({
                                                coinDecimals,
                                                coins: coins ?? [],
                                                activeAddress,
                                                to: values.to,
                                                amount: values.amount,
                                                isPayAllIota: values.isPayAllIota,
                                            });

                                            return (
                                                <SendTokenFormInput
                                                    gasBudgetEstimation={gasBudgetEstimation}
                                                    symbol={symbol}
                                                    coins={coins}
                                                    values={values}
                                                    onActionClick={onMaxTokenButtonClick}
                                                    isActionButtonDisabled={isMaxActionDisabled}
                                                    value={field.value}
                                                    onChange={(value) => setFieldValue('amount', value)}
                                                    onBlur={handleBlur}
                                                    errorMessage={
                                                        touched.amount && errors.amount
                                                            ? errors.amount
                                                            : undefined
                                                    }
                                                />
                                        )}}
                                    </Field>

                                    <Field
                                        component={AddressInput}
                                        name="to"
                                        placeholder="Enter Address"
                                        errorMessage={
                                            touched.to && errors.to ? errors.to : undefined
                                        }
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
        </div>
    );
}

export default EnterValuesFormView;
