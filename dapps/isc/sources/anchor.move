// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

module isc::anchor {
    use sui::borrow::{Self, Referent, Borrow};   
    use isc::{
        request::{Self, Request},
        assets_bag::{Self, AssetsBag},
    }; 

    // === Main structs ===

    /// An object which allows managing assets within the "ISC" ecosystem.
    /// By default it is owned by a single address.
    public struct Anchor has key, store {
        id: UID,
        /// Anchor assets.
        assets: Referent<AssetsBag>,
    }

    // === Anchor packing and unpacking ===

    /// Starts a new chain by creating a new `Anchor` for it
    public fun start_new_chain(ctx: &mut TxContext): Anchor {
        Anchor{
            id: object::new(ctx),
            assets: borrow::new(assets_bag::new(ctx), ctx),
         }
    }

    /// Destroys an Anchor object and returns its assets bag.   
    public fun destroy(self: Anchor): AssetsBag {
        let Anchor { id, assets } = self;
        id.delete();
        
        assets.destroy()
    }
    
    // === Borrow assets from the Anchor ===

    /// Simulates a borrow mutable for the AssetsBag implementing the HotPotato pattern.   
    public fun borrow_assets(self: &mut Anchor): (AssetsBag, Borrow) {
        borrow::borrow(&mut self.assets)
    }

    /// Finishes the simulation of a borrow mutable putting back the HotPotato. 
    public fun return_assets_from_borrow(
        self: &mut Anchor,
        assets: AssetsBag,
        b: Borrow
    ) {
        borrow::put_back(&mut self.assets, assets, b)
    }

    // === Receive a Request ===

    /// The Anchor receives a request and destroys it, implementing the HotPotato pattern.
    public fun receive_request(anchor: &mut Anchor, request: transfer::Receiving<Request>): AssetsBag {
        let req = request::receive(&mut anchor.id, request);
        req.destroy()
    }
}