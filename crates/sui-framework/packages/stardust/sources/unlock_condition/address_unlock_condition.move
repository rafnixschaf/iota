module stardust::address_unlock_condition{
    use sui::coin::{TreasuryCap};
    use sui::transfer::{Receiving};
    use stardust::basic::{Self,BasicOutput};
    use stardust::nft::{Nft};
    use stardust::nft_output::{Self,NftOutput};
    use stardust::alias::{Alias};
    use stardust::alias_output::{Self,AliasOutput};

    // === Receiving on Alias Address/AliasID as ObjectID ===

    /// Unlock Basic outputs locked to this alias address
    public fun unlock_alias_address_owned_basic(
      self: &mut Alias,
      output_to_unlock: Receiving<BasicOutput>,
      ): BasicOutput {
        basic::receive(self.id(), output_to_unlock)
    }

    /// Unlock NFT outputs locked to this alias address
    public fun unlock_alias_address_owned_nft(
      self: &mut Alias,
      output_to_unlock: Receiving<NftOutput>,
      ): NftOutput {
        nft_output::receive(self.id(), output_to_unlock)
    }

    /// Unlock Alias outputs locked to this alias address
    public fun unlock_alias_address_owned_alias(
      self: &mut Alias,
      output_to_unlock: Receiving<AliasOutput>,
      ): AliasOutput {
        alias_output::receive(self.id(), output_to_unlock)
    }

    /// Unlock Alias outputs locked to this alias address
    public fun unlock_alias_address_owned_treasury<T: key + store>(
      self: &mut Alias,
      treasury_cap: Receiving<TreasuryCap<T>>,
      ): TreasuryCap<T> {
        transfer::public_receive(self.id(), treasury_cap)
    }

    // TODO: be able to receive MaxSupplyPolicy from https://github.com/iotaledger/kinesis/pull/145

    // === Receiving on NFT Address/NFTID as ObjectID ===
    
    /// Unlock Basic outputs locked to this alias address
    public fun unlock_nft_address_owned_basic(
      self: &mut Nft,
      output_to_unlock: Receiving<BasicOutput>,
      ): BasicOutput {
        basic::receive(self.id(), output_to_unlock)
    }

    /// Unlock NFT outputs locked to this nft address
    public fun unlock_nft_address_owned_nft(
      self: &mut Nft,
      output_to_unlock: Receiving<NftOutput>,
      ): NftOutput {
        nft_output::receive(self.id(), output_to_unlock)
    }

    /// Unlock Alias outputs locked to this Nft address
    public fun unlock_nft_address_owned_alias(
      self: &mut Nft,
      output_to_unlock: Receiving<AliasOutput>,
      ): AliasOutput {
        alias_output::receive(self.id(), output_to_unlock)
    }
}