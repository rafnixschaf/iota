// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

'use client';

import { AmountBox, Box, StakeCard, StakeDialog, Button, StakedDetailsDialog } from '@/components';
import {
    ExtendedDelegatedStake,
    formatDelegatedStake,
    useFormatCoin,
    useGetDelegatedStake,
    useTotalDelegatedRewards,
    useTotalDelegatedStake,
    DELEGATED_STAKES_QUERY_REFETCH_INTERVAL,
    DELEGATED_STAKES_QUERY_STALE_TIME,
} from '@iota/core';
import { useCurrentAccount } from '@iota/dapp-kit';
import { IOTA_TYPE_ARG } from '@iota/iota-sdk/utils';
import { useState } from 'react';

function StakingDashboardPage(): JSX.Element {
    const account = useCurrentAccount();
    const [isDialogStakeOpen, setIsDialogStakeOpen] = useState(false);
    const [stakedDetails, setStakedDetails] = useState<ExtendedDelegatedStake | null>(null);
    const { data: delegatedStakeData } = useGetDelegatedStake({
        address: account?.address || '',
        staleTime: DELEGATED_STAKES_QUERY_STALE_TIME,
        refetchInterval: DELEGATED_STAKES_QUERY_REFETCH_INTERVAL,
    });

    const extendedStakes = delegatedStakeData ? formatDelegatedStake(delegatedStakeData) : [];
    const totalDelegatedStake = useTotalDelegatedStake(extendedStakes);
    const totalDelegatedRewards = useTotalDelegatedRewards(extendedStakes);
    const [formattedDelegatedStake, stakeSymbol, stakeResult] = useFormatCoin(
        totalDelegatedStake,
        IOTA_TYPE_ARG,
    );
    const [formattedDelegatedRewards, rewardsSymbol, rewardsResult] = useFormatCoin(
        totalDelegatedRewards,
        IOTA_TYPE_ARG,
    );

    const viewStakeDetails = (extendedStake: ExtendedDelegatedStake) => {
        setStakedDetails(extendedStake);
    };
    function handleCloseStakedDetails() {
        setStakedDetails(null);
    }

    function handleNewStake() {
        setIsDialogStakeOpen(true);
    }
    console.log(stakedDetails);

    return (
        <>
            <div className="flex flex-col items-center justify-center gap-4 pt-12">
                <AmountBox
                    title="Currently staked"
                    amount={
                        stakeResult.isPending ? '-' : `${formattedDelegatedStake} ${stakeSymbol}`
                    }
                />
                <AmountBox
                    title="Earned"
                    amount={`${
                        rewardsResult.isPending ? '-' : formattedDelegatedRewards
                    } ${rewardsSymbol}`}
                />
                <Box title="Stakes">
                    <div className="flex flex-col items-center gap-4">
                        <h1>List of stakes</h1>
                        {extendedStakes?.map((extendedStake) => (
                            <StakeCard
                                key={extendedStake.stakedIotaId}
                                extendedStake={extendedStake}
                                onDetailsClick={viewStakeDetails}
                            />
                        ))}
                    </div>
                </Box>
                <Button onClick={handleNewStake}>New Stake</Button>
            </div>
            <StakeDialog isOpen={isDialogStakeOpen} setOpen={setIsDialogStakeOpen} />;
            {!!stakedDetails && (
                <StakedDetailsDialog
                    stakedDetails={stakedDetails}
                    handleClose={handleCloseStakedDetails}
                />
            )}
        </>
    );
}

export default StakingDashboardPage;
