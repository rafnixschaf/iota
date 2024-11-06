// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import React from 'react';
import { useFormatCoin, useBalance, CoinFormat } from '@iota/core';
import { IOTA_TYPE_ARG } from '@iota/iota-sdk/utils';
import {
    Button,
    ButtonType,
    KeyValueInfo,
    Panel,
    Divider,
    Input,
    InputType,
} from '@iota/apps-ui-kit';
import { useStakeTxnInfo } from '../hooks';
import { useCurrentAccount, useIotaClientQuery } from '@iota/dapp-kit';
import { Validator } from './Validator';
import { StakedInfo } from './StakedInfo';

interface EnterAmountViewProps {
    selectedValidator: string;
    amount: string;
    onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
    onBack: () => void;
    onStake: () => void;
    showActiveStatus?: boolean;
    gasBudget?: string | number | null;
}

function EnterAmountView({
    selectedValidator: selectedValidatorAddress,
    amount,
    showActiveStatus,
    onChange,
    onBack,
    onStake,
    gasBudget = 0,
}: EnterAmountViewProps): JSX.Element {
    const account = useCurrentAccount();
    const accountAddress = account?.address;

    const { data: system } = useIotaClientQuery('getLatestIotaSystemState');
    const { data: iotaBalance } = useBalance(accountAddress!);

    const coinBalance = BigInt(iotaBalance?.totalBalance || 0);
    const maxTokenBalance = coinBalance - BigInt(Number(gasBudget));
    const [maxTokenFormatted, maxTokenFormattedSymbol] = useFormatCoin(
        maxTokenBalance,
        IOTA_TYPE_ARG,
        CoinFormat.FULL,
    );
    const [gas, symbol] = useFormatCoin(gasBudget, IOTA_TYPE_ARG);
    const { stakedRewardsStartEpoch, timeBeforeStakeRewardsRedeemableAgoDisplay } = useStakeTxnInfo(
        system?.epoch,
    );

    return (
        <div className="flex w-full flex-col justify-between">
            <div>
                <Validator address={selectedValidatorAddress} isSelected showAction={false} />
                <StakedInfo
                    validatorAddress={selectedValidatorAddress}
                    accountAddress={accountAddress!}
                />
                <div className="my-md w-full">
                    <Input
                        type={InputType.NumericFormat}
                        label="Amount"
                        value={amount}
                        onChange={onChange}
                        placeholder="Enter amount to stake"
                        caption={`${maxTokenFormatted} ${maxTokenFormattedSymbol} Available`}
                    />
                </div>

                <Panel hasBorder>
                    <div className="flex flex-col gap-y-sm p-md">
                        <KeyValueInfo
                            keyText="Staking Rewards Start"
                            value={stakedRewardsStartEpoch}
                            fullwidth
                        />
                        <KeyValueInfo
                            keyText="Redeem Rewards"
                            value={timeBeforeStakeRewardsRedeemableAgoDisplay}
                            fullwidth
                        />
                        <Divider />
                        <KeyValueInfo
                            keyText="Gas fee"
                            value={gas || '--'}
                            supportingLabel={symbol}
                            fullwidth
                        />
                    </div>
                </Panel>
            </div>

            <div className="flex w-full justify-between gap-sm">
                <Button fullWidth type={ButtonType.Secondary} onClick={onBack} text="Back" />
                <Button
                    fullWidth
                    type={ButtonType.Primary}
                    onClick={onStake}
                    disabled={!amount}
                    text="Stake"
                />
            </div>
        </div>
    );
}

export default EnterAmountView;
