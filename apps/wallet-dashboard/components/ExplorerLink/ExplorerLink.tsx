// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { ExplorerLinkConfig, getExplorerLink } from '@iota/core';
import { useIotaClientContext } from '@iota/dapp-kit';
import { PropsWithChildren } from 'react';

type ExplorerLinkProps = {
    address: string;
    linkProps: ExplorerLinkConfig;
};

export function ExplorerLink({
    children,
    address,
    linkProps,
}: PropsWithChildren<ExplorerLinkProps>) {
    const { network } = useIotaClientContext();
    const explorerHref = getExplorerLink(linkProps, address, network);

    if (!explorerHref) {
        return null;
    }
    return (
        <a href={explorerHref} target="_blank" rel="noreferrer">
            {children}
        </a>
    );
}
