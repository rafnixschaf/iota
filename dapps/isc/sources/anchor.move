
/// Module: isc
module isc::anchor {
    use isc::{
        assets::{Self, Assets},
        request::{Request},
    };
    use sui::{
        bag::{Self, Bag},
    };

    /// privileged function was called without authorization
    const EWrongCaller: u64 = 1;

    /// Only one who is allowed to create new `Anchor`s (start new chains).
    public struct AnchorCap has key { id: UID }

    /// Send `AnchorCap` to the publisher
    fun init(ctx: &mut TxContext) {
        transfer::transfer(AnchorCap {
            id: object::new(ctx)
        }, ctx.sender())
    }

    /// Governor of a chain `Anchor`
    public struct GovernorCap has key, store{
        id: UID,
        anchor_id: ID
    }

    public struct Anchor has key, store {
        id: UID,
        assets: Assets,
        /// pool of requests that have not been consumed by ISC chain yet
        requests: vector<Request>,
        /// state index
        state_index: u32,
        // state metadata
        state_metadata: vector<u8>,
        // table that holds all the treasury caps of tokens minted by this chain
        minted_token_treasuries: Bag,
    }

    /// starts a new chain
    public fun start_new_chain(_: &AnchorCap, ctx: &mut TxContext): (Anchor, GovernorCap) {
        let anchor = Anchor{
            id: object::new(ctx),
            assets: assets::new(ctx),
            requests: vector::empty<Request>(),
            state_index: 0,
            state_metadata: vector[],
            minted_token_treasuries: bag::new(ctx),
         };

        let governor = GovernorCap{
            id: object::new(ctx),
            anchor_id: anchor.id.uid_to_inner(),
        };

        (anchor, governor)
    }
 
    /// clients call this to send a request to the anchor
    public fun send_request(anchor: &Anchor, req: Request, _ctx: &mut TxContext) {
        // we could implement specific checks here for the request

        // send the request object to the chain's id
        transfer::public_transfer(req, anchor.id.to_address());
    }

    fun check_governor(anchor: &Anchor, governor: &GovernorCap) {
        assert!(governor.anchor_id == anchor.id.uid_to_inner(), EWrongCaller);
     }

    //TODO: how to know when to call `receive_request`?
    // Generate event upon `send_request`? Or call periodically? Or both?

    /// chain governor calls this to receive a request and add it to the request pool
    public fun receive_request(anchor: &mut Anchor, governor: &GovernorCap, request: transfer::Receiving<Request>, _ctx: &mut TxContext){
        check_governor(anchor, governor);
        let mut req = transfer::public_receive(&mut anchor.id, request);
        let assets = isc::request::get_assets(&mut req);
        anchor.assets.join(assets);
        //TODO assets.join() should create Mutations we can save per Request
        // Mutations are string/amount combinations.
        // 0x... strings indicate nft ids. Otherwise native token type names. "" for base tokens.
        anchor.requests.push_back<Request>(req);
    }

    /// chain governor calls this to transfer base tokens from its `Assets` to an address
    public fun send_base_tokens(anchor: &mut Anchor, governor: &GovernorCap, to: address, amount: u64, ctx: &mut TxContext){
        check_governor(anchor, governor);
        let tokens = anchor.assets.take_base_tokens(amount).into_coin(ctx);
        transfer::public_transfer(tokens, to);
    }

    /// chain governor calls this to transfer native tokens from its `Assets` to an address
    public fun send_native_tokens<T>(anchor: &mut Anchor, governor: &GovernorCap, to: address, amount: u64, ctx: &mut TxContext){
        check_governor(anchor, governor);
        let tokens = anchor.assets.take_native_tokens<T>(amount).into_coin(ctx);
        transfer::public_transfer(tokens, to);
    }

    /// chain governor calls this to transfer an nft from its `Assets` to an address
    public fun send_nft(anchor: &mut Anchor, governor: &GovernorCap, to: address, nft_id: ID, _ctx: &mut TxContext){
        check_governor(anchor, governor);
        let nft = anchor.assets.take_nft(nft_id);
        transfer::public_transfer(nft, to);
    }


////////////////////////////////// SCRATCH \\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\

    // /// PTB 1
    // /// command 1: publish -> TreasuryCap, CoinMetadata
    // ///
    // /// PTB 2
    // /// command 2: call register_isc_token(anchor, treasury, ctx)

    // public fun register_isc_token<T>(anchor: &mut Anchor, treasury: TreasuryCap<T>) {
    //     let token_type: std::ascii::String = type_name::get<T>().into_string();
    //     anchor.minted_token_treasuries.add(token_type, treasury);
    // }

    // /// PTB
    // /// call mint_token -> Coin
    // /// call transfer coin somewhere

    // public fun mint_token<T>(anchor: &mut Anchor, amount: u64, ctx: &mut TxContext): Coin<T> {
    //     let token_type: std::ascii::String = type_name::get<T>().into_string();
    //     let treasury = anchor.minted_token_treasuries.borrow_mut<std::ascii::String, TreasuryCap<T>>(token_type);
    //     treasury.mint(amount, ctx)
    // }
 }

