// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { IOTAClientProvider } from '@iota/dapp-kit';
import { type Meta, type StoryObj } from '@storybook/react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

import { CoinBalance, type CoinBalanceProps } from '../CoinBalance';
import { Network, SupportedNetworks, createIOTAClient } from '~/utils/api/DefaultRpcClient';

export default {
    component: CoinBalance,
    decorators: [
        (Story) => (
            <QueryClientProvider client={new QueryClient()}>
                <IOTAClientProvider
                    networks={SupportedNetworks}
                    defaultNetwork={Network.Local}
                    createClient={createIOTAClient}
                >
                    <Story />
                </IOTAClientProvider>
            </QueryClientProvider>
        ),
    ],
} as Meta;

export const Default: StoryObj<CoinBalanceProps> = {
    args: {
        amount: 1000,
        coinType: '0x2::iota::IOTA',
    },
};

export const WithoutSymbol: StoryObj<CoinBalanceProps> = {
    args: {
        amount: 10000,
    },
};
