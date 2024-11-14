// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Button,
    ButtonType,
    Card,
    CardBody,
    CardImage,
    CardType,
    ImageShape,
    ImageType,
} from '@iota/apps-ui-kit';
import { ValidatorApyData } from '@iota/core';
import { StakingTransactionDetails } from './StakingTransactionDetails';
import { Validator } from './Validator';
import { IotaLogoMark } from '@iota/ui-icons';

interface SuccessScreenViewProps {
    validatorAddress: string;
    gasBudget: string | number | null | undefined;
    onConfirm: () => void;
    amount: string;
    symbol: string | undefined;
    validatorApy: ValidatorApyData;
}

export function SuccessScreenView({
    validatorAddress,
    gasBudget,
    onConfirm,
    amount,
    symbol,
    validatorApy: { apy, isApyApproxZero },
}: SuccessScreenViewProps): React.JSX.Element {
    return (
        <div className="flex flex-1 flex-col">
            <div className="flex w-full flex-1 flex-col justify-between">
                <div className="flex flex-col gap-y-md">
                    <Validator address={validatorAddress} isSelected showAction={false} />

                    <Card type={CardType.Outlined}>
                        <CardImage type={ImageType.BgSolid} shape={ImageShape.Rounded}>
                            <IotaLogoMark className="h-5 w-5 text-neutral-10" />
                        </CardImage>
                        <CardBody title={`${amount} ${symbol}`} subtitle="Stake" />
                    </Card>

                    <StakingTransactionDetails
                        gasBudget={gasBudget}
                        apy={apy}
                        isApyApproxZero={isApyApproxZero}
                    />
                </div>
            </div>

            <div className="flex w-full">
                <Button
                    type={ButtonType.Primary}
                    fullWidth
                    onClick={onConfirm}
                    text="Confirm & Exit"
                />
            </div>
        </div>
    );
}
