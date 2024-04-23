module stardust::basic{
    // std imports
    use std::option::{is_some,none, fill};
    // sui imports
    use sui::balance::{Balance, destroy_zero, value};
    use sui::object::{delete as delete_object};
    use sui::sui::SUI;
    use sui::bag::{Bag};
    use sui::transfer::{Receiving};
    // package imports
    use stardust::expiration_unlock_condition::{ExpirationUnlockCondition as Expiration};
    use stardust::storage_deposit_return_unlock_condition::{StorageDepositReturnUnlockCondition as StorageDepositReturn};
    use stardust::timelock_unlock_condition::{TimelockUnlockCondition as Timelock};

    // errors


    // a basic output that has unlock conditions/features
    //   - basic outputs with expiration unlock condition must be a shared object, since that's the only
    //     way to handle the two possible addresses that can unlock the output
    //   - notice that there is no `store` ability and there is no custom transfer function:
    //       -  you can call `extract_assets`
    //       -  or you can call `receive` in other models to receive a Basic
    public struct BasicOutput has key {
        // hash of the outputId that was migrated
        id: UID,
        // for now IOTA/SMR = SUI bc we use the sui framework
        iota: Balance<SUI>,
        // the bag holds native tokens, key-ed by the stringified type of the asset
        // Example: key: "0xabcded::soon::SOON", value: Balance<0xabcded::soon::SOON>
        tokens: Bag,

        // possible unlock conditions
        sdr: Option<StorageDepositReturn>,
        timelock: Option<Timelock>,
        expiration: Option<Expiration>,

        // possible features
        // they have no effect and only here to hold data until the object is deleted
        metadata: Option<vector<u8>>,
        tag: Option<vector<u8>>,
        sender: Option<address>
    }

    // extract the assets inside the output, respecting the unlock conditions
    //  - the object will be deleted
    //  - SDRUC will return the deposit
    //  - remaining assets (iota coins and native tokens) will be returned
    public fun extract_assets(
        // the output to be migrated
        output: BasicOutput,
        ctx: &mut TxContext
    ) : (Option<Balance<SUI>>, Bag) {
        let mut extracted_base_token : Option<Balance<SUI>> = none();

        // unpack the output into its basic part
        let BasicOutput {
            id: id_to_delete,
            iota: mut iota_balance,
            tokens: tokens,
            // `none` options can be dropped
            sdr: mut sdr,
            timelock: mut timelock,
            expiration: mut expiration,
            // the features have `drop` so we can just ignore them
            sender: _,
            metadata: _,
            tag: _ } = output;
 
        // if the output has a timelock, then we need to check if the timelock has expired
        if (timelock.is_some()) {
            // extract will make the option `None`
            timelock.extract().unlock(ctx);
        };

        // if the output has an expiration, then we need to check who can unlock the output
        if (expiration.is_some()) {
            // extract will make the option `None`
            expiration.extract().unlock(ctx);
        };

        // if the output has an SDRUC, then we need to return the deposit
        if (sdr.is_some()) {
            // extract will make the option `None`
            sdr.extract().unlock(&mut iota_balance, ctx);
        };

        // Destroy the options
        option::destroy_none(timelock);
        option::destroy_none(expiration);
        option::destroy_none(sdr);

        // fil lthe return value with the remaining IOTA balance
        let iotas = iota_balance.value();
        if (iotas > 0) {
            // there is a balance remaining after fuflilling SDRUC
            extracted_base_token.fill(iota_balance);
        } else {
            // SDRUC consumed all the balance of the output
            iota_balance.destroy_zero();
        };

        // delete the output object's UID
        delete_object(id_to_delete);

        return (extracted_base_token, tokens)
    }

    // utility function to receive a basic output in other stardust models
    // since BasicOutput only has `key`, it can not be received via `public_receive`
    // the private receiver must be implemented in its defining module (here)
    // other modules in the stardust pacakge can call this function to receive a basic output (alias, NFT)
    public(package) fun receive(parent: &mut UID, basic: Receiving<BasicOutput>) : BasicOutput {
        transfer::receive(parent, basic)
    }

    // test only function to create a basic output
    #[test_only]
    public fun create_for_testing(
        iotas: Balance<SUI>,
        tokens: Bag,
        sdr: Option<StorageDepositReturn>,
        timelock: Option<Timelock>,
        expiration: Option<Expiration>,
        metadata: Option<vector<u8>>,
        tag: Option<vector<u8>>,
        sender: Option<address>,
        ctx: &mut TxContext
    ): BasicOutput {
        BasicOutput {
            id: object::new(ctx),
            iota: iotas,
            tokens: tokens,
            sdr: sdr,
            timelock: timelock,
            expiration: expiration,
            metadata: metadata,
            tag: tag,
            sender: sender
        }
    }
}
