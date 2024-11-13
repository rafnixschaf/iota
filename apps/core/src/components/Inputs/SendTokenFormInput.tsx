// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { ButtonPill, Input, InputType, NumericFormatInputProps } from '@iota/apps-ui-kit';
import { CoinStruct } from '@iota/iota-sdk/client';
import { useGasBudgetEstimation } from '../../hooks';
import React, { useEffect } from 'react';

export interface SendTokenInputProps {
    coins: CoinStruct[];
    symbol: string;
    coinDecimals: number;
    activeAddress: string;
    setFieldValue: (field: string, value: string, shouldValidate?: boolean) => void;
    values: {
        amount: string;
        to: string;
        isPayAllIota: boolean;
    };
    onActionClick: () => Promise<void>;
    isMaxActionDisabled?: boolean | 'auto';
    value: string;
    onChange: (value: string) => void;
    onBlur?: React.FocusEventHandler<HTMLInputElement>;
    errorMessage?: string;
}

export function SendTokenFormInput({
    coins,
    values,
    symbol,
    coinDecimals,
    activeAddress,
    setFieldValue,
    onActionClick,
    isMaxActionDisabled,
    value,
    onChange,
    onBlur,
    errorMessage,
}: SendTokenInputProps) {
    const gasBudgetEstimation = useGasBudgetEstimation({
        coinDecimals,
        coins: coins ?? [],
        activeAddress,
        to: values.to,
        amount: values.amount,
        isPayAllIota: values.isPayAllIota,
    });
    
    const numericPropsOnly: Partial<NumericFormatInputProps> = {
        decimalScale: coinDecimals ? undefined : 0,
        thousandSeparator: true,
        onValueChange: (values) => {
            onChange(values.value);
        },
    };

    
    const isActionButtonDisabled = !value || !!errorMessage;
    
    const renderAction = (isButtonDisabled: boolean | undefined) => (
        <ButtonPill
            disabled={
                isMaxActionDisabled === 'auto'
                    ? isButtonDisabled
                    : isActionButtonDisabled
                }
            onClick={onActionClick}
            >
            Max
        </ButtonPill>
    )

    // gasBudgetEstimation should change when the amount above changes
    useEffect(() => {
        setFieldValue('gasBudgetEst', gasBudgetEstimation, false);
    }, [gasBudgetEstimation, setFieldValue, values.amount]);

    return (
        <Input
            type={InputType.NumericFormat}
            name={"amount"}
            value={value}
            caption="Est. Gas Fees:"
            placeholder="0.00"
            onBlur={onBlur}
            label="Send Amount"
            suffix={` ${symbol}`}
            prefix={values.isPayAllIota ? '~ ' : undefined}
            allowNegative={false}
            errorMessage={errorMessage}
            onChange={(e) => onChange(e.currentTarget.value)}
            amountCounter={!errorMessage ? (coins ? gasBudgetEstimation : '--') : undefined}
            trailingElement={renderAction(isActionButtonDisabled)}
            {...numericPropsOnly}
        />
    );
}
