// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

'use client';

import { FormDataValues } from '../SendTokenDialog';
import {
    Button,
    Card,
    CardType,
    CardImage,
    ImageType,
    CardBody,
    CardAction,
    CardActionType,
    KeyValueInfo,
    Divider,
    ButtonType,
} from '@iota/apps-ui-kit';
import { formatAddress } from '@iota/iota-sdk/utils';
import { GAS_SYMBOL } from '@/lib/constants';
import { CoinIcon, ImageIconSize, parseAmount, useCoinMetadata, useFormatCoin } from '@iota/core';
import { Loader } from '@iota/ui-icons';

interface ReviewValuesFormProps {
    formData: FormDataValues;
    senderAddress: string;
    error: string | undefined;
    isPending: boolean;
    executeTransfer: () => void;
    onBack: () => void;
    coinType: string;
}

function ReviewValuesFormView({
    formData: { amount, to, isPayAllIota, gasBudgetEst },
    senderAddress,
    isPending,
    error,
    executeTransfer,
    onBack,
    coinType,
}: ReviewValuesFormProps): JSX.Element {
    const { data: metadata } = useCoinMetadata(coinType);
    const amountWithoutDecimals = parseAmount(amount, metadata?.decimals ?? 0);
    const [formatAmount, symbol] = useFormatCoin(amountWithoutDecimals, coinType);

    return (
        <div className="flex h-full flex-col">
            <div className="flex h-full w-full flex-col gap-md">
                <div className="flex h-full flex-col justify-between">
                    <div className="h-full flex-1">
                        <div className="flex w-full flex-col gap-md">
                            {Number(amount) !== 0 ? (
                                <Card type={CardType.Filled}>
                                    <CardImage type={ImageType.BgSolid}>
                                        <CoinIcon
                                            coinType={coinType}
                                            size={ImageIconSize.Small}
                                            rounded
                                            hasCoinWrapper
                                        />
                                    </CardImage>
                                    <CardBody
                                        title={`${isPayAllIota ? '~' : ''}${formatAmount} ${symbol}`}
                                        subtitle="Amount"
                                    />
                                    <CardAction type={CardActionType.SupportingText} />
                                </Card>
                            ) : null}
                            <div className="flex flex-col gap-md--rs p-sm--rs">
                                <KeyValueInfo
                                    keyText={'From'}
                                    value={formatAddress(senderAddress)}
                                    fullwidth
                                />

                                <Divider />
                                <KeyValueInfo
                                    keyText={'To'}
                                    value={formatAddress(to || '')}
                                    fullwidth
                                />

                                <Divider />
                                <KeyValueInfo
                                    keyText={'Est. Gas Fees'}
                                    value={gasBudgetEst}
                                    supportingLabel={GAS_SYMBOL}
                                    fullwidth
                                />
                            </div>
                        </div>
                    </div>
                    <Button
                        type={ButtonType.Primary}
                        onClick={executeTransfer}
                        text="Send Now"
                        disabled={coinType === null || isPending}
                        fullWidth
                        icon={isPending ? <Loader className="animate-spin" /> : undefined}
                        iconAfterText
                    />
                </div>
            </div>
        </div>
    );
}
export default ReviewValuesFormView;
