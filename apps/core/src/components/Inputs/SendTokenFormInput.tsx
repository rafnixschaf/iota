// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { ButtonPill, Input, InputType, NumericFormatInputProps } from '@iota/apps-ui-kit';
import { CoinStruct } from '@iota/iota-sdk/client';
import { useGasBudgetEstimation } from '../../hooks';
import { useEffect } from 'react';
import { FormikProps, useField } from 'formik';
import React from 'react';

export interface SendTokenInputProps<FormValues> {
    coins: CoinStruct[];
    symbol: string;
    coinDecimals: number;
    activeAddress: string;
    amount: string;
    to: string;
    isPayAllIota: boolean;
    onActionClick: () => Promise<void>;
    isMaxActionDisabled?: boolean;
    name: string;
    form: FormikProps<FormValues>;
}

export function SendTokenFormInput<FormValues>({
    coins,
    amount,
    to,
    isPayAllIota,
    symbol,
    coinDecimals,
    activeAddress,
    onActionClick,
    isMaxActionDisabled,
    name,
    form,
}: SendTokenInputProps<FormValues>) {
    const gasBudgetEstimation = useGasBudgetEstimation({
        coinDecimals,
        coins: coins ?? [],
        activeAddress,
        to: to,
        amount: amount,
        isPayAllIota: isPayAllIota,
    });

    const [field, meta, helpers] = useField<string>(name);

    const numericPropsOnly: Partial<NumericFormatInputProps> = {
        decimalScale: coinDecimals ? undefined : 0,
        thousandSeparator: true,
        onValueChange: (values) => {
            helpers.setValue(values.value)
        },
    };

    const errorMessage = meta?.error ? meta.error : undefined;
    const isActionButtonDisabled = form.isSubmitting || !!errorMessage || isMaxActionDisabled;

    const renderAction = () => (
        <ButtonPill disabled={isActionButtonDisabled} onClick={onActionClick}>
            Max
        </ButtonPill>
    );

    // gasBudgetEstimation should change when the amount above changes
    useEffect(() => {
        form.setFieldValue('gasBudgetEst', gasBudgetEstimation, false);
    }, [gasBudgetEstimation, form.setFieldValue, amount]);

    return (
        <Input
            type={InputType.NumericFormat}
            name="amount"
            value={field.value}
            caption="Est. Gas Fees:"
            placeholder="0.00"
            label="Send Amount"
            suffix={` ${symbol}`}
            prefix={isPayAllIota ? '~ ' : undefined}
            allowNegative={false}
            errorMessage={errorMessage}
            amountCounter={!errorMessage ? (coins ? gasBudgetEstimation : '--') : undefined}
            trailingElement={renderAction()}
            {...numericPropsOnly}
        />
    );
}
