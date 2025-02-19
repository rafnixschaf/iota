module examples::trade_in {
    use iota::iota::IOTA;
    use iota::coin::{Self, Coin};

    /// Price for the first phone model in the series
    const MODEL_ONE_PRICE: u64 = 10000;

    /// Price for the second phone model
    const MODEL_TWO_PRICE: u64 = 20000;

    /// Error when attempting to purchase a non-existent model
    const EWrongModel: u64 = 1;

    /// Error when the paid amount does not match the required price
    const EIncorrectAmount: u64 = 2;

    /// Struct representing a phone, which can be purchased or traded in for a newer model
    public struct Phone has key, store { id: UID, model: u8 }

    /// Struct representing a payable receipt. 
    /// This receipt must be used immediately in one of the payment functions:
    /// either `trade_in` or `pay_full`. It cannot be stored, owned, or dropped.
    public struct Receipt { price: u64 }

    /// Function to initiate the purchase of a phone, with the payment deferred.
    /// The `Receipt` returned must be passed
    /// to either the `pay_full` or `trade_in` functions to complete the transaction.
    public fun buy_phone(model: u8, ctx: &mut TxContext): (Phone, Receipt) {
        assert!(model == 1 || model == 2, EWrongModel);

        let price = if (model == 1) MODEL_ONE_PRICE else MODEL_TWO_PRICE;

        (
            Phone { id: object::new(ctx), model },
            Receipt { price }
        )
    }

    /// Function to pay the full price for the phone and finalize the transaction by consuming the `Receipt`.
    public fun pay_full(receipt: Receipt, payment: Coin<IOTA>) {
        let Receipt { price } = receipt;
        assert!(coin::value(&payment) == price, EIncorrectAmount);

        // For simplicity, transfer directly to the @examples account
        transfer::public_transfer(payment, @examples);
    }

    /// Function to trade in an old phone and receive a 50% discount on the new phone.
    /// The `Receipt` is consumed as part of this transaction.
    public fun trade_in(receipt: Receipt, old_phone: Phone, payment: Coin<IOTA>) {
        let Receipt { price } = receipt;
        let tradein_price = if (old_phone.model == 1) MODEL_ONE_PRICE else MODEL_TWO_PRICE;
        let to_pay = price - (tradein_price / 2);

        assert!(coin::value(&payment) == to_pay, EIncorrectAmount);

        transfer::public_transfer(old_phone, @examples);
        transfer::public_transfer(payment, @examples);
    }
}