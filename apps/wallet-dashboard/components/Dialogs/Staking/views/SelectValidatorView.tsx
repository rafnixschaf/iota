// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import React from 'react';
import { Button } from '@iota/apps-ui-kit';
import { Validator } from './Validator';

interface SelectValidatorViewProps {
    validators: string[];
    onSelect: (validator: string) => void;
    onNext: () => void;
    selectedValidator: string;
}

function SelectValidatorView({
    validators,
    onSelect,
    onNext,
    selectedValidator,
}: SelectValidatorViewProps): JSX.Element {
    return (
        <div className="flex w-full flex-1 flex-col justify-between">
            <div className="flex w-full flex-col">
                {validators.map((validator) => (
                    <Validator
                        key={validator}
                        address={validator}
                        onClick={onSelect}
                        isSelected={selectedValidator === validator}
                    />
                ))}
            </div>
            {!!selectedValidator && (
                <Button fullWidth data-testid="select-validator-cta" onClick={onNext} text="Next" />
            )}
        </div>
    );
}

export default SelectValidatorView;
