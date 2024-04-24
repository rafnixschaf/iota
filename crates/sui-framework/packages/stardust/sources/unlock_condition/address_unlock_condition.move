module stardust::address_unlock_condition{
    use sui::coin::{TreasuryCap};
    use sui::transfer::{Receiving};
    use stardust::basic::{Self,BasicOutput};
    use stardust::nft_output::{Self,NftOutput};
    use stardust::alias::{Self,AliasOutput,StateCap};

    /// Unlock Basic outputs locked to this alias address
    public fun unlock_alias_address_owned_basic(
      self: &mut AliasOutput,
      cap: &StateCap,
      output_to_unlock: Receiving<BasicOutput>,
      ): BasicOutput {
        self.state_index_increment(cap);
        basic::receive(self.id(), output_to_unlock)
    }

    /// Unlock NFT outputs locked to this alias address
    public fun unlock_alias_address_owned_nft(
      self: &mut AliasOutput,
      cap: &StateCap,
      output_to_unlock: Receiving<NftOutput>,
      ): NftOutput {
        self.state_index_increment(cap);
        nft_output::receive(self.id(), output_to_unlock)
    }

    /// Unlock Alias outputs locked to this alias address
    public fun unlock_alias_address_owned_alias(
      self: &mut AliasOutput,
      cap: &StateCap,
      output_to_unlock: Receiving<AliasOutput>,
      ): AliasOutput {
        self.state_index_increment(cap);
        alias::receive(self.id(), output_to_unlock)
    }

    /// Unlock Alias outputs locked to this alias address
    public fun unlock_alias_address_owned_treasury<T: key + store>(
      self: &mut AliasOutput,
      cap: &StateCap,
      treasury_cap: Receiving<TreasuryCap<T>>,
      ): TreasuryCap<T> {
        self.state_index_increment(cap);
        transfer::public_receive(self.id(), treasury_cap)
    }

    
    /// Unlock Basic outputs locked to this alias address
    public fun unlock_nft_address_owned_basic(
      self: &mut NftOutput,
      output_to_unlock: Receiving<BasicOutput>,
      ): BasicOutput {
        basic::receive(self.id(), output_to_unlock)
    }

    /// Unlock NFT outputs locked to this alias address
    public fun unlock_nft_address_owned_nft(
      self: &mut NftOutput,
      output_to_unlock: Receiving<NftOutput>,
      ): NftOutput {
        nft_output::receive(self.id(), output_to_unlock)
    }

    /// Unlock Alias outputs locked to this alias address
    public fun unlock_nft_address_owned_alias(
      self: &mut NftOutput,
      output_to_unlock: Receiving<AliasOutput>,
      ): AliasOutput {
        alias::receive(self.id(), output_to_unlock)
    }
}