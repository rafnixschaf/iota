// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const guides = [
	{
		type: 'doc',
		label: 'Guides',
		id: 'guides',
	},
	{
		type: 'category',
		label: 'Developer Guides',
		collapsed: false,
		link: {
			type: 'doc',
			id: 'guides/developer',
		},
		items: [
			{
				type: 'category',
				label: 'Getting Started',
				collapsed: false,
				link: {
					type: 'doc',
					id: 'guides/developer/getting-started',
				},
				items: [
					'guides/developer/getting-started/iota-environment',
					'guides/developer/getting-started/iota-install',
					'guides/developer/getting-started/connect',
					'guides/developer/getting-started/local-network',
					'guides/developer/getting-started/get-address',
					'guides/developer/getting-started/get-coins',
					'guides/developer/getting-started/graphql-rpc',
				],
			},
			{
				type: 'category',
				label: 'From Solidity/EVM to Move',
				collapsed: true,
				link: {
					type: 'doc',
					id: 'guides/developer/evm-to-move',
				},
				items: [
					'guides/developer/evm-to-move/why-move',
					'guides/developer/evm-to-move/tooling-apis',
					'guides/developer/evm-to-move/creating-token',
					'guides/developer/evm-to-move/creating-nft',
				],
			},
			{
				type: 'category',
				label: 'Your First IOTA dApp',
				link: {
					type: 'doc',
					id: 'guides/developer/first-app',
				},
				items: [
					'guides/developer/first-app/write-package',
					'guides/developer/first-app/build-test',
					'guides/developer/first-app/publish',
					'guides/developer/first-app/debug',
					'guides/developer/first-app/client-tssdk',
				],
			},
			{
				type: 'category',
				label: 'IOTA 101',
				link: {
					type: 'doc',
					id: 'guides/developer/iota-101',
				},
				items: [
					'guides/developer/iota-101/shared-owned',
					{
						type: 'category',
						label: 'Create Coins and Tokens',
						link: {
							type: 'doc',
							id: 'guides/developer/iota-101/create-coin',
						},
						items: [
							'guides/developer/iota-101/create-coin/regulated',
							'guides/developer/iota-101/create-coin/in-game-token',
							'guides/developer/iota-101/create-coin/loyalty',
						],
					},
					'guides/developer/iota-101/create-nft',
					'guides/developer/iota-101/using-events',
					'guides/developer/iota-101/access-time',
					'guides/developer/iota-101/sign-and-send-txn',
					'guides/developer/iota-101/sponsor-txn',
					{
						type: 'category',
						label: 'Working with PTBs',
						items: [
							'guides/developer/iota-101/building-ptb',
							'guides/developer/iota-101/coin-mgt',
							'guides/developer/iota-101/simulating-refs',
						],
					},
				],
			},
			{
				type: 'category',
				label: 'Cryptography',
				link: {
					type: 'doc',
					id: 'guides/developer/cryptography',
				},
				items: [
					'guides/developer/cryptography/signing',
					'guides/developer/cryptography/groth16',
					'guides/developer/cryptography/hashing',
					'guides/developer/cryptography/ecvrf',
				],
			},
			{
				type: 'category',
				label: 'Advanced Topics',
				link: {
					type: 'doc',
					id: 'guides/developer/advanced',
				},
				items: [
					/*{
						type: 'category',
						label: 'Efficient Smart Contracts',
						link: {
							type: 'doc',
							id: 'guides/developer/advanced/efficient-smart-contracts',
						},
						items: ['guides/developer/advanced/min-gas-fees'],
					},*/
					'guides/developer/advanced/graphql-migration',
					'guides/developer/advanced/move-2024-migration',
					'guides/developer/advanced/asset-tokenization',
					'guides/developer/advanced/custom-indexer',
					'guides/developer/advanced/stardust-on-move',
				],
			},
			{
				type: 'category',
				label: 'App Examples',
				link: {
					type: 'doc',
					id: 'guides/developer/app-examples',
				},
				items: [
					'guides/developer/app-examples/auction',
					'guides/developer/app-examples/blackjack',
					'guides/developer/app-examples/coin-flip',
					'guides/developer/app-examples/e2e-counter',
					{
						type: 'category',
						label: 'Oracles',
						link: {
							type: 'doc',
							id: 'guides/developer/app-examples/oracle',
						},
						items: [
							'guides/developer/app-examples/weather-oracle',
							'guides/developer/app-examples/meta-pricing-oracle',
						],
					},
					'guides/developer/app-examples/plinko',
					'guides/developer/app-examples/recaptcha',
					'guides/developer/app-examples/tic-tac-toe',
					{
						type: 'category',
						label: 'Trustless Token Swap',
						link: {
							type: 'doc',
							id: 'guides/developer/app-examples/trustless-token-swap',
						},
						items: [
							'guides/developer/app-examples/trustless-token-swap/backend',
							'guides/developer/app-examples/trustless-token-swap/indexer-api',
							'guides/developer/app-examples/trustless-token-swap/frontend',
						],
					},
					'guides/developer/app-examples/trusted-swap',
					'guides/developer/app-examples/turnip-town',
				],
			},
			{
				type: 'category',
				label: 'IOTA Chains',
				link: {
					type: 'doc',
					id: 'guides/developer/iota-chains/introduction',
				},
				items: [
					{
						type: 'doc',
						label: 'Introduction',
						id: 'guides/developer/iota-chains/introduction',
					},
					{
						type: 'category',
						label: 'Getting Started',
						items: [
							{
								type: 'doc',
								label: 'Languages & VMs',
								id: 'guides/developer/iota-chains/getting-started/languages-and-vms',
							},
							'guides/developer/iota-chains/getting-started/quick-start',
							'guides/developer/iota-chains/getting-started/compatibility',
							{
								type: 'doc',
								label: 'Networks & Chains',
								id: 'guides/developer/iota-chains/getting-started/networks-and-chains',
							},
							{
								type: 'doc',
								label: 'Tools',
								id: 'guides/developer/iota-chains/getting-started/tools',
							},
						],
					},
					{
						type: 'category',
						label: 'How To',
						items: [
							'guides/developer/iota-chains/how-tos/introduction',
							{
								type: 'doc',
								label: 'Send Funds from L1 to L2',
								id: 'guides/developer/iota-chains/how-tos/send-funds-from-L1-to-L2',
							},
							{
								type: 'doc',
								label: 'Create a Basic Contract',
								id: 'guides/developer/iota-chains/how-tos/create-a-basic-contract',
							},
							{
								type: 'doc',
								label: 'Deploy a Smart Contract',
								id: 'guides/developer/iota-chains/how-tos/deploy-a-smart-contract',
							},
							{
								type: 'doc',
								label: 'Create Custom Tokens - ERC20',
								id: 'guides/developer/iota-chains/how-tos/ERC20',
							},
							{
								type: 'doc',
								label: 'Send ERC20 Tokens Across Chains',
								id: 'guides/developer/iota-chains/how-tos/send-ERC20-across-chains',
							},
							{
								type: 'doc',
								label: 'Create NFTs - ERC721',
								id: 'guides/developer/iota-chains/how-tos/ERC721',
							},
							{
								type: 'doc',
								label: 'Send NFTs Across Chains',
								id: 'guides/developer/iota-chains/how-tos/send-NFTs-across-chains',
							},
							{
								type: 'doc',
								label: 'Test Smart Contracts',
								id: 'guides/developer/iota-chains/how-tos/test-smart-contracts',
							},
							{
								type: 'category',
								label: 'Interact with the Core Contracts',
								items: [
									{
										type: 'doc',
										label: 'Introduction',
										id: 'guides/developer/iota-chains/how-tos/core-contracts/introduction',
									},
									{
										type: 'category',
										label: 'Basics',
										items: [
											{
												type: 'doc',
												label: 'Get Native Assets Balance',
												id: 'guides/developer/iota-chains/how-tos/core-contracts/basics/get-balance',
											},
											{
												type: 'category',
												label: 'Allowance',
												items: [
													{
														type: 'doc',
														label: 'Allow',
														id: 'guides/developer/iota-chains/how-tos/core-contracts/basics/allowance/allow',
													},
													{
														type: 'doc',
														label: 'Get Allowance',
														id: 'guides/developer/iota-chains/how-tos/core-contracts/basics/allowance/get-allowance',
													},
													{
														type: 'doc',
														label: 'Take Allowance',
														id: 'guides/developer/iota-chains/how-tos/core-contracts/basics/allowance/take-allowance',
													},
												],
											},
											{
												type: 'doc',
												label: 'Send Assets to L1',
												id: 'guides/developer/iota-chains/how-tos/core-contracts/basics/send-assets-to-l1',
											},
										],
									},
									{
										type: 'category',
										label: 'Token',
										items: [
											{
												label: 'Introduction',
												type: 'doc',
												id: 'guides/developer/iota-chains/how-tos/core-contracts/token/introduction',
											},
											{
												type: 'doc',
												label: 'Create a Native Token',
												id: 'guides/developer/iota-chains/how-tos/core-contracts/token/create-native-token',
											},
											{
												type: 'doc',
												label: 'Mint Native Tokens',
												id: 'guides/developer/iota-chains/how-tos/core-contracts/token/mint-token',
											},
											{
												type: 'doc',
												label: 'Custom ERC20 Functions',
												id: 'guides/developer/iota-chains/how-tos/core-contracts/token/erc20-native-token',
											},
											{
												type: 'doc',
												label: 'Create a Foundry',
												id: 'guides/developer/iota-chains/how-tos/core-contracts/token/create-foundry',
											},
											{
												type: 'doc',
												label: 'Register Token as ERC20',
												id: 'guides/developer/iota-chains/how-tos/core-contracts/token/register-token',
											},
										],
									},
									{
										type: 'category',
										label: 'NFT',
										items: [
											{
												label: 'Introduction',
												type: 'doc',
												id: 'guides/developer/iota-chains/how-tos/core-contracts/nft/introduction',
											},
											{
												type: 'doc',
												label: 'Mint an NFT',
												id: 'guides/developer/iota-chains/how-tos/core-contracts/nft/mint-nft',
											},
											{
												type: 'doc',
												label: 'Use as ERC721',
												id: 'guides/developer/iota-chains/how-tos/core-contracts/nft/use-as-erc721',
											},
										],
									},
									{
										type: 'doc',
										label: 'Get Randomness on L2',
										id: 'guides/developer/iota-chains/how-tos/core-contracts/get-randomness-on-l2',
									},
									{
										type: 'doc',
										label: 'Call and Call View',
										id: 'guides/developer/iota-chains/how-tos/core-contracts/call-view',
									},
								],
							},
						],
					},
					{
						type: 'category',
						label: 'Explanations',
						items: [
							{
								type: 'doc',
								label: 'Anatomy of a Smart Contract',
								id: 'guides/developer/iota-chains/explanations/smart-contract-anatomy',
							},
							{
								type: 'doc',
								label: 'Sandbox Interface',
								id: 'guides/developer/iota-chains/explanations/sandbox',
							},
							{
								type: 'doc',
								label: 'Calling a Smart Contract',
								id: 'guides/developer/iota-chains/explanations/invocation',
							},
							{
								type: 'doc',
								label: 'State, Transitions and State Anchoring',
								id: 'guides/developer/iota-chains/explanations/states',
							},
							{
								type: 'doc',
								label: 'State manager',
								id: 'guides/developer/iota-chains/explanations/state_manager',
							},
							{
								type: 'doc',
								label: 'Validators and Access Nodes',
								id: 'guides/developer/iota-chains/explanations/validators',
							},
							{
								type: 'doc',
								label: 'Consensus',
								id: 'guides/developer/iota-chains/explanations/consensus',
							},
							{
								type: 'doc',
								label: 'How Accounts Work',
								id: 'guides/developer/iota-chains/explanations/how-accounts-work',
							},
							{
								type: 'doc',
								label: 'Core Contracts',
								id: 'references/iota-chains/core-contracts/overview',
							},
						],
					},
					{
						type: 'category',
						label: 'Test with Solo',
						items: [
							{
								label: 'Getting Started',
								id: 'guides/developer/iota-chains/solo/getting-started',
								type: 'doc',
							},
							{
								type: 'category',
								label: 'How To',
								items: [
									{
										type: 'doc',
										label: 'First Example',
										id: 'guides/developer/iota-chains/solo/how-tos/first-example',
									},
									{
										type: 'doc',
										label: 'The L1 Ledger',
										id: 'guides/developer/iota-chains/solo/how-tos/the-l1-ledger',
									},
									{
										type: 'doc',
										label: 'Deploy a Smart Contract',
										id: 'guides/developer/iota-chains/solo/how-tos/deploying-sc',
									},
									{
										type: 'doc',
										label: 'Invoke a Smart Contract',
										id: 'guides/developer/iota-chains/solo/how-tos/invoking-sc',
									},
									{
										type: 'doc',
										label: 'Call a View',
										id: 'guides/developer/iota-chains/solo/how-tos/view-sc',
									},
									{
										type: 'doc',
										label: 'Error Handling',
										id: 'guides/developer/iota-chains/solo/how-tos/error-handling',
									},
									{
										type: 'doc',
										label: 'Accounts',
										id: 'guides/developer/iota-chains/solo/how-tos/the-l2-ledger',
									},
									{
										type: 'doc',
										label: 'Test Smart Contracts',
										id: 'guides/developer/iota-chains/solo/how-tos/test',
									},
									{
										type: 'doc',
										label: 'Example Tests',
										id: 'guides/developer/iota-chains/solo/how-tos/examples',
									},
									{
										type: 'doc',
										label: 'Colored Tokens and Time Locks',
										id: 'guides/developer/iota-chains/solo/how-tos/timelock',
									},
								],
							},
						],
					},
					{
						type: 'category',
						label: 'Wasm - Schema Tool',
						items: [
							{
								type: 'doc',
								label: 'The Schema Tool',
								id: 'guides/developer/iota-chains/schema/introduction',
							},
							{
								type: 'doc',
								label: 'Data Access Proxies',
								id: 'guides/developer/iota-chains/schema/proxies',
							},
							{
								type: 'category',
								label: 'How To',
								items: [
									{
										type: 'doc',
										label: 'Create a Schema',
										id: 'guides/developer/iota-chains/schema/how-tos/usage',
									},
									{
										type: 'doc',
										label: 'Define the State',
										id: 'guides/developer/iota-chains/schema/how-tos/state',
									},
									{
										type: 'doc',
										label: 'Use Structured Data Types',
										id: 'guides/developer/iota-chains/schema/how-tos/structs',
									},
									{
										type: 'doc',
										label: 'Generate Type Definitions',
										id: 'guides/developer/iota-chains/schema/how-tos/typedefs',
									},
									{
										type: 'doc',
										label: 'Trigger Events',
										id: 'guides/developer/iota-chains/schema/how-tos/events',
									},
									{
										type: 'doc',
										label: 'Define Functions',
										id: 'guides/developer/iota-chains/schema/how-tos/funcs',
									},
									{
										type: 'doc',
										label: 'Limit Access',
										id: 'guides/developer/iota-chains/schema/how-tos/access',
									},
									{
										type: 'doc',
										label: 'Define Function Parameters',
										id: 'guides/developer/iota-chains/schema/how-tos/params',
									},
									{
										type: 'doc',
										label: 'Define Function Results',
										id: 'guides/developer/iota-chains/schema/how-tos/results',
									},
									{
										type: 'doc',
										label: 'Use Thunk Functions',
										id: 'guides/developer/iota-chains/schema/how-tos/thunks',
									},
									{
										type: 'doc',
										label: 'Use View-Only Functions',
										id: 'guides/developer/iota-chains/schema/how-tos/views',
									},
									{
										type: 'doc',
										label: 'Initialize a Smart Contract',
										id: 'guides/developer/iota-chains/schema/how-tos/init',
									},
									{
										type: 'doc',
										label: 'Transfer Tokens',
										id: 'guides/developer/iota-chains/schema/how-tos/transfers',
									},
									{
										type: 'doc',
										label: 'Add Function Descriptors',
										id: 'guides/developer/iota-chains/schema/how-tos/funcdesc',
									},
									{
										type: 'doc',
										label: 'Call Functions',
										id: 'guides/developer/iota-chains/schema/how-tos/call',
									},
									{
										type: 'doc',
										label: 'Post Asynchronous Requests',
										id: 'guides/developer/iota-chains/schema/how-tos/post',
									},
								],
							},
						],
					},
				],
			},
			'guides/developer/starter-templates',
			'guides/developer/zklogin-onboarding',
			'guides/developer/dev-cheat-sheet',
		],
	},
	{
		type: 'category',
		label: 'Operator Guides',
		link: {
			type: 'doc',
			id: 'guides/operator',
		},
		items: [
			'guides/operator/iota-full-node',
			'guides/operator/validator-config',
			'guides/operator/data-management',
			'guides/operator/snapshots',
			'guides/operator/archives',
			'guides/operator/genesis',
			'guides/operator/validator-committee',
			'guides/operator/validator-tasks',
			{
				type: 'category',
				label: 'IOTA Chains Node',
				link: {
					type: 'doc',
					id: 'guides/operator/iota-chains-node/how-tos/running-a-node',
				},
				items: [
					{
						type: 'category',
						label: 'How To',
						collapsed: false,
						items: [
							{
								type: 'doc',
								id: 'guides/operator/iota-chains-node/how-tos/running-a-node',
								label: 'Run a Node',
							},
							{
								type: 'doc',
								id: 'guides/operator/iota-chains-node/how-tos/running-an-access-node',
								label: 'Run an Access Node',
							},
							{
								id: 'guides/operator/iota-chains-node/how-tos/wasp-cli',
								label: 'Configure wasp-cli',
								type: 'doc',
							},
							{
								id: 'guides/operator/iota-chains-node/how-tos/setting-up-a-chain',
								label: 'Set Up a Chain',
								type: 'doc',
							},
							{
								id: 'guides/operator/iota-chains-node/how-tos/chain-management',
								label: 'Manage a Chain',
								type: 'doc',
							},
						],
					},
					{
						type: 'category',
						label: 'Reference',
						items: [
							{
								type: 'doc',
								id: 'guides/operator/iota-chains-node/reference/configuration',
							},
							{
								type: 'doc',
								id: 'guides/operator/iota-chains-node/reference/metrics',
							},
						],
					},
				],
			},
			'guides/operator/node-tools',
			'guides/operator/exchange-integration',
		],
	},
	{
		type: 'category',
		label: 'Migrating IOTA/Shimmer Stardust',
		link: {
			type: 'doc',
			id: 'guides/stardust-migration',
		},
		items: [
			'guides/stardust/move-models',
			'guides/stardust/addresses',
			'guides/stardust/units',
			'guides/stardust/migration-process',
			'guides/stardust/claiming',
			'guides/stardust/vested',
			'guides/stardust/testing',
			'guides/stardust/if-tools',
			'guides/stardust/faq',
		],
	},
];
module.exports = guides;
