// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type Meta, type StoryObj } from '@storybook/react';
import { useState } from 'react';

import { NetworkSelect, type NetworkSelectProps } from '~/ui/header/NetworkSelect';
import { Network } from '~/utils/api/DefaultRpcClient';

export default {
	component: NetworkSelect,
	decorators: [
		(Story) => (
			<div className="flex justify-end bg-headerNav p-6">
				<Story />
			</div>
		),
	],
} as Meta;

const NETWORKS = Object.entries(Network).map(([label, id]) => ({
	id,
	label,
}));

export const Default: StoryObj<NetworkSelectProps> = {
	render: (args) => {
		const [network, setNetwork] = useState<string>(NETWORKS[0].id);

		return <NetworkSelect {...args} value={network} version="1" onChange={setNetwork} />;
	},
	args: {
		networks: NETWORKS,
	},
};
