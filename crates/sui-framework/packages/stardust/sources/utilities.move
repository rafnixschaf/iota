module stardust::utilities{
    // === Imports ===
    use std::type_name::get;
    
    use sui::{
        bag::{Bag},
        balance::{Balance},
        coin::{from_balance},
        transfer::{public_transfer},
    };

    // === Errors ===

    // returned when trying to extract a balance<T> from a bag and the balance is zero
    const EZeroNativeTokenBalance: u64 = 0;

    // === Public-Mutative Functions ===

    // extract a balance<T> from a bag, create a coin out of it and send it to an address
    // NOTE: we return the bag by value so the function can be called repeatedly in a PTB
    public fun extract_and_send_to<T>(mut b: Bag, to: address, ctx: &mut TxContext) : Bag  {
        let coin = from_balance(extract_<T>( &mut b), ctx);
        public_transfer(coin, to);
        b
    }

    // extract a balance<T> from a bag and return it. Caller can decide what to do with it
    // NOTE: we return the bag by value so the function can be called repeatedly in a PTB
    public fun extract<T>(mut b: Bag) : (Bag, Balance<T>) {
        let nt = extract_<T>(&mut b);
        (b, nt)
    }

    // === Private Functions ===

    // get a balance<T> from a bag, and abort if the balance is zero or if there is no balance for the <T>
    fun extract_<T>(b: &mut Bag) : Balance<T> {
       let key = get<T>().into_string();
       // this will abort if the key doesn't exist
       let nt : Balance<T> = b.remove(key);
       assert!(nt.value() != 0, EZeroNativeTokenBalance);
       nt
    }
}