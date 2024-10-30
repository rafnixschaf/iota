// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { GAS_SYMBOL } from '@/lib/constants';
import {
    Button,
    Dialog,
    DialogContent,
    DialogBody,
    Header,
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
    DialogPosition,
} from '@iota/apps-ui-kit';
import { CoinIcon, ImageIconSize, parseAmount, useCoinMetadata, useFormatCoin } from '@iota/core';
import { formatAddress } from '@iota/iota-sdk/utils';
import { Loader } from '@iota/ui-icons';

export type SendAndReviewDialogProps = {
    coinType: string;
    to: string;
    amount: string;
    approximation?: boolean;
    gasBudget?: string;
    open: boolean;
    setOpen?: (open: boolean) => void;
    onSend: () => void;
    isPending?: boolean;
    senderAddress: string;
    onClose: () => void;
    onBack: () => void;
};

export function SendAndReviewDialog({
    coinType,
    senderAddress,
    to,
    amount,
    approximation,
    gasBudget,
    open,
    setOpen,
    onSend,
    isPending,
    onClose,
    onBack,
}: SendAndReviewDialogProps): React.JSX.Element {
    const { data: metadata } = useCoinMetadata(coinType);
    const amountWithoutDecimals = parseAmount(amount, metadata?.decimals ?? 0);
    const [formatAmount, symbol] = useFormatCoin(Math.abs(Number(amountWithoutDecimals)), coinType);

    return (
        <Dialog open={open} onOpenChange={setOpen}>
            <DialogContent containerId="overlay-portal-container" position={DialogPosition.Right}>
                <Header title="Review & Send" onClose={onClose} onBack={onBack} />
                <div className="flex h-full flex-col">
                    <div className="flex h-full w-full flex-col gap-md">
                        <div className="flex h-full flex-col justify-between">
                            <DialogBody>
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
                                                    title={`${approximation ? '~' : ''}${formatAmount} ${symbol}`}
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
                                                value={gasBudget}
                                                supportingLabel={GAS_SYMBOL}
                                                fullwidth
                                            />
                                        </div>
                                    </div>
                                </div>
                            </DialogBody>
                            <DialogBody>
                                <Button
                                    type={ButtonType.Primary}
                                    onClick={onSend}
                                    text="Send Now"
                                    disabled={coinType === null || isPending}
                                    fullWidth
                                    icon={
                                        isPending ? <Loader className="animate-spin" /> : undefined
                                    }
                                    iconAfterText
                                />
                            </DialogBody>
                        </div>
                    </div>
                </div>
            </DialogContent>
        </Dialog>
    );
}
