// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { useMemo } from 'react';
import { IOTA_TYPE_ARG } from '@iota/iota-sdk/utils';
import {
    formatPercentageDisplay,
    calculateStakeShare,
    useFormatCoin,
    getTokenStakeIotaForValidator,
    useGetDelegatedStake,
    DELEGATED_STAKES_QUERY_REFETCH_INTERVAL,
    DELEGATED_STAKES_QUERY_STALE_TIME,
} from '@iota/core';
import { KeyValueInfo, Panel, TooltipPosition } from '@iota/apps-ui-kit';
import { useValidatorInfo } from '@/hooks';

export function StakedInfo({
    validatorAddress,
    accountAddress,
}: {
    validatorAddress: string;
    accountAddress: string;
}) {
    const { data: delegatedStake } = useGetDelegatedStake({
        address: accountAddress || '',
        staleTime: DELEGATED_STAKES_QUERY_STALE_TIME,
        refetchInterval: DELEGATED_STAKES_QUERY_REFETCH_INTERVAL,
    });
    const { apy, isApyApproxZero, validatorSummary, system } = useValidatorInfo({
        validatorAddress: validatorAddress,
    });

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

    const totalStake = useMemo(() => {
        if (!delegatedStake) return 0n;
        return getTokenStakeIotaForValidator(delegatedStake, validatorAddress);
    }, [delegatedStake, validatorAddress]);

    //TODO: verify this is the correct validator stake balance
    const totalValidatorStake = validatorSummary?.stakingPoolIotaBalance || 0;

    const [totalValidatorStakeFormatted, totalValidatorStakeSymbol] = useFormatCoin(
        totalValidatorStake,
        IOTA_TYPE_ARG,
    );
    const [totalStakeFormatted, totalStakeSymbol] = useFormatCoin(totalStake, IOTA_TYPE_ARG);

    return (
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
    );
}
