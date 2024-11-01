// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import React, { useMemo } from 'react';
import {
    useGetValidatorsApy,
    ExtendedDelegatedStake,
    ImageIcon,
    ImageIconSize,
    useFormatCoin,
    formatPercentageDisplay,
} from '@iota/core';
import {
    Dialog,
    DialogBody,
    DialogContent,
    DialogPosition,
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
import { useUnstakeTransaction } from '@/hooks';
import {
    useCurrentAccount,
    useIotaClientQuery,
    useSignAndExecuteTransaction,
} from '@iota/dapp-kit';
import { formatAddress, IOTA_TYPE_ARG } from '@iota/iota-sdk/utils';
interface StakeDialogProps {
    stakedDetails: ExtendedDelegatedStake;
    showActiveStatus?: boolean;
    handleClose: () => void;
}
export function StakedDetailsDialog({
    handleClose,
    stakedDetails,
    showActiveStatus,
}: StakeDialogProps): JSX.Element {
    const account = useCurrentAccount();
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

    const { data: unstakeData } = useUnstakeTransaction(
        stakedDetails.stakedIotaId,
        account?.address || '',
    );
    const { mutateAsync: signAndExecuteTransaction } = useSignAndExecuteTransaction();

    // flag if the validator is at risk of being removed from the active set
    const isAtRisk = system?.atRiskValidators.some((item) => item[0] === validatorAddress);

    const validatorSummary = useMemo(() => {
        if (!system) return null;

        return (
            system.activeValidators.find(
                (validator) => validator.iotaAddress === validatorAddress,
            ) || null
        );
    }, [validatorAddress, system]);

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

    async function handleUnstake(): Promise<void> {
        if (!unstakeData) return;
        await signAndExecuteTransaction({
            transaction: unstakeData.transaction,
        });
    }

    function handleAddNewStake() {
        console.log('Stake');
    }

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
        <Dialog open onOpenChange={handleClose}>
            <DialogContent containerId="overlay-portal-container" position={DialogPosition.Right}>
                <div className="flex min-h-full flex-col">
                    <Header
                        title="Validator"
                        onClose={handleClose}
                        onBack={handleClose}
                        titleCentered
                    />
                    <div className="flex w-full flex-1 [&_>div]:flex [&_>div]:w-full [&_>div]:flex-col [&_>div]:justify-between">
                        <DialogBody>
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
                                    <CardBody
                                        title={validatorName}
                                        subtitle={subtitle}
                                        isTextTruncated
                                    />
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
                                            value={formatPercentageDisplay(
                                                apy,
                                                '--',
                                                isApyApproxZero,
                                            )}
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
                            <div>
                                <div className="my-3.75 flex w-full gap-2.5">
                                    <Button
                                        type={ButtonType.Secondary}
                                        onClick={handleUnstake}
                                        text="Unstake"
                                        fullWidth
                                    />
                                    <Button
                                        type={ButtonType.Primary}
                                        text="Stake"
                                        onClick={handleAddNewStake}
                                        disabled
                                        fullWidth
                                    />
                                </div>
                            </div>
                        </DialogBody>
                    </div>
                </div>
            </DialogContent>
        </Dialog>
    );
}
