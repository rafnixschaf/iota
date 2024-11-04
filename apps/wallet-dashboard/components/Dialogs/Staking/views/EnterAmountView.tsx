// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import React, { useMemo } from 'react';
import { Button, Input } from '@/components';
import {
    ImageIcon,
    ImageIconSize,
    formatPercentageDisplay,
    calculateStakeShare,
    useFormatCoin,
    getTokenStakeIotaForValidator,
} from '@iota/core';
import { IOTA_TYPE_ARG, formatAddress } from '@iota/iota-sdk/utils';
import { useValidatorInfo } from '@/hooks';
import {
    Card,
    CardBody,
    CardImage,
    CardType,
    Badge,
    BadgeType,
    KeyValueInfo,
    Panel,
    TooltipPosition,
} from '@iota/apps-ui-kit';

interface EnterAmountViewProps {
    selectedValidator: string;
    amount: string;
    onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
    onBack: () => void;
    onStake: () => void;
    isStakeDisabled: boolean;
    showActiveStatus?: boolean;
}

function EnterAmountView({
    selectedValidator: validatorAddress,
    amount,
    showActiveStatus,
    onChange,
    onBack,
    onStake,
    isStakeDisabled,
}: EnterAmountViewProps): JSX.Element {
    const {
        name,
        newValidator,
        isAtRisk,
        apy,
        isApyApproxZero,
        validatorSummary,
        delegatedStake: stakeData,
        system,
    } = useValidatorInfo(validatorAddress);

    const totalStake = useMemo(() => {
        if (!stakeData) return 0n;
        console.log('stakeddata', stakeData);
        return getTokenStakeIotaForValidator(stakeData, validatorAddress);
    }, [stakeData, validatorAddress]);

    const totalValidatorsStake = useMemo(() => {
        if (!system) return 0;
        return system.activeValidators.reduce(
            (acc, curr) => (acc += BigInt(curr.stakingPoolIotaBalance)),
            0n,
        );
    }, [system]);

    const totalStakePercentage = useMemo(() => {
        if (!system || !validatorSummary) return null;

        return calculateStakeShare(
            BigInt(validatorSummary.stakingPoolIotaBalance),
            BigInt(totalValidatorsStake),
        );
    }, [system, totalValidatorsStake, validatorSummary]);

    //TODO: verify this is the correct validator stake balance
    const totalValidatorStake = validatorSummary?.stakingPoolIotaBalance || 0;

    const [totalValidatorStakeFormatted, totalValidatorStakeSymbol] = useFormatCoin(
        totalValidatorStake,
        IOTA_TYPE_ARG,
    );
    console.log(totalStake);
    const [totalStakeFormatted, totalStakeSymbol] = useFormatCoin(totalStake, IOTA_TYPE_ARG);

    const subtitle = showActiveStatus ? (
        <div className="flex items-center gap-1">
            {formatAddress(validatorAddress)}
            {newValidator && <Badge label="New" type={BadgeType.PrimarySoft} />}
            {isAtRisk && <Badge label="At Risk" type={BadgeType.PrimarySolid} />}
        </div>
    ) : (
        formatAddress(validatorAddress)
    );
    return (
        <div className="flex w-full flex-col items-start gap-2">
            <Card type={CardType.Default}>
                <CardImage>
                    <ImageIcon src={null} label={name} fallback={name} size={ImageIconSize.Large} />
                </CardImage>
                <CardBody title={name} subtitle={subtitle} isTextTruncated />
            </Card>
            <div className="w-full">
                <Panel hasBorder>
                    <div className="flex flex-col gap-y-sm p-md">
                        <KeyValueInfo
                            keyText="Staking APY"
                            tooltipPosition={TooltipPosition.Right}
                            tooltipText="Annualized percentage yield based on past validator performance. Future APY may vary"
                            value={formatPercentageDisplay(apy, '--', isApyApproxZero)}
                            fullwidth
                        />
                        <KeyValueInfo
                            keyText="Stake Share"
                            tooltipPosition={TooltipPosition.Right}
                            tooltipText="Stake percentage managed by this validator."
                            value={formatPercentageDisplay(totalStakePercentage)}
                            fullwidth
                        />
                        <KeyValueInfo
                            keyText="Total Staked"
                            tooltipPosition={TooltipPosition.Right}
                            tooltipText="Stake percentage managed by this validator."
                            value={totalValidatorStakeFormatted}
                            supportingLabel={totalValidatorStakeSymbol}
                            fullwidth
                        />
                        <KeyValueInfo
                            keyText="Your Staked IOTA"
                            tooltipPosition={TooltipPosition.Right}
                            tooltipText="Your current staked balance."
                            value={totalStakeFormatted}
                            supportingLabel={totalStakeSymbol}
                            fullwidth
                        />
                    </div>
                </Panel>
            </div>
            <Input
                label="Amount"
                value={amount}
                onChange={onChange}
                placeholder="Enter amount to stake"
            />
            <div className="flex w-full justify-between gap-2">
                <Button onClick={onBack}>Back</Button>
                <Button onClick={onStake} disabled={isStakeDisabled}>
                    Stake
                </Button>
            </div>
        </div>
    );
}

export default EnterAmountView;
