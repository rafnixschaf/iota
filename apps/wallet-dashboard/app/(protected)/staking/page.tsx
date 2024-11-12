// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

'use client';

import { StakeDialog } from '@/components';
import { StakeDetailsDialog } from '@/components/Dialogs';
import { StartStaking } from '@/components/staking-overview/StartStaking';
import { Button, ButtonSize, ButtonType, InfoBox, InfoBoxStyle, InfoBoxType, Panel, Title, TitleSize } from '@iota/apps-ui-kit';
import {
    ExtendedDelegatedStake,
    formatDelegatedStake,
    useGetDelegatedStake,
    useTotalDelegatedRewards,
    useTotalDelegatedStake,
    DELEGATED_STAKES_QUERY_REFETCH_INTERVAL,
    DELEGATED_STAKES_QUERY_STALE_TIME,
    StakingCard,
    StakingStats,
} from '@iota/core';
import { useCurrentAccount, useIotaClientQuery } from '@iota/dapp-kit';
import { IotaSystemStateSummary } from '@iota/iota-sdk/client';
import { Info } from '@iota/ui-icons';
import { useMemo, useState } from 'react';

function StakingDashboardPage(): JSX.Element {
    const account = useCurrentAccount();
    const [isDialogStakeOpen, setIsDialogStakeOpen] = useState(false);
    const [selectedStake, setSelectedStake] = useState<ExtendedDelegatedStake | null>(null);
    const { data: system } = useIotaClientQuery('getLatestIotaSystemState');
    const activeValidators = (system as IotaSystemStateSummary)?.activeValidators;

    const { data: delegatedStakeData } = useGetDelegatedStake({
        address: account?.address || '',
        staleTime: DELEGATED_STAKES_QUERY_STALE_TIME,
        refetchInterval: DELEGATED_STAKES_QUERY_REFETCH_INTERVAL,
    });

    const extendedStakes = delegatedStakeData ? formatDelegatedStake(delegatedStakeData) : [];
    const totalDelegatedStake = useTotalDelegatedStake(extendedStakes);
    const totalDelegatedRewards = useTotalDelegatedRewards(extendedStakes);

    const delegations = useMemo(() => {
        return delegatedStakeData?.flatMap((delegation) => {
            return delegation.stakes.map((d) => ({
                ...d,
                // flag any inactive validator for the stakeIota object
                // if the stakingPoolId is not found in the activeValidators list flag as inactive
                inactiveValidator: !activeValidators?.find(
                    ({ stakingPoolId }) => stakingPoolId === delegation.stakingPool,
                ),
                validatorAddress: delegation.validatorAddress,
            }));
        });
    }, [activeValidators, delegatedStakeData]);

    // Check if there are any inactive validators
    const hasInactiveValidatorDelegation = delegations?.some(
        ({ inactiveValidator }) => inactiveValidator,
    );

    const viewStakeDetails = (extendedStake: ExtendedDelegatedStake) => {
        setSelectedStake(extendedStake);
    };

    function handleNewStake() {
        setIsDialogStakeOpen(true);
    }

    return (delegatedStakeData?.length ?? 0) > 0 ? (
        <Panel>
            <Title
                title="Staking"
                trailingElement={
                    <Button
                        onClick={() => handleNewStake()}
                        size={ButtonSize.Small}
                        type={ButtonType.Primary}
                        text="Stake"
                    />
                }
            />
            <div className="flex h-full w-full flex-col flex-nowrap gap-md p-md--rs">
                <div className="flex gap-xs">
                    <StakingStats title="Your stake" balance={totalDelegatedStake} />
                    <StakingStats title="Earned" balance={totalDelegatedRewards} />
                </div>
                <Title title="In progress" size={TitleSize.Small} />
                <div className="flex max-h-[420px] w-full flex-1 flex-col items-start overflow-auto">
                    {hasInactiveValidatorDelegation ? (
                        <div className="mb-3">
                            <InfoBox
                                type={InfoBoxType.Default}
                                title="Earn with active validators"
                                supportingText="Unstake IOTA from the inactive validators and stake on an active
validator to start earning rewards again."
                                icon={<Info />}
                                style={InfoBoxStyle.Elevated}
                            />
                        </div>
                    ) : null}
                    <div className="w-full gap-2">
                        {system &&
                            delegations
                                ?.filter(({ inactiveValidator }) => inactiveValidator)
                                .map((delegation) => (
                                    <StakingCard
                                        extendedStake={delegation}
                                        currentEpoch={Number(system.epoch)}
                                        key={delegation.stakedIotaId}
                                        inactiveValidator
                                        onClick={() => viewStakeDetails(delegation)}
                                    />
                                ))}
                    </div>
                    <div className="w-full gap-2">
                        {system &&
                            delegations
                                ?.filter(({ inactiveValidator }) => !inactiveValidator)
                                .map((delegation) => (
                                    <StakingCard
                                        extendedStake={delegation}
                                        currentEpoch={Number(system.epoch)}
                                        key={delegation.stakedIotaId}
                                        onClick={() => viewStakeDetails(delegation)}
                                    />
                                ))}
                    </div>
                </div>
            </div>
            <StakeDialog isOpen={isDialogStakeOpen} setOpen={setIsDialogStakeOpen} />
            {selectedStake && (
                <StakeDetailsDialog
                    extendedStake={selectedStake}
                    handleClose={() => setSelectedStake(null)}
                    showActiveStatus
                />
            )}
        </Panel>
    ) : (
        <StartStaking />
    );
}

export default StakingDashboardPage;
