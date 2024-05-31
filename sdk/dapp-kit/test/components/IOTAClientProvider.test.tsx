// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { IOTAClient } from '@iota/iota.js/client';
import { screen } from '@testing-library/dom';
import { render } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { useState } from 'react';

import { IOTAClientProvider } from '../../src/components/IOTAClientProvider.js';
import { useIOTAClient, useIOTAClientContext } from '../../src/index.js';

describe('IOTAClientProvider', () => {
	it('renders without crashing', () => {
		render(
			<IOTAClientProvider>
				<div>Test</div>
			</IOTAClientProvider>,
		);
		expect(screen.getByText('Test')).toBeInTheDocument();
	});

	it('provides a IOTAClient instance to its children', () => {
		const ChildComponent = () => {
			const client = useIOTAClient();
			expect(client).toBeInstanceOf(IOTAClient);
			return <div>Test</div>;
		};

		render(
			<IOTAClientProvider>
				<ChildComponent />
			</IOTAClientProvider>,
		);
	});

	it('can accept pre-configured IOTAClients', () => {
		const iotaClient = new IOTAClient({ url: 'http://localhost:8080' });
		const ChildComponent = () => {
			const client = useIOTAClient();
			expect(client).toBeInstanceOf(IOTAClient);
			expect(client).toBe(iotaClient);
			return <div>Test</div>;
		};

		render(
			<IOTAClientProvider networks={{ localnet: iotaClient }}>
				<ChildComponent />
			</IOTAClientProvider>,
		);

		expect(screen.getByText('Test')).toBeInTheDocument();
	});

	test('can create iota clients with custom options', async () => {
		function NetworkSelector() {
			const ctx = useIOTAClientContext();

			return (
				<div>
					{Object.keys(ctx.networks).map((network) => (
						<button key={network} onClick={() => ctx.selectNetwork(network)}>
							{`select ${network}`}
						</button>
					))}
				</div>
			);
		}
		function CustomConfigProvider() {
			const [selectedNetwork, setSelectedNetwork] = useState<string>();

			return (
				<IOTAClientProvider
					networks={{
						a: {
							url: 'http://localhost:8080',
							custom: setSelectedNetwork,
						},
						b: {
							url: 'http://localhost:8080',
							custom: setSelectedNetwork,
						},
					}}
					createClient={(name, { custom, ...config }) => {
						custom(name);
						return new IOTAClient(config);
					}}
				>
					<div>{`selected network: ${selectedNetwork}`}</div>
					<NetworkSelector />
				</IOTAClientProvider>
			);
		}

		const user = userEvent.setup();

		render(<CustomConfigProvider />);

		expect(screen.getByText('selected network: a')).toBeInTheDocument();

		await user.click(screen.getByText('select b'));

		expect(screen.getByText('selected network: b')).toBeInTheDocument();
	});

	test('controlled mode', async () => {
		function NetworkSelector(props: { selectNetwork: (network: string) => void }) {
			const ctx = useIOTAClientContext();

			return (
				<div>
					<div>{`selected network: ${ctx.network}`}</div>
					{Object.keys(ctx.networks).map((network) => (
						<button key={network} onClick={() => props.selectNetwork(network)}>
							{`select ${network}`}
						</button>
					))}
				</div>
			);
		}

		function ControlledProvider() {
			const [selectedNetwork, setSelectedNetwork] = useState<'a' | 'b'>('a');

			return (
				<IOTAClientProvider
					networks={{
						a: {
							url: 'http://localhost:8080',
							custom: setSelectedNetwork,
						},
						b: {
							url: 'http://localhost:8080',
							custom: setSelectedNetwork,
						},
					}}
					network={selectedNetwork}
				>
					<NetworkSelector
						selectNetwork={(network) => {
							setSelectedNetwork(network as 'a' | 'b');
						}}
					/>
				</IOTAClientProvider>
			);
		}

		const user = userEvent.setup();

		render(<ControlledProvider />);

		expect(screen.getByText('selected network: a')).toBeInTheDocument();

		await user.click(screen.getByText('select b'));

		expect(screen.getByText('selected network: b')).toBeInTheDocument();
	});

	test('onNetworkChange', async () => {
		function NetworkSelector() {
			const ctx = useIOTAClientContext();

			return (
				<div>
					<div>{`selected network: ${ctx.network}`}</div>
					{Object.keys(ctx.networks).map((network) => (
						<button key={network} onClick={() => ctx.selectNetwork(network)}>
							{`select ${network}`}
						</button>
					))}
				</div>
			);
		}

		function ControlledProvider() {
			const [selectedNetwork, setSelectedNetwork] = useState<string>('a');

			return (
				<IOTAClientProvider
					networks={{
						a: {
							url: 'http://localhost:8080',
							custom: setSelectedNetwork,
						},
						b: {
							url: 'http://localhost:8080',
							custom: setSelectedNetwork,
						},
					}}
					network={selectedNetwork as 'a' | 'b'}
					onNetworkChange={(network) => {
						setSelectedNetwork(network);
					}}
				>
					<NetworkSelector />
				</IOTAClientProvider>
			);
		}

		const user = userEvent.setup();

		render(<ControlledProvider />);

		expect(screen.getByText('selected network: a')).toBeInTheDocument();

		await user.click(screen.getByText('select b'));

		expect(screen.getByText('selected network: b')).toBeInTheDocument();
	});
});
