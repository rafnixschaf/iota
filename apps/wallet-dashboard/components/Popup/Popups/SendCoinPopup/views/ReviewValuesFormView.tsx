// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SendAndReviewDialog } from '@/components/Dialogs';
import { FormDataValues } from '../SendCoinPopup';
import { useState } from 'react';

interface ReviewValuesFormProps {
    formData: FormDataValues;
    senderAddress: string;
    gasBudget: string;
    error: string | undefined;
    isPending: boolean;
    executeTransfer: () => void;
    onBack: () => void;
    coinType: string;
    onClose: () => void;
}

function ReviewValuesFormView({
    formData: { amount, recipientAddress },
    senderAddress,
    gasBudget,
    isPending,
    executeTransfer,
    onBack,
    coinType,
    onClose,
}: ReviewValuesFormProps): JSX.Element {
    const [open, setOpen] = useState(true);
    return (
        <SendAndReviewDialog
            to={recipientAddress}
            amount={amount}
            coinType={coinType}
            onSend={executeTransfer}
            open={open}
            setOpen={setOpen}
            gasBudget={gasBudget}
            isPending={isPending}
            senderAddress={senderAddress}
            onClose={() => {
                setOpen(false);
                onClose();
            }}
            onBack={onBack}
        />
    );
}
export default ReviewValuesFormView;
