// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import TransportWebHID from '@ledgerhq/hw-transport-webhid';
import TransportWebUSB from '@ledgerhq/hw-transport-webusb';
import IOTALedgerClient from '@iota/ledgerjs-hw-app-iota';
import { createContext, useCallback, useContext, useEffect, useMemo, useState } from 'react';

import {
    convertErrorToLedgerConnectionFailedError,
    LedgerDeviceNotFoundError,
    LedgerNoTransportMechanismError,
} from './ledgerErrors';

type IOTALedgerClientProviderProps = {
    children: React.ReactNode;
};

type IOTALedgerClientContextValue = {
    iotaLedgerClient: IOTALedgerClient | undefined;
    connectToLedger: (requestPermissionsFirst?: boolean) => Promise<IOTALedgerClient>;
};

const IOTALedgerClientContext = createContext<IOTALedgerClientContextValue | undefined>(undefined);

export function IOTALedgerClientProvider({ children }: IOTALedgerClientProviderProps) {
    const [iotaLedgerClient, setIOTALedgerClient] = useState<IOTALedgerClient>();
    const resetIOTALedgerClient = useCallback(async () => {
        await iotaLedgerClient?.transport.close();
        setIOTALedgerClient(undefined);
    }, [iotaLedgerClient]);

    useEffect(() => {
        // NOTE: The disconnect event is fired when someone physically disconnects
        // their Ledger device in addition to when user's exit out of an application
        iotaLedgerClient?.transport.on('disconnect', resetIOTALedgerClient);
        return () => {
            iotaLedgerClient?.transport.off('disconnect', resetIOTALedgerClient);
        };
    }, [resetIOTALedgerClient, iotaLedgerClient?.transport]);

    const connectToLedger = useCallback(
        async (requestPermissionsFirst = false) => {
            // If we've already connected to a Ledger device, we need
            // to close the connection before we try to re-connect
            await resetIOTALedgerClient();

            const ledgerTransport = requestPermissionsFirst
                ? await requestLedgerConnection()
                : await openLedgerConnection();
            const ledgerClient = new IOTALedgerClient(ledgerTransport);
            setIOTALedgerClient(ledgerClient);
            return ledgerClient;
        },
        [resetIOTALedgerClient],
    );
    const contextValue: IOTALedgerClientContextValue = useMemo(() => {
        return {
            iotaLedgerClient,
            connectToLedger,
        };
    }, [connectToLedger, iotaLedgerClient]);

    return (
        <IOTALedgerClientContext.Provider value={contextValue}>
            {children}
        </IOTALedgerClientContext.Provider>
    );
}

export function useIOTALedgerClient() {
    const iotaLedgerClientContext = useContext(IOTALedgerClientContext);
    if (!iotaLedgerClientContext) {
        throw new Error('useIOTALedgerClient must be used within IOTALedgerClientContext');
    }
    return iotaLedgerClientContext;
}

async function requestLedgerConnection() {
    const ledgerTransportClass = await getLedgerTransportClass();
    try {
        return await ledgerTransportClass.request();
    } catch (error) {
        throw convertErrorToLedgerConnectionFailedError(error);
    }
}

async function openLedgerConnection() {
    const ledgerTransportClass = await getLedgerTransportClass();
    let ledgerTransport: TransportWebHID | TransportWebUSB | null | undefined;

    try {
        ledgerTransport = await ledgerTransportClass.openConnected();
    } catch (error) {
        throw convertErrorToLedgerConnectionFailedError(error);
    }
    if (!ledgerTransport) {
        throw new LedgerDeviceNotFoundError(
            "The user doesn't have a Ledger device connected to their machine",
        );
    }
    return ledgerTransport;
}

async function getLedgerTransportClass() {
    if (await TransportWebHID.isSupported()) {
        return TransportWebHID;
    } else if (await TransportWebUSB.isSupported()) {
        return TransportWebUSB;
    }
    throw new LedgerNoTransportMechanismError(
        "There are no supported transport mechanisms to connect to the user's Ledger device",
    );
}
