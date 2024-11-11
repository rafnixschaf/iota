// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Input, InputType, type InputProps, type NumericFormatInputProps } from '@iota/apps-ui-kit';
import React from 'react';

interface FormInputProps extends Omit<InputProps, 'onChange'> {
    name: string;
    value: string;
    suffix: string;
    allowNegative: boolean;
    onChange: (value: string) => void;
    onBlur?: React.FocusEventHandler<HTMLInputElement>;
    errorMessage?: string;
    renderAction?: (isDisabled?: boolean) => React.JSX.Element;
    decimals?: boolean;
    disabled?: boolean;
    isSubmitting?: boolean;
}

export function FormInput({
    renderAction,
    decimals,
    value,
    onChange,
    onBlur,
    errorMessage,
    isSubmitting = false,
    disabled,
    name,
    type,
    placeholder,
    caption,
    amountCounter,
    label,
    suffix,
    allowNegative,
}: FormInputProps) {
    const isInputDisabled = isSubmitting || disabled;
    const isNumericFormat = type === InputType.NumericFormat;

    const numericPropsOnly: Partial<NumericFormatInputProps> = {
        decimalScale: decimals ? undefined : 0,
        thousandSeparator: true,
        onValueChange: (values) => {
            onChange(values.value);
        },
    };

    const isActionButtonDisabled = isInputDisabled || !value || !!errorMessage;

    return (
        <Input
            name={name}
            value={value}
            type={type}
            caption={caption}
            disabled={isInputDisabled}
            placeholder={placeholder}
            onBlur={onBlur}
            label={label}
            suffix={suffix}
            allowNegative={allowNegative}
            errorMessage={errorMessage}
            onChange={(e) => onChange(e.currentTarget.value)}
            amountCounter={!errorMessage ? amountCounter : undefined}
            trailingElement={renderAction?.(isActionButtonDisabled)}
            {...(isNumericFormat ? numericPropsOnly : {})}
        />
    );
}
