// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { useRouter } from 'next/router';

const config = {
	logo: <span>IOTA TypeScript Docs</span>,
	project: {
		link: 'https://github.com/iotaledger/kinesis/tree/main/sdk/',
	},
	chat: {
		link: 'https://discord.com/invite/IOTA',
	},
	docsRepositoryBase: 'https://github.com/iotaledger/kinesis/tree/main/sdk/docs',
	footer: {
		text: `Copyright © ${new Date().getFullYear()}, Mysten Labs, Inc.`,
	},
	head: (
		<>
			<meta name="google-site-verification" content="T-2HWJAKh8s63o9KFxCFXg5MON_NGLJG76KJzr_Hp0A" />
			<meta httpEquiv="Content-Language" content="en" />
		</>
	),
	useNextSeoProps() {
		const { asPath } = useRouter();

		return {
			titleTemplate: asPath !== '/' ? '%s | IOTA TypeScript Docs' : 'IOTA TypeScript Docs',
			description:
				'IOTA TypeScript Documentation. Discover the power of IOTA through examples, guides, and concepts.',
			openGraph: {
				title: 'IOTA TypeScript Docs',
				description:
					'IOTA TypeScript Documentation. Discover the power of IOTA through examples, guides, and concepts.',
				site_name: 'IOTA TypeScript Docs',
			},
			additionalMetaTags: [{ content: 'IOTA TypeScript Docs', name: 'apple-mobile-web-app-title' }],
			twitter: {
				card: 'summary_large_image',
				site: '@Mysten_Labs',
			},
		};
	},
};

export default config;
