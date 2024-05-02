/// Module: isc
module isc::assets {
    use isc::{
        ledger::{Self, Ledger},
    };
    use stardust::{
        nft::{Nft},
    };
    use std::ascii::String;
    use std::type_name;
    use sui::{
        bag::{Self, Bag},
        balance::{Self, Balance},
        coin::{Coin},
        sui::SUI,
    };

    const ENftNotFound: u64 = 1;

    public struct Assets has key, store {
        id: UID,
        ledger: Ledger,
        base_tokens: Balance<SUI>,
        native_tokens: Bag,
        nfts: vector<Nft>,
    }

    public fun new(ctx: &mut TxContext): Assets {
        Assets {
            id: object::new(ctx),
            ledger: ledger::new(),
            base_tokens: balance::zero<SUI>(),
            native_tokens: bag::new(ctx),
            nfts: vector[],
        }
    }

    /// add base tokens to the `Assets` by joining them to the `Assets`' balance
    public fun add_base_tokens(assets: &mut Assets, base_tokens: Coin<SUI>) {
        assets.ledger.add_base_tokens(base_tokens.value());
        assets.base_tokens.join(base_tokens.into_balance());
    }

    /// add native tokens to the `Assets` by joining them to the `Assets`' balance
    public fun add_native_tokens<T>(assets: &mut Assets, native_tokens: Coin<T>) {
        let token_type_name = type_name::get<T>().into_string();
        assets.ledger.add_native_tokens(&token_type_name, native_tokens.value());
        let tokens = native_tokens.into_balance();
        if (!assets.native_tokens.contains(token_type_name)) {
            assets.native_tokens.add(token_type_name, tokens);
        } else {
            assets.native_tokens.borrow_mut<String, Balance<T>>(token_type_name).join(tokens);
        }
     }

    /// add an `Nft` to the `Assets`
    public fun add_nft(assets: &mut Assets, nft: Nft) {
        assets.ledger.add_nft(*object::borrow_id<Nft>(&nft));
        assets.nfts.push_back(nft);
    }

    /// joins all the provided `Assets` to the current `Assets`
    public fun join(assets: &mut Assets, from: Assets): Ledger {
        let Assets { id, ledger, base_tokens, mut native_tokens, mut nfts, } = from;
        assets.ledger.add_base_tokens(base_tokens.value());
        assets.base_tokens.join(base_tokens);
    
        let native_token_types = ledger.get_native_token_types();
        let mut i = 0;
        while (i < native_token_types.length()) {
            let token_type_name = *native_token_types.borrow(i);
            assets.ledger.add_native_tokens(&token_type_name, ledger.get_native_token_amount(&token_type_name));
            let tokens = native_tokens.remove(token_type_name);
            if (!assets.native_tokens.contains(token_type_name)) {
                assets.native_tokens.add(token_type_name, tokens);
            } else {
                // T can be any token here, but since it's a phantom T
                // anyway, we fool the type system by using SUI instead
                assets.native_tokens.borrow_mut<String, Balance<SUI>>(token_type_name).join(tokens);
            };
            i = i + 1
        };
        native_tokens.destroy_empty();

        while (!nfts.is_empty()) {
            assets.add_nft(nfts.pop_back());
        };
        nfts.destroy_empty();

        object::delete(id);
        ledger
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

    public fun destroy_empty(assets: Assets) {
        let Assets {
            id,
            ledger,
            base_tokens,
            mut native_tokens,
            nfts,
        } = assets;
        base_tokens.destroy_zero<SUI>();
        let native_token_types = ledger.get_native_token_types();
        let mut i = 0;
        while (i < native_token_types.length()) {
            let native_token_type = native_token_types.borrow(i);
            let tokens = native_tokens.remove<String, Balance<SUI>>(*native_token_type);
            tokens.destroy_zero();
            i = i + 1
        };
        native_tokens.destroy_empty();
        nfts.destroy_empty();
        object::delete(id);
    }
}
