// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
import { DialogBody, DialogContent, DialogPosition } from '@iota/apps-ui-kit';

export function Layout({ children }: { children: React.ReactNode }) {
    return (
        <DialogContent containerId="overlay-portal-container" position={DialogPosition.Right}>
            <div className="flex h-full flex-col">{children}</div>
        </DialogContent>
    );
}

export function LayoutBody({ children }: { children: React.ReactNode }) {
    return (
        <div className="flex w-full flex-1 overflow-y-hidden [&_>div]:w-full [&_>div]:overflow-y-auto">
            <DialogBody>{children}</DialogBody>
        </div>
    );
}

export function LayoutFooter({ children }: { children: React.ReactNode }) {
    return <div className="p-md--rs">{children}</div>;
}
