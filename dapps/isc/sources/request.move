// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

module isc::request {
    use std::string::String;
    use sui::{
        borrow::{Self, Referent},
        event::{Self},
    };
    use isc::{
        assets_bag::{AssetsBag},
    }; 

    // === Main structs ===

    /// Contains the request data to be used off-chain.
    public struct RequestData has copy, drop, store {
        /// ID of the request object
        request_id: ID,
        /// Contract name
        contract: String,
        /// Function name
        function: String,
        /// Function arguments
        args: vector<vector<u8>>,
        /// Request sender
        sender: address,
    }

    /// Represents a request object
    public struct Request has key {
        id: UID,
        /// Bag of assets associated to the request
        assets_bag: Referent<AssetsBag>,
        /// The request data, to be used off-chain 
        data: RequestData,
    }

    // === Events ===
    
    /// Emitted when a request is sent to an address.
    public struct RequestEvent has copy, drop {
        data: RequestData
    }
    
    // === Request packing and unpacking ===

    /// Creates a request to call a specific SC function.
    public fun create_request(
        contract: String, 
        function: String, 
        args: vector<vector<u8>>, 
        sender: address,
        assets_bag: AssetsBag,
        ctx: &mut TxContext
    ): Request {
        let id = object::new(ctx);
        let data = RequestData {
            request_id: id.uid_to_inner(),
            contract,
            function,
            args,
            sender,
        };
        Request{
            id,
            assets_bag: borrow::new(assets_bag, ctx),
            data,
        }
    }

    /// Destroys a Request object and returns its balance and assets bag.
    public fun destroy(self: Request): AssetsBag {
        let Request {
            id,
            assets_bag,
            data: _,
        } = self;
        id.delete();
        
        assets_bag.destroy()
    }

    // === Send and receive the Request ===

    /// Send a Request object to a receiver and emits the RequestEvent.
    public fun send(self: Request, receiver: address) {
        event::emit(RequestEvent { data: self.data });
        transfer::transfer(self, receiver)
    }

    /// Utility function to receive a `Request` object in other ISC modules.
    /// Other modules in the ISC package can call this function to receive an `Request` object.
    public(package) fun receive(parent: &mut UID, self: transfer::Receiving<Request>) : Request {
        transfer::receive(parent, self)
    }

}