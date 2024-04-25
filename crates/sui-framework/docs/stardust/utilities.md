---
title: Module `0x107a::utilities`
---



-  [Constants](#@Constants_0)
-  [Function `create_coin_from_option_balance`](#0x107a_utilities_create_coin_from_option_balance)
-  [Function `extract_and_send_to`](#0x107a_utilities_extract_and_send_to)
-  [Function `extract`](#0x107a_utilities_extract)
-  [Function `extract_`](#0x107a_utilities_extract_)


<pre><code><b>use</b> <a href="../move-stdlib/ascii.md#0x1_ascii">0x1::ascii</a>;
<b>use</b> <a href="../move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../move-stdlib/type_name.md#0x1_type_name">0x1::type_name</a>;
<b>use</b> <a href="../sui-framework/bag.md#0x2_bag">0x2::bag</a>;
<b>use</b> <a href="../sui-framework/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../sui-framework/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../sui-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
</code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="0x107a_utilities_EZeroNativeTokenBalance"></a>



<pre><code><b>const</b> <a href="utilities.md#0x107a_utilities_EZeroNativeTokenBalance">EZeroNativeTokenBalance</a>: u64 = 0;
</code></pre>



<a name="0x107a_utilities_create_coin_from_option_balance"></a>

## Function `create_coin_from_option_balance`



<pre><code><b>public</b> <b>fun</b> <a href="utilities.md#0x107a_utilities_create_coin_from_option_balance">create_coin_from_option_balance</a>&lt;T&gt;(b: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;T&gt;&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="utilities.md#0x107a_utilities_create_coin_from_option_balance">create_coin_from_option_balance</a>&lt;T&gt;(<b>mut</b> b: Option&lt;Balance&lt;T&gt;&gt;, ctx: &<b>mut</b> TxContext) : Coin&lt;T&gt; {
    <b>assert</b>!(b.is_some(), 0);
    <b>let</b> eb = b.<a href="utilities.md#0x107a_utilities_extract">extract</a>();
    b.destroy_none();
    from_balance(eb, ctx)
}
</code></pre>



</details>

<a name="0x107a_utilities_extract_and_send_to"></a>

## Function `extract_and_send_to`



<pre><code><b>public</b> <b>fun</b> <a href="utilities.md#0x107a_utilities_extract_and_send_to">extract_and_send_to</a>&lt;T&gt;(b: <a href="../sui-framework/bag.md#0x2_bag_Bag">bag::Bag</a>, <b>to</b>: <b>address</b>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../sui-framework/bag.md#0x2_bag_Bag">bag::Bag</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="utilities.md#0x107a_utilities_extract_and_send_to">extract_and_send_to</a>&lt;T&gt;(<b>mut</b> b: Bag, <b>to</b>: <b>address</b>, ctx: &<b>mut</b> TxContext) : Bag  {
    <b>let</b> <a href="../sui-framework/coin.md#0x2_coin">coin</a> = from_balance(<a href="utilities.md#0x107a_utilities_extract_">extract_</a>&lt;T&gt;( &<b>mut</b> b), ctx);
    public_transfer(<a href="../sui-framework/coin.md#0x2_coin">coin</a>, <b>to</b>);
    b
}
</code></pre>



</details>

<a name="0x107a_utilities_extract"></a>

## Function `extract`



<pre><code><b>public</b> <b>fun</b> <a href="utilities.md#0x107a_utilities_extract">extract</a>&lt;T&gt;(b: <a href="../sui-framework/bag.md#0x2_bag_Bag">bag::Bag</a>): (<a href="../sui-framework/bag.md#0x2_bag_Bag">bag::Bag</a>, <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="utilities.md#0x107a_utilities_extract">extract</a>&lt;T&gt;(<b>mut</b> b: Bag) : (Bag, Balance&lt;T&gt;) {
    <b>let</b> nt = <a href="utilities.md#0x107a_utilities_extract_">extract_</a>&lt;T&gt;(&<b>mut</b> b);
    (b, nt)
}
</code></pre>



</details>

<a name="0x107a_utilities_extract_"></a>

## Function `extract_`



<pre><code><b>fun</b> <a href="utilities.md#0x107a_utilities_extract_">extract_</a>&lt;T&gt;(b: &<b>mut</b> <a href="../sui-framework/bag.md#0x2_bag_Bag">bag::Bag</a>): <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="utilities.md#0x107a_utilities_extract_">extract_</a>&lt;T&gt;(b: &<b>mut</b> Bag) : Balance&lt;T&gt; {
   <b>let</b> key = get&lt;T&gt;().into_string();
   // this will <b>abort</b> <b>if</b> the key doesn't exist
   <b>let</b> nt : Balance&lt;T&gt; = b.remove(key);
   <b>assert</b>!(nt.value() != 0, <a href="utilities.md#0x107a_utilities_EZeroNativeTokenBalance">EZeroNativeTokenBalance</a>);
   nt
}
</code></pre>



</details>
