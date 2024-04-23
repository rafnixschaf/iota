module stardust::utilities{
    // === Imports ===
    use std::type_name::get;
    
    use sui::{
        bag::{Bag},
        balance::{Balance},
        coin::{Coin, from_balance},
        transfer::{public_transfer},
    };

    // === Errors ===

    // returned when trying to extract a balance<T> from a bag and the balance is zero
    const EZeroNativeTokenBalance: u64 = 0;

    // === Public-Mutative Functions ===

    // utility function for the claiming flow that can be called in a PTB
    // it creates a coin from an option<balance<T>>
    // aborts, if the balance is none
    public fun create_coin_from_option_balance<T>(mut b: Option<Balance<T>>, ctx: &mut TxContext) : Coin<T> {
        assert!(b.is_some(), 0);
        let eb = b.extract();
        b.destroy_none();
        from_balance(eb, ctx)
    }

    // get a balance<T> from a bag, and abort if the balance is zero or if there is no balance for the <T>
    fun extract<T>(b: &mut Bag) : Balance<T> {
       let key = get<T>().into_string();
       // this will abort if the key doesn't exist
       let nt : Balance<T> = b.remove(key);
       assert!(nt.value() != 0, EZeroNativeTokenBalance);
       nt
    }

    // extract a balance<T> from a bag, create a coin out of it and send it to an address
    public fun extract_and_send_to<T>(b: &mut Bag, to: address, ctx: &mut TxContext)  {
        let coin = from_balance(extract<T>(b), ctx);
        public_transfer(coin, to);
    }
}