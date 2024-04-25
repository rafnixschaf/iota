---
title: Module `0x107a::basic`
---



-  [Resource `BasicOutput`](#0x107a_basic_BasicOutput)
-  [Function `extract_assets`](#0x107a_basic_extract_assets)
-  [Function `receive`](#0x107a_basic_receive)


<pre><code><b>use</b> <a href="expiration_unlock_condition.md#0x107a_expiration_unlock_condition">0x107a::expiration_unlock_condition</a>;
<b>use</b> <a href="storage_deposit_return_unlock_condition.md#0x107a_storage_deposit_return_unlock_condition">0x107a::storage_deposit_return_unlock_condition</a>;
<b>use</b> <a href="timelock_unlock_condition.md#0x107a_timelock_unlock_condition">0x107a::timelock_unlock_condition</a>;
<b>use</b> <a href="../move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../sui-framework/bag.md#0x2_bag">0x2::bag</a>;
<b>use</b> <a href="../sui-framework/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/sui.md#0x2_sui">0x2::sui</a>;
<b>use</b> <a href="../sui-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
</code></pre>



<a name="0x107a_basic_BasicOutput"></a>

## Resource `BasicOutput`



<pre><code><b>struct</b> <a href="basic.md#0x107a_basic_BasicOutput">BasicOutput</a> <b>has</b> key
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

</dd>
<dt>
<code>tokens: <a href="../sui-framework/bag.md#0x2_bag_Bag">bag::Bag</a></code>
</dt>
<dd>

</dd>
<dt>
<code>sdr: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="storage_deposit_return_unlock_condition.md#0x107a_storage_deposit_return_unlock_condition_StorageDepositReturnUnlockCondition">storage_deposit_return_unlock_condition::StorageDepositReturnUnlockCondition</a>&gt;</code>
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
<dt>
<code>metadata: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>tag: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>sender: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x107a_basic_extract_assets"></a>

## Function `extract_assets`



<pre><code><b>public</b> <b>fun</b> <a href="basic.md#0x107a_basic_extract_assets">extract_assets</a>(output: <a href="basic.md#0x107a_basic_BasicOutput">basic::BasicOutput</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): (<a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/sui.md#0x2_sui_SUI">sui::SUI</a>&gt;&gt;, <a href="../sui-framework/bag.md#0x2_bag_Bag">bag::Bag</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="basic.md#0x107a_basic_extract_assets">extract_assets</a>(
    // the output <b>to</b> be migrated
    output: <a href="basic.md#0x107a_basic_BasicOutput">BasicOutput</a>,
    ctx: &<b>mut</b> TxContext
) : (Option&lt;Balance&lt;SUI&gt;&gt;, Bag) {
    <b>let</b> <b>mut</b> extracted_base_token : Option&lt;Balance&lt;SUI&gt;&gt; = none();

    // unpack the output into its <a href="basic.md#0x107a_basic">basic</a> part
    <b>let</b> <a href="basic.md#0x107a_basic_BasicOutput">BasicOutput</a> {
        id: id_to_delete,
        iota: <b>mut</b> iota_balance,
        tokens: tokens,
        // `none` options can be dropped
        sdr: <b>mut</b> sdr,
        timelock: <b>mut</b> timelock,
        expiration: <b>mut</b> expiration,
        // the features have `drop` so we can just ignore them
        sender: _,
        metadata: _,
        tag: _ } = output;

    // <b>if</b> the output <b>has</b> a timelock, then we need <b>to</b> check <b>if</b> the timelock <b>has</b> expired
    <b>if</b> (timelock.is_some()) {
        // extract will make the <a href="../move-stdlib/option.md#0x1_option">option</a> `None`
        timelock.extract().unlock(ctx);
    };

    // <b>if</b> the output <b>has</b> an expiration, then we need <b>to</b> check who can unlock the output
    <b>if</b> (expiration.is_some()) {
        // extract will make the <a href="../move-stdlib/option.md#0x1_option">option</a> `None`
        expiration.extract().unlock(ctx);
    };

    // <b>if</b> the output <b>has</b> an SDRUC, then we need <b>to</b> <b>return</b> the deposit
    <b>if</b> (sdr.is_some()) {
        // extract will make the <a href="../move-stdlib/option.md#0x1_option">option</a> `None`
        sdr.extract().unlock(&<b>mut</b> iota_balance, ctx);
    };

    // Destroy the options
    <a href="../move-stdlib/option.md#0x1_option_destroy_none">option::destroy_none</a>(timelock);
    <a href="../move-stdlib/option.md#0x1_option_destroy_none">option::destroy_none</a>(expiration);
    <a href="../move-stdlib/option.md#0x1_option_destroy_none">option::destroy_none</a>(sdr);

    // fil lthe <b>return</b> value <b>with</b> the remaining IOTA <a href="../sui-framework/balance.md#0x2_balance">balance</a>
    <b>let</b> iotas = iota_balance.value();
    <b>if</b> (iotas &gt; 0) {
        // there is a <a href="../sui-framework/balance.md#0x2_balance">balance</a> remaining after fuflilling SDRUC
        extracted_base_token.fill(iota_balance);
    } <b>else</b> {
        // SDRUC consumed all the <a href="../sui-framework/balance.md#0x2_balance">balance</a> of the output
        iota_balance.destroy_zero();
    };

    // delete the output <a href="../sui-framework/object.md#0x2_object">object</a>'s UID
    delete_object(id_to_delete);

    <b>return</b> (extracted_base_token, tokens)
}
</code></pre>



</details>

<a name="0x107a_basic_receive"></a>

## Function `receive`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="basic.md#0x107a_basic_receive">receive</a>(parent: &<b>mut</b> <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>, <a href="basic.md#0x107a_basic">basic</a>: <a href="../sui-framework/transfer.md#0x2_transfer_Receiving">transfer::Receiving</a>&lt;<a href="basic.md#0x107a_basic_BasicOutput">basic::BasicOutput</a>&gt;): <a href="basic.md#0x107a_basic_BasicOutput">basic::BasicOutput</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="basic.md#0x107a_basic_receive">receive</a>(parent: &<b>mut</b> UID, <a href="basic.md#0x107a_basic">basic</a>: Receiving&lt;<a href="basic.md#0x107a_basic_BasicOutput">BasicOutput</a>&gt;) : <a href="basic.md#0x107a_basic_BasicOutput">BasicOutput</a> {
    <a href="../sui-framework/transfer.md#0x2_transfer_receive">transfer::receive</a>(parent, <a href="basic.md#0x107a_basic">basic</a>)
}
</code></pre>



</details>
