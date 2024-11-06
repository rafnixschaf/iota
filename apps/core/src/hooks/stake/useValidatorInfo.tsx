// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { useMemo } from 'react';
import { useIotaClientQuery } from '@iota/dapp-kit';
import { useGetValidatorsApy } from '../';

export function useValidatorInfo({ validatorAddress }: { validatorAddress: string }) {
    const { data: system } = useIotaClientQuery('getLatestIotaSystemState');
    const { data: rollingAverageApys } = useGetValidatorsApy();

    const validatorSummary = useMemo(() => {
        if (!system) return null;

        return (
            system.activeValidators.find(
                (validator) => validator.iotaAddress === validatorAddress,
            ) || null
        );
    }, [validatorAddress, system]);

    const currentEpoch = Number(system?.epoch || 0);

    //TODO: verify this is the correct validator stake balance
    const totalValidatorStake = validatorSummary?.stakingPoolIotaBalance || 0;

    const stakingPoolActivationEpoch = Number(validatorSummary?.stakingPoolActivationEpoch || 0);

    // flag as new validator if the validator was activated in the last epoch
    // for genesis validators, this will be false
    const newValidator = currentEpoch - stakingPoolActivationEpoch <= 1 && currentEpoch !== 0;

    // flag if the validator is at risk of being removed from the active set
    const isAtRisk = system?.atRiskValidators.some((item) => item[0] === validatorAddress);

    const { apy, isApyApproxZero } = rollingAverageApys?.[validatorAddress] ?? {
        apy: null,
    };

    return {
        system,
        validatorSummary,
        name: validatorSummary?.name || '',
        stakingPoolActivationEpoch,
        commission: validatorSummary ? Number(validatorSummary.commissionRate) / 100 : 0,
        newValidator,
        isAtRisk,
        apy,
        isApyApproxZero,
        totalValidatorStake,
    };
}
