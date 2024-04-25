// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module stardust::nft {

    use stardust::irc27::{Self, Irc27Metadata};

    /// The Stardust NFT representation.
    public struct Nft has key, store {
        /// The Nft's ID is nested from Stardust.
        id: UID,

        /// The sender feature holds the last sender address assigned before the migration and
        /// is not supported by the protocol after it.
        legacy_sender: Option<address>,
        /// The metadata feature.
        metadata: Option<vector<u8>>,
        /// The tag feature.
        tag: Option<vector<u8>>,

        /// The immutable issuer feature.
        immutable_issuer: Option<address>,
        /// The immutable metadata feature.
        immutable_metadata: Irc27Metadata,
    }

    /// Permanently destroy an `Nft` object.
    public fun destroy(nft: Nft) {
        let Nft {
            id: id,
            legacy_sender: _,
            metadata: _,
            tag: _,
            immutable_issuer: _,
            immutable_metadata: immutable_metadata,
        } = nft;

        irc27::destroy(immutable_metadata);

        object::delete(id);
    }

    /// Get the NFT's `legacy_sender`.
    public fun legacy_sender(nft: &Nft): &Option<address> {
        &nft.legacy_sender
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

    #[test_only]
    public fun create_for_testing(
        legacy_sender: Option<address>,
        metadata: Option<vector<u8>>,
        tag: Option<vector<u8>>,
        immutable_issuer: Option<address>,
        immutable_metadata: Irc27Metadata,
        ctx: &mut TxContext,
    ): Nft {
        Nft {
            id: object::new(ctx),
            legacy_sender,
            metadata,
            tag,
            immutable_issuer,
            immutable_metadata,
        }
    }
}
