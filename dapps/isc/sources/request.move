
/// Module: isc
module isc::request {
    use isc::{
        assets::{Self, Assets},
    };
    use stardust::{
        nft::{Nft},
    };
    use sui::{
        coin::{Coin},
        sui::SUI,
        };
    use std::ascii::String;

    /// privileged function was called without authorization
    const EMultipleTimeLocks: u64 = 1;

    public struct Request has key, store {
        id: UID,
        assets: Option<Assets>,
         //TODO allowance
        contract: String,
        function: String,
        args: vector<vector<u8>>,
        time_lock: u64,
        sender: address,
    }

    /// creates a request to call a specific SC function
    /// requests can also be used to send assets to the sender account on the
    /// chain by using `create_request("account", "deposit", vector[])`
    public fun create_request(contract: String, function: String, args: vector<vector<u8>>, ctx: &mut TxContext): Request {
        Request{
            id: object::new(ctx),
            assets: option::some(assets::new(ctx)),
            contract: contract,
            function: function,
            args: args,
            time_lock: 0,
            sender: ctx.sender(),
        }
    }

    /// add base tokens to the request by joining them to the request's balance
    public fun add_base_tokens(req: &mut Request, base_tokens: Coin<SUI>, ctx: &mut TxContext) {
        assert!(option::is_some(&req.assets), 0);
        let assets = option::borrow_mut(&mut req.assets);
        assets.add_base_tokens(base_tokens, ctx);
    }

    /// add native tokens to the request by joining them to the request's balances
    public fun add_native_tokens<T>(req: &mut Request, native_tokens: Coin<T>, ctx: &mut TxContext) {
        assert!(option::is_some(&req.assets), 0);
        let assets = option::borrow_mut(&mut req.assets);
        assets.add_native_tokens(native_tokens, ctx);
    }

    /// add an `Nft` to the request
    public fun add_nft(req: &mut Request, nft: Nft, ctx: &mut TxContext) {
        assert!(option::is_some(&req.assets), 0);
        let assets = option::borrow_mut(&mut req.assets);
        assets.add_nft(nft, ctx);
    }

    /// get the nft vector
    public fun get_assets(req: &mut Request, _ctx: &mut TxContext): Assets {
        option::extract(&mut req.assets)
    }

    public fun set_time_lock(req: &mut Request, time_lock: u64, _ctx: &mut TxContext) {
        assert!(req.time_lock == 0, EMultipleTimeLocks);
        //TODO check time lock value to be in the future?
        req.time_lock = time_lock;
    }
}

