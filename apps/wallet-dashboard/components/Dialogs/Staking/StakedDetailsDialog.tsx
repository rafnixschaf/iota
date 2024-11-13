// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import React from 'react';
import {
    ExtendedDelegatedStake,
    formatPercentageDisplay,
    ImageIcon,
    ImageIconSize,
    useFormatCoin,
    useValidatorInfo,
} from '@iota/core';
import {
    Badge,
    BadgeType,
    Button,
    ButtonType,
    Card,
    CardBody,
    CardImage,
    CardType,
    Dialog,
    DialogBody,
    DialogContent,
    DialogPosition,
    Divider,
    Header,
    InfoBox,
    InfoBoxStyle,
    InfoBoxType,
    KeyValueInfo,
    LoadingIndicator,
    Panel,
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
    const { isPending: loadingValidators, isError: errorValidators } = useIotaClientQuery(
        'getLatestIotaSystemState',
    );
    const iotaEarned = BigInt(stakedDetails?.estimatedReward || 0n);
    const [iotaEarnedFormatted, iotaEarnedSymbol] = useFormatCoin(iotaEarned, IOTA_TYPE_ARG);
    const [totalStakeFormatted, totalStakeSymbol] = useFormatCoin(totalStake, IOTA_TYPE_ARG);

    const { name, commission, newValidator, isAtRisk, apy, isApyApproxZero } = useValidatorInfo({
        validatorAddress: validatorAddress,
    });

    const { data: unstakeData } = useUnstakeTransaction(
        stakedDetails.stakedIotaId,
        account?.address || '',
    );
    const { mutateAsync: signAndExecuteTransaction } = useSignAndExecuteTransaction();

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
        // pass
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
                                            label={name}
                                            fallback={name}
                                            size={ImageIconSize.Large}
                                        />
                                    </CardImage>
                                    <CardBody title={name} subtitle={subtitle} isTextTruncated />
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
