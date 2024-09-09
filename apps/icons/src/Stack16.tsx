// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { SVGProps } from 'react';

const SvgStack16 = (props: SVGProps<SVGSVGElement>) => (
	<svg
		xmlns="http://www.w3.org/2000/svg"
		width="1em"
		height="1em"
		fill="none"
		viewBox="0 0 16 16"
		{...props}
	>
		<g clipPath="url(#stack_16_svg__a)">
			<path
				fill="currentColor"
				d="M3.717 1.023h8.723C12.384.367 11.967.005 11.27.005H4.886c-.698 0-1.115.362-1.17 1.018Zm-1.36 2.113H13.8c-.09-.698-.465-1.115-1.265-1.115H3.621c-.8 0-1.176.417-1.265 1.115Zm.8 12.974h9.836c1.559 0 2.42-.847 2.42-2.4V6.67c0-1.552-.861-2.4-2.42-2.4H3.156c-1.558 0-2.413.848-2.413 2.4v7.04c0 1.553.855 2.4 2.413 2.4Zm.19-1.763c-.546 0-.84-.267-.84-.848V6.882c0-.581.294-.848.84-.848h9.455c.547 0 .847.267.847.848v6.617c0 .581-.3.848-.847.848H3.348Zm5.12-1.518c.144 0 .247-.102.274-.24.322-1.934.533-2.276 2.523-2.529a.281.281 0 0 0 .253-.28c0-.136-.103-.246-.253-.266-1.949-.26-2.215-.595-2.523-2.523-.027-.15-.13-.253-.273-.253-.137 0-.246.096-.274.246-.307 1.935-.547 2.263-2.522 2.53-.144.02-.246.13-.246.266 0 .144.102.253.246.28 1.948.26 2.215.575 2.522 2.53.028.137.13.24.274.24Zm-2.706 1.08a.204.204 0 0 0 .198-.17c.157-.958.123-.978 1.162-1.156a.204.204 0 0 0 .164-.198.199.199 0 0 0-.164-.198c-1.032-.158-1.005-.185-1.162-1.142-.02-.11-.096-.185-.198-.185-.103 0-.178.069-.199.185-.164.936-.123.957-1.162 1.142a.204.204 0 0 0-.17.198c0 .11.068.178.19.198 1.02.157.978.191 1.143 1.142.02.109.088.184.198.184Z"
			/>
		</g>
		<defs>
			<clipPath id="stack_16_svg__a">
				<path fill="#fff" d="M0 0h16v16H0z" />
			</clipPath>
		</defs>
	</svg>
);
export default SvgStack16;
