// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { IOTACustomRpc, IOTADevnet, IOTALocal, IOTAMainnet, IOTATestnet } from '@iota/icons';
import { Network } from '@iota/iota.js/client';

type LogoProps = {
    network?: Network;
};

const networkLogos = {
    [Network.Mainnet]: IOTAMainnet,
    [Network.Devnet]: IOTADevnet,
    [Network.Testnet]: IOTATestnet,
    [Network.Local]: IOTALocal,
    [Network.Custom]: IOTACustomRpc,
};

const Logo = ({ network }: LogoProps) => {
    let LogoComponent = networkLogos[Network.Custom];

    if (network && networkLogos[network]) {
        LogoComponent = networkLogos[network];
    }

    return <LogoComponent className="h-7 w-walletLogo text-gray-90" />;
};

export default Logo;
