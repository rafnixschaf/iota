/// Module: isc
module isc::ledger {
    use std::ascii::String;

    const EDuplicateNft: u64 = 1;

    /// A `Ledger` keeps track of administration of the objects in an `Assets` object
    /// It can be used to track mutations on the `Assets` object and to describe the
    /// asset allowance for an ISC SC request.
    /// Note that a `Ledger` can be passed around and copied even after the assets it
    /// describes have been moved out of an the `Assets` object.
    public struct Ledger has copy, drop, store {
        base_tokens: u64,
        native_token_amounts: vector<u64>,
        native_token_types: vector<String>,
        nfts: vector<ID>,
    }

    public fun new(): Ledger {
        Ledger {
            base_tokens: 0,
            native_token_amounts: vector[],
            native_token_types: vector[],
            nfts: vector[],
        }
    }

    public fun add_base_tokens(ledger: &mut Ledger, amount: u64) {
        ledger.base_tokens = ledger.base_tokens + amount;
    }

    public fun add_native_tokens(ledger: &mut Ledger, native_token_type: &String, amount: u64) {
        //TODO do we want to prevent theuse of SUI base token type here?
        let (found, i) = ledger.native_token_types.index_of(native_token_type);
        if (found) {
            let tokens = ledger.native_token_amounts.borrow_mut(i);
            *tokens = *tokens + amount
        } else {
            ledger.native_token_types.push_back(*native_token_type);
            ledger.native_token_amounts.push_back(amount)
        }
    }

    public fun add_nft(ledger: &mut Ledger, nft: ID) {
        assert!(!ledger.nfts.contains(&nft), EDuplicateNft);
        ledger.nfts.push_back(nft)
    }

    public fun get_base_tokens(ledger: &Ledger): u64 {
        ledger.base_tokens
    }

    public fun get_native_token_amount(ledger: &Ledger, native_token_type: &String): u64 {
        let (found, i) = ledger.native_token_types.index_of(native_token_type);
        if (!found) {
            return 0
        };
        ledger.native_token_amounts[i]
    }

    public fun get_native_token_types(ledger: &Ledger): &vector<String> {
        &ledger.native_token_types
    }

    public fun get_nfts(ledger: &Ledger): &vector<ID> {
        &ledger.nfts
    }

    public fun has_nft(ledger: &Ledger, nft: ID): bool {
        ledger.nfts.contains(&nft)
    }
}
