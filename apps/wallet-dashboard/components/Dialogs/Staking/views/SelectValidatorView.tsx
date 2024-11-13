// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import React from 'react';
import { ImageIcon, ImageIconSize, formatPercentageDisplay } from '@iota/core';
import {
    Header,
    Button,
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
import { Layout, LayoutFooter, LayoutBody } from './Layout';
interface SelectValidatorViewProps {
    validators: string[];
    onSelect: (validator: string) => void;
    handleClose: () => void;
    onNext: () => void;
    selectedValidator: string;
}

function SelectValidatorView({
    validators,
    onSelect,
    handleClose,
    onNext,
    selectedValidator,
}: SelectValidatorViewProps): JSX.Element {
    return (
        <Layout>
            <Header
                title="Select Validator"
                onClose={handleClose}
                onBack={handleClose}
                titleCentered
            />
            <LayoutBody>
                <div className="flex w-full flex-col gap-md">
                    {validators.map((validator) => (
                        <Validator
                            key={validator}
                            address={validator}
                            onClick={onSelect}
                            isSelected={selectedValidator === validator}
                        />
                    ))}
                </div>
            </LayoutBody>
            <LayoutFooter>
                {!!selectedValidator && (
                    <Button
                        fullWidth
                        data-testid="select-validator-cta"
                        onClick={onNext}
                        text="Next"
                    />
                )}
            </LayoutFooter>
        </Layout>
    );
}

function Validator({
    address,
    showActiveStatus,
    onClick,
    isSelected,
}: {
    isSelected: boolean;
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

    const handleClick = onClick ? () => onClick(address) : undefined;

    return (
        <Card type={isSelected ? CardType.Filled : CardType.Default} onClick={handleClick}>
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
