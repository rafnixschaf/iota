// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import React from 'react';
import {
    useGetValidatorsApy,
    ExtendedDelegatedStake,
    ImageIcon,
    ImageIconSize,
    useFormatCoin,
    formatPercentageDisplay,
} from '@iota/core';
import {
    Header,
    Button,
    ButtonType,
    Card,
    CardBody,
    CardImage,
    CardType,
    Panel,
    KeyValueInfo,
    Badge,
    BadgeType,
    Divider,
    InfoBox,
    InfoBoxStyle,
    InfoBoxType,
    LoadingIndicator,
} from '@iota/apps-ui-kit';
import { Warning } from '@iota/ui-icons';
import { useIotaClientQuery } from '@iota/dapp-kit';
import { formatAddress, IOTA_TYPE_ARG } from '@iota/iota-sdk/utils';
import { Layout, LayoutFooter, LayoutBody } from './Layout';

interface StakeDialogProps {
    stakedDetails: ExtendedDelegatedStake;
    showActiveStatus?: boolean;
    handleClose: () => void;
    handleUnstake: () => void;
    handleStake: () => void;
}

export function DetailsView({
    handleClose,
    handleUnstake,
    handleStake,
    stakedDetails,
    showActiveStatus,
}: StakeDialogProps): JSX.Element {
    const totalStake = BigInt(stakedDetails?.principal || 0n);
    const validatorAddress = stakedDetails?.validatorAddress;
    const {
        data: system,
        isPending: loadingValidators,
        isError: errorValidators,
    } = useIotaClientQuery('getLatestIotaSystemState');
    const { data: rollingAverageApys } = useGetValidatorsApy();
    const { apy, isApyApproxZero } = rollingAverageApys?.[validatorAddress] ?? {
        apy: null,
    };
    const iotaEarned = BigInt(stakedDetails?.estimatedReward || 0n);
    const [iotaEarnedFormatted, iotaEarnedSymbol] = useFormatCoin(iotaEarned, IOTA_TYPE_ARG);
    const [totalStakeFormatted, totalStakeSymbol] = useFormatCoin(totalStake, IOTA_TYPE_ARG);

    // flag if the validator is at risk of being removed from the active set
    const isAtRisk = system?.atRiskValidators.some((item) => item[0] === validatorAddress);

    const validatorSummary =
        system?.activeValidators.find((validator) => validator.iotaAddress === validatorAddress) ||
        null;

    const validatorName = validatorSummary?.name || '';
    const stakingPoolActivationEpoch = Number(validatorSummary?.stakingPoolActivationEpoch || 0);
    const currentEpoch = Number(system?.epoch || 0);
    const commission = validatorSummary ? Number(validatorSummary.commissionRate) / 100 : 0;

    // flag as new validator if the validator was activated in the last epoch
    // for genesis validators, this will be false
    const newValidator = currentEpoch - stakingPoolActivationEpoch <= 1 && currentEpoch !== 0;
    const subtitle = showActiveStatus ? (
        <div className="flex items-center gap-1">
            {formatAddress(validatorAddress)}
            {newValidator && <Badge label="New" type={BadgeType.PrimarySoft} />}
            {isAtRisk && <Badge label="At Risk" type={BadgeType.PrimarySolid} />}
        </div>
    ) : (
        formatAddress(validatorAddress)
    );

    if (loadingValidators) {
        return (
            <div className="flex h-full w-full items-center justify-center p-2">
                <LoadingIndicator />
            </div>
        );
    }

    if (errorValidators) {
        return (
            <div className="mb-2 flex h-full w-full items-center justify-center p-2">
                <InfoBox
                    title="Something went wrong"
                    supportingText={'An error occurred'}
                    style={InfoBoxStyle.Default}
                    type={InfoBoxType.Error}
                    icon={<Warning />}
                />
            </div>
        );
    }

    return (
        <Layout>
            <Header title="Validator" onClose={handleClose} onBack={handleClose} titleCentered />
            <LayoutBody>
                <div className="flex w-full flex-col gap-md">
                    <Card type={CardType.Filled}>
                        <CardImage>
                            <ImageIcon
                                src={null}
                                label={validatorName}
                                fallback={validatorName}
                                size={ImageIconSize.Large}
                            />
                        </CardImage>
                        <CardBody title={validatorName} subtitle={subtitle} isTextTruncated />
                    </Card>
                    <Panel hasBorder>
                        <div className="flex flex-col gap-y-sm p-md">
                            <KeyValueInfo
                                keyText="Your Stake"
                                value={totalStakeFormatted}
                                supportingLabel={totalStakeSymbol}
                                fullwidth
                            />
                            <KeyValueInfo
                                keyText="Earned"
                                value={iotaEarnedFormatted}
                                supportingLabel={iotaEarnedSymbol}
                                fullwidth
                            />
                            <Divider />
                            <KeyValueInfo
                                keyText="APY"
                                value={formatPercentageDisplay(apy, '--', isApyApproxZero)}
                                fullwidth
                            />
                            <KeyValueInfo
                                keyText="Commission"
                                value={`${commission.toString()}%`}
                                fullwidth
                            />
                        </div>
                    </Panel>
                </div>
            </LayoutBody>
            <LayoutFooter>
                <div className="flex w-full gap-sm">
                    <Button
                        type={ButtonType.Secondary}
                        onClick={handleUnstake}
                        text="Unstake"
                        fullWidth
                    />
                    <Button
                        type={ButtonType.Primary}
                        text="Stake"
                        onClick={handleStake}
                        fullWidth
                    />
                </div>
            </LayoutFooter>
        </Layout>
    );
}
