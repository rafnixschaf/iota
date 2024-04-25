module stardust::alias_output{
    use sui::balance::{Balance};
    use sui::dynamic_object_field;
    use sui::sui::SUI;
    use sui::bag::{Bag};
    use sui::transfer::{Receiving};

    use stardust::alias::{Alias};

    /// The Alias dynamic object field name.
    const ALIAS_NAME: vector<u8> = b"alias";

    /// Owned Object controlled by the Governor Address.
    public struct AliasOutput has key {
      /// This is a "random" UID, not the AliasID from stardust.
      id: UID,
      /// The amount of IOTA coins held by the output.
      iota: Balance<SUI>,
      /// the bag holds native tokens, key-ed by the stringified type of the asset
      /// Example: key: "0xabcded::soon::SOON", value: Balance<0xabcded::soon::SOON>
      tokens: Bag,
    }

    // === Public-Mutative Functions ===
    
    /// The function extracts assets from a legacy Alias output.
    ///    - returns the IOTA Balance,
    ///    - the Tokens Bag,
    ///    - and the Alias object that persists the AliasID=ObjectID from stardust
    public fun extract_assets(mut self: AliasOutput): (Balance<SUI>, Bag, Alias) {
        // Load the related alias object.
        let alias = load_alias(&mut self);

        // unpack the output into its basic part
        let AliasOutput {
          id,
          iota,
          tokens
        } = self;

        object::delete(id);

        (iota, tokens, alias)
    }

    // === Public-Package Functions ===

    /// utility function to receive a alias output in other stardust models
    /// other modules in the stardust pacakge can call this function to receive an alias output (nft)
    public(package) fun receive(parent: &mut UID, alias_output: Receiving<AliasOutput>) : AliasOutput {
        transfer::receive(parent, alias_output)
    }

    // === Private Functions ===

    /// loads the alias object from the dynamic object field.
    fun load_alias(output: &mut AliasOutput): Alias {
        dynamic_object_field::remove(&mut output.id, ALIAS_NAME)
    }

    // === Test Functions ===

    #[test_only]
    public fun create_for_testing(
        iota: Balance<SUI>,
        tokens: Bag,
        ctx: &mut TxContext
    ): AliasOutput  {
        AliasOutput {
            id: object::new(ctx),
            iota,
            tokens,
        }
    }

    #[test_only]
    public fun attach_alias(output: &mut AliasOutput, alias: Alias) {
        dynamic_object_field::add(&mut output.id, ALIAS_NAME, alias)
    }
}