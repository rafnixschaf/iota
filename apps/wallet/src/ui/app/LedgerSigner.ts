// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type IOTALedgerClient from '@iota/ledgerjs-hw-app-iota';
import { type IOTAClient } from '@iota/iota.js/client';
import {
    toSerializedSignature,
    type SerializedSignature,
    type SignatureScheme,
} from '@iota/iota.js/cryptography';
import { Ed25519PublicKey } from '@iota/iota.js/keypairs/ed25519';

import { WalletSigner } from './WalletSigner';

export class LedgerSigner extends WalletSigner {
    #iotaLedgerClient: IOTALedgerClient | null;
    readonly #connectToLedger: () => Promise<IOTALedgerClient>;
    readonly #derivationPath: string;
    readonly #signatureScheme: SignatureScheme = 'ED25519';

    constructor(
        connectToLedger: () => Promise<IOTALedgerClient>,
        derivationPath: string,
        client: IOTAClient,
    ) {
        super(client);
        this.#connectToLedger = connectToLedger;
        this.#iotaLedgerClient = null;
        this.#derivationPath = derivationPath;
    }

    async #initializeIOTALedgerClient() {
        if (!this.#iotaLedgerClient) {
            // We want to make sure that there's only one connection established per Ledger signer
            // instance since some methods make multiple calls like getAddress and signData
            this.#iotaLedgerClient = await this.#connectToLedger();
        }
        return this.#iotaLedgerClient;
    }

    async getAddress(): Promise<string> {
        const ledgerClient = await this.#initializeIOTALedgerClient();
        const publicKeyResult = await ledgerClient.getPublicKey(this.#derivationPath);
        const publicKey = new Ed25519PublicKey(publicKeyResult.publicKey);
        return publicKey.toIOTAAddress();
    }

    async getPublicKey(): Promise<Ed25519PublicKey> {
        const ledgerClient = await this.#initializeIOTALedgerClient();
        const { publicKey } = await ledgerClient.getPublicKey(this.#derivationPath);
        return new Ed25519PublicKey(publicKey);
    }

    async signData(data: Uint8Array): Promise<SerializedSignature> {
        const ledgerClient = await this.#initializeIOTALedgerClient();
        const { signature } = await ledgerClient.signTransaction(this.#derivationPath, data);
        const publicKey = await this.getPublicKey();
        return toSerializedSignature({
            signature,
            signatureScheme: this.#signatureScheme,
            publicKey,
        });
    }

    connect(client: IOTAClient) {
        return new LedgerSigner(this.#connectToLedger, this.#derivationPath, client);
    }
}
