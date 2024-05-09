// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useSuiClientQuery } from '@mysten/dapp-kit';
import { useContext } from 'react';

import { NetworkContext } from '../../context';
import { Network, NetworkConfigs } from '../../utils/api/DefaultRpcClient';
import { NetworkSelect } from '~/ui/header/NetworkSelect';
import { ampli } from '~/utils/analytics/ampli';

export default function WrappedNetworkSelect() {
	const [network, setNetwork] = useContext(NetworkContext);
	const { data } = useSuiClientQuery('getLatestSuiSystemState');
	const { data: binaryVersion } = useSuiClientQuery('getRpcApiVersion');

	const networks: { id: Network; label: string }[] = [
		{ id: Network.MAINNET, label: 'Mainnet' },
		{ id: Network.TESTNET, label: 'Testnet' },
		{ id: Network.DEVNET, label: 'Devnet' },
		{ id: Network.LOCAL, label: 'Local' },
		{ id: Network.ALPHANET, label: 'Alphanet' },
	].filter(Boolean);

	const filteredNetworks = networks.filter((network) => Boolean(NetworkConfigs[network.id].url));

	return (
		<NetworkSelect
			value={network}
			onChange={(networkId) => {
				ampli.switchedNetwork({ toNetwork: networkId });
				setNetwork(networkId);
			}}
			networks={filteredNetworks}
			version={data?.protocolVersion}
			binaryVersion={binaryVersion}
		/>
	);
}
