// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import React from 'react';
import { useFormatCoin, useBalance, CoinFormat, ValidatorApyData } from '@iota/core';
import { IOTA_TYPE_ARG } from '@iota/iota-sdk/utils';
import { Button, ButtonType, Input, InputType, Header } from '@iota/apps-ui-kit';
import { useCurrentAccount } from '@iota/dapp-kit';
import { Validator } from './Validator';
import { StakedInfo } from './StakedInfo';
import { Layout, LayoutBody, LayoutFooter } from './Layout';
import { StakingTransactionDetails } from './StakingTransactionDetails';

interface EnterAmountViewProps {
    selectedValidator: string;
    amount: string;
    onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
    onBack: () => void;
    onStake: () => void;
    showActiveStatus?: boolean;
    gasBudget?: string | number | null;
    handleClose: () => void;
    validatorApy: ValidatorApyData;
}

function EnterAmountView({
    selectedValidator: selectedValidatorAddress,
    amount,
    onChange,
    onBack,
    onStake,
    gasBudget = 0,
    handleClose,
    validatorApy,
}: EnterAmountViewProps): JSX.Element {
    const account = useCurrentAccount();
    const accountAddress = account?.address;

    const { data: iotaBalance } = useBalance(accountAddress!);

    const coinBalance = BigInt(iotaBalance?.totalBalance || 0);
    const maxTokenBalance = coinBalance - BigInt(Number(gasBudget));
    const [maxTokenFormatted, maxTokenFormattedSymbol] = useFormatCoin(
        maxTokenBalance,
        IOTA_TYPE_ARG,
        CoinFormat.FULL,
    );

    return (
        <Layout>
            <Header title="Enter amount" onClose={handleClose} onBack={handleClose} titleCentered />
            <LayoutBody>
                <div className="flex w-full flex-col justify-between">
                    <div>
                        <Validator
                            address={selectedValidatorAddress}
                            isSelected
                            showAction={false}
                        />
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
                        <StakingTransactionDetails gasBudget={gasBudget} {...validatorApy} />
                    </div>
                </div>
            </LayoutBody>
            <LayoutFooter>
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
            </LayoutFooter>
        </Layout>
    );
}

export default EnterAmountView;
