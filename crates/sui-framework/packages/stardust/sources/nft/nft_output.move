// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module stardust::nft_output {

    use sui::balance::Balance;
    use sui::dynamic_field;
    use sui::sui::SUI;

    use stardust::nft::Nft;

    use stardust::expiration_unlock_condition::ExpirationUnlockCondition;
    use stardust::storage_deposit_return_unlock_condition::StorageDepositReturnUnlockCondition;
    use stardust::timelock_unlock_condition::TimelockUnlockCondition;

    /// The NFT dynamic field name.
    const NFT_NAME: vector<u8> = b"nft";

    /// The Stardust NFT output representation.
    public struct NftOutput has key {
        id: UID,

        /// The amount of IOTA tokens held by the output.
        iota: Balance<SUI>,

        // The storage deposit return unlock condition.
        storage_deposit_return: Option<StorageDepositReturnUnlockCondition>,
        // The timelock unlock condition
        timelock: Option<TimelockUnlockCondition>,
        // The expiration unlock condition
        expiration: Option<ExpirationUnlockCondition>,
    }

    /// The function extracts assets from a legacy NFT output.
    public fun extract_assets(mut output: NftOutput, ctx: &mut TxContext): (Balance<SUI>, Nft) {
        // Load the related Nft object.
        let nft = load_nft(&mut output);

        // Unpuck the output.
        let NftOutput {
            id: id,
            iota: mut iota,
            storage_deposit_return: mut storage_deposit_return,
            timelock: mut timelock,
            expiration: mut expiration
        } = output;

        // If the output has a timelock, then we need to check if the timelock has expired.
        if (timelock.is_some()) {
            timelock.extract().unlock(ctx);
        };

        // If the output has an expiration, then we need to check who can unlock the output.
        if (expiration.is_some()) {
            expiration.extract().unlock(ctx);
        };

        // If the output has an SDRUC, then we need to return the deposit.
        if (storage_deposit_return.is_some()) {
            storage_deposit_return.extract().unlock(&mut iota, ctx);
        };

        // Destroy the output.
        object::delete(id);

        return (iota, nft)
    }

    /// Loads the related `Nft` object.
    fun load_nft(output: &mut NftOutput): Nft {
        dynamic_field::remove(&mut output.id, NFT_NAME)
    }
}
