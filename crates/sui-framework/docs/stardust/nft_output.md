---
title: Module `0x107a::nft_output`
---



-  [Resource `NftOutput`](#0x107a_nft_output_NftOutput)
-  [Constants](#@Constants_0)
-  [Function `extract_assets`](#0x107a_nft_output_extract_assets)
-  [Function `load_nft`](#0x107a_nft_output_load_nft)


<pre><code><b>use</b> <a href="expiration_unlock_condition.md#0x107a_expiration_unlock_condition">0x107a::expiration_unlock_condition</a>;
<b>use</b> <a href="nft.md#0x107a_nft">0x107a::nft</a>;
<b>use</b> <a href="storage_deposit_return_unlock_condition.md#0x107a_storage_deposit_return_unlock_condition">0x107a::storage_deposit_return_unlock_condition</a>;
<b>use</b> <a href="timelock_unlock_condition.md#0x107a_timelock_unlock_condition">0x107a::timelock_unlock_condition</a>;
<b>use</b> <a href="../move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../sui-framework/bag.md#0x2_bag">0x2::bag</a>;
<b>use</b> <a href="../sui-framework/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../sui-framework/dynamic_object_field.md#0x2_dynamic_object_field">0x2::dynamic_object_field</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/sui.md#0x2_sui">0x2::sui</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
</code></pre>



<a name="0x107a_nft_output_NftOutput"></a>

## Resource `NftOutput`

The Stardust NFT output representation.


<pre><code><b>struct</b> <a href="nft_output.md#0x107a_nft_output_NftOutput">NftOutput</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>iota: <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/sui.md#0x2_sui_SUI">sui::SUI</a>&gt;</code>
</dt>
<dd>
 The amount of IOTA tokens held by the output.
</dd>
<dt>
<code>native_tokens: <a href="../sui-framework/bag.md#0x2_bag_Bag">bag::Bag</a></code>
</dt>
<dd>

</dd>
<dt>
<code>storage_deposit_return: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="storage_deposit_return_unlock_condition.md#0x107a_storage_deposit_return_unlock_condition_StorageDepositReturnUnlockCondition">storage_deposit_return_unlock_condition::StorageDepositReturnUnlockCondition</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>timelock: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="timelock_unlock_condition.md#0x107a_timelock_unlock_condition_TimelockUnlockCondition">timelock_unlock_condition::TimelockUnlockCondition</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>expiration: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="expiration_unlock_condition.md#0x107a_expiration_unlock_condition_ExpirationUnlockCondition">expiration_unlock_condition::ExpirationUnlockCondition</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x107a_nft_output_NFT_NAME"></a>

The NFT dynamic field name.


<pre><code><b>const</b> <a href="nft_output.md#0x107a_nft_output_NFT_NAME">NFT_NAME</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [110, 102, 116];
</code></pre>



<a name="0x107a_nft_output_extract_assets"></a>

## Function `extract_assets`

The function extracts assets from a legacy NFT output.


<pre><code><b>public</b> <b>fun</b> <a href="nft_output.md#0x107a_nft_output_extract_assets">extract_assets</a>(output: <a href="nft_output.md#0x107a_nft_output_NftOutput">nft_output::NftOutput</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): (<a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/sui.md#0x2_sui_SUI">sui::SUI</a>&gt;, <a href="../sui-framework/bag.md#0x2_bag_Bag">bag::Bag</a>, <a href="nft.md#0x107a_nft_Nft">nft::Nft</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="nft_output.md#0x107a_nft_output_extract_assets">extract_assets</a>(<b>mut</b> output: <a href="nft_output.md#0x107a_nft_output_NftOutput">NftOutput</a>, ctx: &<b>mut</b> TxContext): (Balance&lt;SUI&gt;, Bag, Nft) {
    // Load the related Nft <a href="../sui-framework/object.md#0x2_object">object</a>.
    <b>let</b> <a href="nft.md#0x107a_nft">nft</a> = <a href="nft_output.md#0x107a_nft_output_load_nft">load_nft</a>(&<b>mut</b> output);

    // Unpuck the output.
    <b>let</b> <a href="nft_output.md#0x107a_nft_output_NftOutput">NftOutput</a> {
        id: id,
        iota: <b>mut</b> iota,
        native_tokens: native_tokens,
        storage_deposit_return: <b>mut</b> storage_deposit_return,
        timelock: <b>mut</b> timelock,
        expiration: <b>mut</b> expiration
    } = output;

    // If the output <b>has</b> a timelock, then we need <b>to</b> check <b>if</b> the timelock <b>has</b> expired.
    <b>if</b> (timelock.is_some()) {
        timelock.extract().unlock(ctx);
    };

    // If the output <b>has</b> an expiration, then we need <b>to</b> check who can unlock the output.
    <b>if</b> (expiration.is_some()) {
        expiration.extract().unlock(ctx);
    };

    // If the output <b>has</b> an SDRUC, then we need <b>to</b> <b>return</b> the deposit.
    <b>if</b> (storage_deposit_return.is_some()) {
        storage_deposit_return.extract().unlock(&<b>mut</b> iota, ctx);
    };

    // Destroy the output.
    <a href="../move-stdlib/option.md#0x1_option_destroy_none">option::destroy_none</a>(timelock);
    <a href="../move-stdlib/option.md#0x1_option_destroy_none">option::destroy_none</a>(expiration);
    <a href="../move-stdlib/option.md#0x1_option_destroy_none">option::destroy_none</a>(storage_deposit_return);

    <a href="../sui-framework/object.md#0x2_object_delete">object::delete</a>(id);

    <b>return</b> (iota, native_tokens, <a href="nft.md#0x107a_nft">nft</a>)
}
</code></pre>



</details>

<a name="0x107a_nft_output_load_nft"></a>

## Function `load_nft`

Loads the related <code>Nft</code> object.


<pre><code><b>fun</b> <a href="nft_output.md#0x107a_nft_output_load_nft">load_nft</a>(output: &<b>mut</b> <a href="nft_output.md#0x107a_nft_output_NftOutput">nft_output::NftOutput</a>): <a href="nft.md#0x107a_nft_Nft">nft::Nft</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="nft_output.md#0x107a_nft_output_load_nft">load_nft</a>(output: &<b>mut</b> <a href="nft_output.md#0x107a_nft_output_NftOutput">NftOutput</a>): Nft {
    <a href="../sui-framework/dynamic_object_field.md#0x2_dynamic_object_field_remove">dynamic_object_field::remove</a>(&<b>mut</b> output.id, <a href="nft_output.md#0x107a_nft_output_NFT_NAME">NFT_NAME</a>)
}
</code></pre>



</details>
