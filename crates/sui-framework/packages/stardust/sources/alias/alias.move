module stardust::alias{
    use sui::balance::{Self, Balance};
    use sui::dynamic_field;
    use sui::sui::SUI;
    use sui::transfer::{Receiving};
    use sui::bag::{Bag};

    const ENotStateControllerAliasId: u64 = 0;
    const ENotStateControllerVersion: u64 = 1;
    const ENotGovernor: u64 = 2;

    /// The tokens bag dynamic field name.
    const TOKENS_NAME: vector<u8> = b"tokens";

    /// The capability owned object that the governor must own. This enables the owner address to execute a 
    /// governance transition.
    public struct GovernorCap has key, store {
      id: UID,
      alias_id: ID,
    }

    /// The capability owned object that the state must own. This enables the state address to execute a 
    /// state transition. The state_cap_version allows the governor to deprecate the state address.
    public struct StateCap has key {
      id: UID,
      alias_id: ID,
      state_cap_version: u64,
    }

    /// Shared Object that can be controlled with the GovernorCap and StateControllerCap.
    public struct AliasOutput has key {
      /// The ID of the AliasOutput.
      /// This is the hash of the Output ID that created the Alias Output.
      /// During the migration, any Alias Output with a zeroed ID must have its corresponding computed Alias ID set.
      /// Alias ID must be kept between the migration from Stardust to Move (for applications like Identity).
      id: UID,
      
      // The amount of IOTA coins held by the output.
      iota: Balance<SUI>,

      // State representation
      // The State Controller Address that can unlock in a transaction that state transitions the alias object. The 
      // state_cap_version enable to validate a StateCap to be able to represent the State Controller Address.
      state_cap_version: u64,
      // A counter that must increase by 1 every time the alias is state transitioned.
      state_index: u32,
      // Metadata that can only be changed by the state controller. 
      state_metadata: Option<vector<u8>>,

      // Features
      // sender feature
      sender: Option<address>,
      // DID<version_byte><encoding_byte>
      metadata: Option<vector<u8>>,

      // Immutable Features
      issuer: Option<address>,
      immutable_metadata: Option<vector<u8>>,
    }

    // === Public-Mutative Functions ===

    /// Set the state controller address
    public fun governor_set_state_controller(self: &mut AliasOutput, cap: &GovernorCap, state_controller: address, ctx: &mut TxContext) {
      self.assert_governor(cap);
      self.state_cap_version = self.state_cap_version + 1;

      transfer::transfer(StateCap {
        id: object::new(ctx),
        alias_id: self.id.uid_to_inner(),
        state_cap_version: self.state_cap_version,
      }, state_controller);
    }

    // extract the assets inside the output
    //  - the object will NOT be destroyd
    //  - remaining assets (iota coins and native tokens) will be returned
    public fun extract_assets(
        self: &mut AliasOutput,
        cap: &StateCap,
    ) : (Balance<SUI>, Option<Bag>) {
        self.state_index_increment(cap);
        // unpack the output into its basic part
        let AliasOutput {
          id: _,
          iota,
          state_cap_version: _,
          state_index: _,
          state_metadata: _,
          sender: _,
          metadata: _,
          issuer: _,
          immutable_metadata: _,
        } = self;
        // extract all iota coins
        let all_iota = balance::withdraw_all(iota);
        // extract the native tokens bag 
        let tokens = self.extract_tokens();

        (all_iota, tokens)
    }

    /// Destroy the AliasOutput Object and return IOTA balance and Bag of tokens
    public fun destroy(self: AliasOutput, cap: GovernorCap) {
      self.assert_governor(&cap);
      let AliasOutput {
        id,
        iota,
        state_cap_version: _,
        state_index: _,
        state_metadata: _,
        sender: _,
        metadata: _,
        issuer: _,
        immutable_metadata: _,
      } = self;

      // iota amount must be zero, extracted by the state address
      iota.destroy_zero();
      
      // destroy the governor cap
      let GovernorCap {
        id: governor_cap_id,
        alias_id: _,
      } = cap;
      object::delete(governor_cap_id);
      
      object::delete(id);
    }

    /// Set the state controller address
    public fun destroy_state_cap(self: &mut AliasOutput, cap: StateCap) {
      assert!(self.id.uid_to_inner() == cap.alias_id, ENotStateControllerAliasId);
      let StateCap {
        id,
        alias_id: _,
        state_cap_version: _,
      } = cap;
      object::delete(id);
    }

    // === Public-Package Functions ===

    /// Get the alias id.
    public(package) fun id(self: &mut AliasOutput): &mut UID {
        &mut self.id
    }

    /// Increment state_index by 1.
    public(package) fun state_index_increment(self: &mut AliasOutput, cap: &StateCap) {
      self.assert_state_controller(cap);
      self.state_index = self.state_index + 1;
    }

    // utility function to receive a alias output in other stardust models
    // other modules in the stardust pacakge can call this function to receive a alias output (nft)
    public(package) fun receive(parent: &mut UID, alias: Receiving<AliasOutput>) : AliasOutput {
        transfer::receive(parent, alias)
    }


    // === Private Functions ===

    /// Assert that the TX sender is equal to the state controller.
    fun assert_state_controller(self: &AliasOutput, cap: &StateCap) {
      assert!(self.id.uid_to_inner() == cap.alias_id, ENotStateControllerAliasId);
      assert!(self.state_cap_version == cap.state_cap_version, ENotStateControllerVersion);
    }

    /// Assert that the GovernorCap governs this alias.
    fun assert_governor(self: &AliasOutput, cap: &GovernorCap) {
      assert!(self.id.uid_to_inner() == cap.alias_id, ENotGovernor);
    }

    /// extracts the related tokens bag object.
    fun extract_tokens(output: &mut AliasOutput): Option<Bag> {
        dynamic_field::remove_if_exists(&mut output.id, TOKENS_NAME)
    }

    // === Test Functions ===

    #[test_only]
    public fun create_for_testing(
        iota: Balance<SUI>,
        state_cap_version: u64,
        state_index: u32,
        state_metadata: Option<vector<u8>>,
        sender: Option<address>,
        metadata: Option<vector<u8>>,
        issuer: Option<address>,
        immutable_metadata: Option<vector<u8>>,
        ctx: &mut TxContext
    ): (AliasOutput, GovernorCap, StateCap)  {
        let id = object::new(ctx);
        let inner = id.uid_to_inner();
        (AliasOutput {
            id,
            iota,
            state_cap_version,
            state_index,
            state_metadata,
            sender,
            metadata,
            issuer,
            immutable_metadata,
        },GovernorCap {
            id: object::new(ctx),
            alias_id: inner,
        }, StateCap {
            id: object::new(ctx),
            alias_id: inner,
            state_cap_version,
        })
    }

    #[test_only]
    public fun attach_tokens(output: &mut AliasOutput, tokens: Bag) {
        dynamic_field::add(&mut output.id, TOKENS_NAME, tokens)
    }
}