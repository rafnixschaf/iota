// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import React from 'react';
import { ImageIcon, ImageIconSize, formatPercentageDisplay } from '@iota/core';
import {
    Card,
    CardBody,
    CardImage,
    CardAction,
    CardActionType,
    CardType,
    Badge,
    BadgeType,
} from '@iota/apps-ui-kit';
import { formatAddress } from '@iota/iota-sdk/utils';
import { useValidatorInfo } from '@/hooks';

interface SelectValidatorViewProps {
    validators: string[];
    onSelect: (validator: string) => void;
}

function SelectValidatorView({ validators, onSelect }: SelectValidatorViewProps): JSX.Element {
    return (
        <div className="flex w-full flex-col items-start gap-2">
            {validators.map((validator) => (
                <Validator key={validator} address={validator} onClick={onSelect} />
            ))}
        </div>
    );
}

function Validator({
    address,
    showActiveStatus,
    onClick,
}: {
    address: string;
    showActiveStatus?: boolean;
    onClick: (address: string) => void;
}) {
    const { name, newValidator, isAtRisk, apy, isApyApproxZero } = useValidatorInfo(address);

    const subtitle = showActiveStatus ? (
        <div className="flex items-center gap-1">
            {formatAddress(address)}
            {newValidator && <Badge label="New" type={BadgeType.PrimarySoft} />}
            {isAtRisk && <Badge label="At Risk" type={BadgeType.PrimarySolid} />}
        </div>
    ) : (
        formatAddress(address)
    );
    return (
        <Card type={CardType.Default} onClick={() => onClick(address)}>
            <CardImage>
                <ImageIcon src={null} label={name} fallback={name} size={ImageIconSize.Large} />
            </CardImage>
            <CardBody title={name} subtitle={subtitle} isTextTruncated />
            <CardAction
                type={CardActionType.SupportingText}
                title={formatPercentageDisplay(apy, '--', isApyApproxZero)}
                iconAfterText
            />
        </Card>
    );
}

export default SelectValidatorView;
