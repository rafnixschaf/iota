// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Input, InputType } from '@iota/apps-ui-kit';
import { Close } from '@iota/ui-icons';
import { useIotaAddressValidation } from '../../hooks';
import React, { ComponentProps, useCallback } from 'react';
import type { Field, FieldInputProps } from 'formik';

export interface AddressInputProps {
    field: FieldInputProps<string>;
    form: ComponentProps<typeof Field>;
    disabled?: boolean;
    placeholder?: string;
    label?: string;
}

export function AddressInput({
    field,
    form,
    disabled,
    placeholder = '0x...',
    label = 'Enter Recipient Address',
}: AddressInputProps) {
    const iotaAddressValidation = useIotaAddressValidation();

    const formattedValue = iotaAddressValidation.cast(field.value);

    const handleOnChange = useCallback(
        (e: React.ChangeEvent<HTMLInputElement>) => {
            const address = e.currentTarget.value;
            iotaAddressValidation.cast(address);
            form.setFieldValue(field.name, iotaAddressValidation.cast(address)).then(() => {
                form.validateField(field.name);
            });
        },
        [form, field.name, iotaAddressValidation],
    );

    const clearAddress = () => {
        form.setFieldValue(field.name, '');
    };

    const errorMessage = form.touched[field.name] && form.errors[field.name];

    return (
        <Input
            type={InputType.Text}
            disabled={disabled}
            placeholder={placeholder}
            value={formattedValue}
            name={field.name}
            onBlur={field.onBlur}
            label={label}
            onChange={handleOnChange}
            errorMessage={errorMessage as string}
            trailingElement={
                formattedValue ? (
                    <button
                        onClick={clearAddress}
                        type="button"
                        className="flex items-center justify-center"
                    >
                        <Close />
                    </button>
                ) : undefined
            }
        />
    );
}
