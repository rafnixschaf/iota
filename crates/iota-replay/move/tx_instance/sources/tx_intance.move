module tx_instance::tx_instance {
    use iota::event;
    use iota::tx_context::{Self, TxContext};

    struct TxIntance has copy, drop {
        user: address,
        published: bool
    }

    public entry fun published(ctx: &mut TxContext) {
        event::emit(TxIntance {
            user: tx_context::sender(ctx),
            published: true
        })
    }
}
