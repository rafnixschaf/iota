// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module stardust::nft {

    use stardust::irc27::{Self, Irc27Metadata};

    /// The Stardust NFT representation.
    public struct Nft has key, store {
        /// The Nft's ID is nested from Stardust for the migrated NFTs.
        id: UID,

        /// The sender features.
        sender: Option<address>,
        /// The metadata features.
        metadata: Option<vector<u8>>,
        /// The tag features.
        tag: Option<vector<u8>>,

        /// The immutable issuer feature.
        immutable_issuer: Option<address>,
        /// The immutable metadata feature.
        immutable_metadata: Irc27Metadata,
    }

    /// Create a new `Nft` object.
    public fun create(
        sender: Option<address>,
        metadata: Option<vector<u8>>,
        tag: Option<vector<u8>>,
        immutable_issuer: Option<address>,
        immutable_metadata: Irc27Metadata,
        ctx: &mut TxContext,
    ): Nft {
        Nft {
            id: object::new(ctx),
            sender,
            metadata,
            tag,
            immutable_issuer,
            immutable_metadata,
        }
    }

    /// Permanently destroy an `Nft` object.
    public fun destroy(output: Nft) {
        let Nft {
            id: id,
            sender: _,
            metadata: _,
            tag: _,
            immutable_issuer: _,
            immutable_metadata: immutable_metadata,
        } = output;

        irc27::destroy(immutable_metadata);

        object::delete(id);
    }

    /// Get the NFT's `sender`.
    public fun sender(nft: &Nft): &Option<address> {
        &nft.sender
    }

    /// Get the NFT's `metadata`.
    public fun metadata(nft: &Nft): &Option<vector<u8>> {
        &nft.metadata
    }

    /// Get the NFT's `tag`.
    public fun tag(nft: &Nft): &Option<vector<u8>> {
        &nft.tag
    }

    /// Get the NFT's `immutable_sender`.
    public fun immutable_issuer(nft: &Nft): &Option<address> {
        &nft.immutable_issuer
    }

    /// Get the NFT's `immutable_metadata`.
    public fun immutable_metadata(nft: &Nft): &Irc27Metadata {
        &nft.immutable_metadata
    }
}
