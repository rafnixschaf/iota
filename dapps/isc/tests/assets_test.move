#[test_only]
module isc::assets_tests {
    use isc::assets::{Self,Assets};
    use sui::coin::{Self, Coin};
    use sui::sui::SUI;
    use sui::test_utils;

    public struct IOTA has drop {}

    #[test]
    fun test_assets() {
        let mut context = tx_context::dummy();
        let ctx = &mut context;
        let sui = coin::mint_for_testing<SUI>(5, ctx);
        let iota = coin::mint_for_testing<IOTA>(10, ctx);

        let mut assets = assets::new(ctx);
        assets.add_base_tokens(sui);
        assets.add_native_tokens(iota);
        let suis = assets.take_base_tokens(5);
        let iotas = assets.take_native_tokens<IOTA>(10);
        assets.destroy_empty();
        test_utils::destroy(suis);
        test_utils::destroy(iotas);
   }

    // #[test, expected_failure(abort_code = isc::assets::EDuplicateNft)]
    // fun test_assets_xxx() {
    //     let mut ctx = tx_context::dummy();

    //     let uid = object::new(&mut ctx);
    //     let nft_id = uid.to_inner();
    //     object::delete(uid);

    //     let mut a = assets::new();
    //     a.add_nft(nft_id);
    //     a.add_nft(nft_id);
    // }
}
