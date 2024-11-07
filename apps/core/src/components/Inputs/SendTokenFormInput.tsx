import { ButtonPill, InputType } from '@iota/apps-ui-kit';
import { CoinStruct } from '@iota/iota-sdk/client';
import FormInput from './FormInput';

export interface SendTokenInputProps {
    gasBudgetEstimation: string;
    coins?: CoinStruct[];
    symbol: string;
    values: {
        amount: string;
        isPayAllIota: boolean;
    };
    onActionClick: () => Promise<void>;
    isActionButtonDisabled?: boolean | 'auto';
    value: string;
    onChange: (value: string) => void;
    onBlur?: React.FocusEventHandler<HTMLInputElement>;
    errorMessage?: string;
}

export function SendTokenFormInput({
    gasBudgetEstimation,
    coins,
    values,
    symbol,
    onActionClick,
    isActionButtonDisabled,
    value,
    onChange,
    onBlur,
    errorMessage,
}: SendTokenInputProps) {
    return (
        <FormInput
            type={InputType.NumericFormat}
            name="amount"
            label="Send Amount"
            placeholder="0.00"
            caption="Est. Gas Fees:"
            suffix={` ${symbol}`}
            decimals
            allowNegative={false}
            prefix={values.isPayAllIota ? '~ ' : undefined}
            amountCounter={coins ? gasBudgetEstimation : '--'}
            value={value}
            onChange={onChange}
            onBlur={onBlur}
            errorMessage={errorMessage}
            renderAction={(isButtonDisabled) => (
                <ButtonPill
                    disabled={
                        isActionButtonDisabled === 'auto'
                            ? isButtonDisabled
                            : isActionButtonDisabled
                    }
                    onClick={onActionClick}
                >
                    Max
                </ButtonPill>
            )}
        />
    );
}
