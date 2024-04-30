/// Module: isc
module isc::assets {
    use stardust::{
        nft::{Nft},
    };
    use sui::{
        bag::{Self, Bag},
        balance::{Self, Balance},
        coin::{Coin},
        sui::SUI,
        };
    use std::ascii::String;
    use std::type_name;

    const ENftNotFound: u64 = 1;

    public struct Assets has key, store {
        id: UID,
        base_tokens: Balance<SUI>,
        native_tokens: Bag,
        native_token_keys: vector<String>,
        nfts: vector<Nft>,
    }

    public fun new(ctx: &mut TxContext): Assets {
        Assets {
            id: object::new(ctx),
            base_tokens: balance::zero<SUI>(),
            native_tokens: bag::new(ctx),
            native_token_keys: vector[],
            nfts: vector[],
        }
    }

    /// add base tokens to the `Assets` by joining them to the `Assets`' balance
    public fun add_base_tokens(assets: &mut Assets, base_tokens: Coin<SUI>, _ctx: &mut TxContext) {
        assets.base_tokens.join(base_tokens.into_balance());
    }

    /// add native tokens to the `Assets` by joining them to the `Assets`' balance
    public fun add_native_tokens<T>(assets: &mut Assets, native_tokens: Coin<T>, _ctx: &mut TxContext) {
        let token_type_name = type_name::get<T>().into_string();
        let tokens = native_tokens.into_balance();
        if (!assets.native_tokens.contains(token_type_name)) {
            assets.native_tokens.add(token_type_name, tokens);
            assets.native_token_keys.push_back(token_type_name);
        } else {
            assets.native_tokens.borrow_mut<String, Balance<T>>(token_type_name).join(tokens);
        }
     }

    /// add an `Nft` to the `Assets`
    public fun add_nft(assets: &mut Assets, nft: Nft, _ctx: &mut TxContext) {
         assets.nfts.push_back(nft);
    }

    /// joins all the provided `Assets` to the current `Assets`
    public fun join(assets: &mut Assets, from: Assets, _ctx: &mut TxContext) {
        let Assets { id, base_tokens, mut native_tokens, mut native_token_keys, mut nfts, } = from;
        assets.base_tokens.join(base_tokens);
        while (!native_token_keys.is_empty()) {
            let token_type_name = native_token_keys.pop_back();
            let tokens = native_tokens.remove(token_type_name);
            if (!assets.native_tokens.contains(token_type_name)) {
                assets.native_tokens.add(token_type_name, tokens);
                assets.native_token_keys.push_back(token_type_name);
            } else {
                // T can be any token here, but since it's a phantom T
                // anyway, we fool the type system by using SUI instead
                assets.native_tokens.borrow_mut<String, Balance<SUI>>(token_type_name).join(tokens);
            }
        };
        native_tokens.destroy_empty();
        native_token_keys.destroy_empty();
        while (!nfts.is_empty()) {
            assets.nfts.push_back(nfts.pop_back());
        };
        nfts.destroy_empty();
        object::delete(id);
    }

    /// takes a specified amount of base tokens from the `Assets`
    public fun take_base_tokens(assets: &mut Assets, amount: u64): Balance<SUI> {
        assets.base_tokens.split(amount)
    }

    /// takes a specified amount of native tokens from the `Assets`
    public fun take_native_tokens<T>(assets: &mut Assets, amount: u64): Balance<T> {
        let token_type_name = type_name::get<T>().into_string();
        assert!(assets.native_tokens.contains(token_type_name), 0);
        assets.native_tokens.borrow_mut<String, Balance<T>>(token_type_name).split(amount)
    }

    /// takes a specified nft from the `Assets`
    public fun take_nft(assets: &mut Assets, nft_id: ID): Nft {
        let mut found = false;
        let mut i = assets.nfts.length();
        while (i > 0 && !found) {
            i = i - 1;
            let nft = assets.nfts.borrow(i);
            let id = object::borrow_id<Nft>(nft);
            found = (id == nft_id)
        };
        assert!(found, ENftNotFound);
        assets.nfts.swap_remove(i)
    }
}
