---
title: Module `0x107a::alias`
---



-  [Resource `GovernorCap`](#0x107a_alias_GovernorCap)
-  [Resource `StateCap`](#0x107a_alias_StateCap)
-  [Resource `AliasOutput`](#0x107a_alias_AliasOutput)
-  [Constants](#@Constants_0)
-  [Function `governor_set_state_controller`](#0x107a_alias_governor_set_state_controller)
-  [Function `extract_assets`](#0x107a_alias_extract_assets)
-  [Function `destroy`](#0x107a_alias_destroy)
-  [Function `destroy_state_cap`](#0x107a_alias_destroy_state_cap)
-  [Function `id`](#0x107a_alias_id)
-  [Function `state_index_increment`](#0x107a_alias_state_index_increment)
-  [Function `receive`](#0x107a_alias_receive)
-  [Function `assert_state_controller`](#0x107a_alias_assert_state_controller)
-  [Function `assert_governor`](#0x107a_alias_assert_governor)
-  [Function `extract_tokens`](#0x107a_alias_extract_tokens)


<pre><code><b>use</b> <a href="../move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../sui-framework/bag.md#0x2_bag">0x2::bag</a>;
<b>use</b> <a href="../sui-framework/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field">0x2::dynamic_field</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/sui.md#0x2_sui">0x2::sui</a>;
<b>use</b> <a href="../sui-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
</code></pre>



<a name="0x107a_alias_GovernorCap"></a>

## Resource `GovernorCap`

The capability owned object that the governor must own. This enables the owner address to execute a
governance transition.


<pre><code><b>struct</b> <a href="alias.md#0x107a_alias_GovernorCap">GovernorCap</a> <b>has</b> store, key
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
<code>alias_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x107a_alias_StateCap"></a>

## Resource `StateCap`

The capability owned object that the state must own. This enables the state address to execute a
state transition. The state_cap_version allows the governor to deprecate the state address.


<pre><code><b>struct</b> <a href="alias.md#0x107a_alias_StateCap">StateCap</a> <b>has</b> key
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
<code>alias_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>state_cap_version: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x107a_alias_AliasOutput"></a>

## Resource `AliasOutput`

Shared Object that can be controlled with the GovernorCap and StateControllerCap.


<pre><code><b>struct</b> <a href="alias.md#0x107a_alias_AliasOutput">AliasOutput</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>
 The ID of the AliasOutput.
 This is the hash of the Output ID that created the Alias Output.
 During the migration, any Alias Output with a zeroed ID must have its corresponding computed Alias ID set.
 Alias ID must be kept between the migration from Stardust to Move (for applications like Identity).
</dd>
<dt>
<code>iota: <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/sui.md#0x2_sui_SUI">sui::SUI</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>state_cap_version: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>state_index: u32</code>
</dt>
<dd>

</dd>
<dt>
<code>state_metadata: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>sender: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>metadata: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>issuer: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>immutable_metadata: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x107a_alias_ENotGovernor"></a>



<pre><code><b>const</b> <a href="alias.md#0x107a_alias_ENotGovernor">ENotGovernor</a>: u64 = 2;
</code></pre>



<a name="0x107a_alias_ENotStateControllerAliasId"></a>



<pre><code><b>const</b> <a href="alias.md#0x107a_alias_ENotStateControllerAliasId">ENotStateControllerAliasId</a>: u64 = 0;
</code></pre>



<a name="0x107a_alias_ENotStateControllerVersion"></a>



<pre><code><b>const</b> <a href="alias.md#0x107a_alias_ENotStateControllerVersion">ENotStateControllerVersion</a>: u64 = 1;
</code></pre>



<a name="0x107a_alias_TOKENS_NAME"></a>

The tokens bag dynamic field name.


<pre><code><b>const</b> <a href="alias.md#0x107a_alias_TOKENS_NAME">TOKENS_NAME</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [116, 111, 107, 101, 110, 115];
</code></pre>



<a name="0x107a_alias_governor_set_state_controller"></a>

## Function `governor_set_state_controller`

Set the state controller address


<pre><code><b>public</b> <b>fun</b> <a href="alias.md#0x107a_alias_governor_set_state_controller">governor_set_state_controller</a>(self: &<b>mut</b> <a href="alias.md#0x107a_alias_AliasOutput">alias::AliasOutput</a>, cap: &<a href="alias.md#0x107a_alias_GovernorCap">alias::GovernorCap</a>, state_controller: <b>address</b>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="alias.md#0x107a_alias_governor_set_state_controller">governor_set_state_controller</a>(self: &<b>mut</b> <a href="alias.md#0x107a_alias_AliasOutput">AliasOutput</a>, cap: &<a href="alias.md#0x107a_alias_GovernorCap">GovernorCap</a>, state_controller: <b>address</b>, ctx: &<b>mut</b> TxContext) {
  self.<a href="alias.md#0x107a_alias_assert_governor">assert_governor</a>(cap);
  self.state_cap_version = self.state_cap_version + 1;

  <a href="../sui-framework/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(<a href="alias.md#0x107a_alias_StateCap">StateCap</a> {
    id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
    alias_id: self.id.uid_to_inner(),
    state_cap_version: self.state_cap_version,
  }, state_controller);
}
</code></pre>



</details>

<a name="0x107a_alias_extract_assets"></a>

## Function `extract_assets`



<pre><code><b>public</b> <b>fun</b> <a href="alias.md#0x107a_alias_extract_assets">extract_assets</a>(self: &<b>mut</b> <a href="alias.md#0x107a_alias_AliasOutput">alias::AliasOutput</a>, cap: &<a href="alias.md#0x107a_alias_StateCap">alias::StateCap</a>): (<a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/sui.md#0x2_sui_SUI">sui::SUI</a>&gt;, <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../sui-framework/bag.md#0x2_bag_Bag">bag::Bag</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="alias.md#0x107a_alias_extract_assets">extract_assets</a>(
    self: &<b>mut</b> <a href="alias.md#0x107a_alias_AliasOutput">AliasOutput</a>,
    cap: &<a href="alias.md#0x107a_alias_StateCap">StateCap</a>,
) : (Balance&lt;SUI&gt;, Option&lt;Bag&gt;) {
    self.<a href="alias.md#0x107a_alias_state_index_increment">state_index_increment</a>(cap);
    // unpack the output into its <a href="basic.md#0x107a_basic">basic</a> part
    <b>let</b> <a href="alias.md#0x107a_alias_AliasOutput">AliasOutput</a> {
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
    <b>let</b> all_iota = <a href="../sui-framework/balance.md#0x2_balance_withdraw_all">balance::withdraw_all</a>(iota);
    // extract the <b>native</b> tokens <a href="../sui-framework/bag.md#0x2_bag">bag</a>
    <b>let</b> tokens = self.<a href="alias.md#0x107a_alias_extract_tokens">extract_tokens</a>();

    (all_iota, tokens)
}
</code></pre>



</details>

<a name="0x107a_alias_destroy"></a>

## Function `destroy`

Destroy the AliasOutput Object and return IOTA balance and Bag of tokens


<pre><code><b>public</b> <b>fun</b> <a href="alias.md#0x107a_alias_destroy">destroy</a>(self: <a href="alias.md#0x107a_alias_AliasOutput">alias::AliasOutput</a>, cap: <a href="alias.md#0x107a_alias_GovernorCap">alias::GovernorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="alias.md#0x107a_alias_destroy">destroy</a>(self: <a href="alias.md#0x107a_alias_AliasOutput">AliasOutput</a>, cap: <a href="alias.md#0x107a_alias_GovernorCap">GovernorCap</a>) {
  self.<a href="alias.md#0x107a_alias_assert_governor">assert_governor</a>(&cap);
  <b>let</b> <a href="alias.md#0x107a_alias_AliasOutput">AliasOutput</a> {
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

  // iota amount must be zero, extracted by the state <b>address</b>
  iota.destroy_zero();

  // destroy the governor cap
  <b>let</b> <a href="alias.md#0x107a_alias_GovernorCap">GovernorCap</a> {
    id: governor_cap_id,
    alias_id: _,
  } = cap;
  <a href="../sui-framework/object.md#0x2_object_delete">object::delete</a>(governor_cap_id);

  <a href="../sui-framework/object.md#0x2_object_delete">object::delete</a>(id);
}
</code></pre>



</details>

<a name="0x107a_alias_destroy_state_cap"></a>

## Function `destroy_state_cap`

Set the state controller address


<pre><code><b>public</b> <b>fun</b> <a href="alias.md#0x107a_alias_destroy_state_cap">destroy_state_cap</a>(self: &<b>mut</b> <a href="alias.md#0x107a_alias_AliasOutput">alias::AliasOutput</a>, cap: <a href="alias.md#0x107a_alias_StateCap">alias::StateCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="alias.md#0x107a_alias_destroy_state_cap">destroy_state_cap</a>(self: &<b>mut</b> <a href="alias.md#0x107a_alias_AliasOutput">AliasOutput</a>, cap: <a href="alias.md#0x107a_alias_StateCap">StateCap</a>) {
  <b>assert</b>!(self.id.uid_to_inner() == cap.alias_id, <a href="alias.md#0x107a_alias_ENotStateControllerAliasId">ENotStateControllerAliasId</a>);
  <b>let</b> <a href="alias.md#0x107a_alias_StateCap">StateCap</a> {
    id,
    alias_id: _,
    state_cap_version: _,
  } = cap;
  <a href="../sui-framework/object.md#0x2_object_delete">object::delete</a>(id);
}
</code></pre>



</details>

<a name="0x107a_alias_id"></a>

## Function `id`

Get the alias id.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="alias.md#0x107a_alias_id">id</a>(self: &<b>mut</b> <a href="alias.md#0x107a_alias_AliasOutput">alias::AliasOutput</a>): &<b>mut</b> <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="alias.md#0x107a_alias_id">id</a>(self: &<b>mut</b> <a href="alias.md#0x107a_alias_AliasOutput">AliasOutput</a>): &<b>mut</b> UID {
    &<b>mut</b> self.id
}
</code></pre>



</details>

<a name="0x107a_alias_state_index_increment"></a>

## Function `state_index_increment`

Increment state_index by 1.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="alias.md#0x107a_alias_state_index_increment">state_index_increment</a>(self: &<b>mut</b> <a href="alias.md#0x107a_alias_AliasOutput">alias::AliasOutput</a>, cap: &<a href="alias.md#0x107a_alias_StateCap">alias::StateCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="alias.md#0x107a_alias_state_index_increment">state_index_increment</a>(self: &<b>mut</b> <a href="alias.md#0x107a_alias_AliasOutput">AliasOutput</a>, cap: &<a href="alias.md#0x107a_alias_StateCap">StateCap</a>) {
  self.<a href="alias.md#0x107a_alias_assert_state_controller">assert_state_controller</a>(cap);
  self.state_index = self.state_index + 1;
}
</code></pre>



</details>

<a name="0x107a_alias_receive"></a>

## Function `receive`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="alias.md#0x107a_alias_receive">receive</a>(parent: &<b>mut</b> <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>, <a href="alias.md#0x107a_alias">alias</a>: <a href="../sui-framework/transfer.md#0x2_transfer_Receiving">transfer::Receiving</a>&lt;<a href="alias.md#0x107a_alias_AliasOutput">alias::AliasOutput</a>&gt;): <a href="alias.md#0x107a_alias_AliasOutput">alias::AliasOutput</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="alias.md#0x107a_alias_receive">receive</a>(parent: &<b>mut</b> UID, <a href="alias.md#0x107a_alias">alias</a>: Receiving&lt;<a href="alias.md#0x107a_alias_AliasOutput">AliasOutput</a>&gt;) : <a href="alias.md#0x107a_alias_AliasOutput">AliasOutput</a> {
    <a href="../sui-framework/transfer.md#0x2_transfer_receive">transfer::receive</a>(parent, <a href="alias.md#0x107a_alias">alias</a>)
}
</code></pre>



</details>

<a name="0x107a_alias_assert_state_controller"></a>

## Function `assert_state_controller`

Assert that the TX sender is equal to the state controller.


<pre><code><b>fun</b> <a href="alias.md#0x107a_alias_assert_state_controller">assert_state_controller</a>(self: &<a href="alias.md#0x107a_alias_AliasOutput">alias::AliasOutput</a>, cap: &<a href="alias.md#0x107a_alias_StateCap">alias::StateCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="alias.md#0x107a_alias_assert_state_controller">assert_state_controller</a>(self: &<a href="alias.md#0x107a_alias_AliasOutput">AliasOutput</a>, cap: &<a href="alias.md#0x107a_alias_StateCap">StateCap</a>) {
  <b>assert</b>!(self.id.uid_to_inner() == cap.alias_id, <a href="alias.md#0x107a_alias_ENotStateControllerAliasId">ENotStateControllerAliasId</a>);
  <b>assert</b>!(self.state_cap_version == cap.state_cap_version, <a href="alias.md#0x107a_alias_ENotStateControllerVersion">ENotStateControllerVersion</a>);
}
</code></pre>



</details>

<a name="0x107a_alias_assert_governor"></a>

## Function `assert_governor`

Assert that the GovernorCap governs this alias.


<pre><code><b>fun</b> <a href="alias.md#0x107a_alias_assert_governor">assert_governor</a>(self: &<a href="alias.md#0x107a_alias_AliasOutput">alias::AliasOutput</a>, cap: &<a href="alias.md#0x107a_alias_GovernorCap">alias::GovernorCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="alias.md#0x107a_alias_assert_governor">assert_governor</a>(self: &<a href="alias.md#0x107a_alias_AliasOutput">AliasOutput</a>, cap: &<a href="alias.md#0x107a_alias_GovernorCap">GovernorCap</a>) {
  <b>assert</b>!(self.id.uid_to_inner() == cap.alias_id, <a href="alias.md#0x107a_alias_ENotGovernor">ENotGovernor</a>);
}
</code></pre>



</details>

<a name="0x107a_alias_extract_tokens"></a>

## Function `extract_tokens`

extracts the related tokens bag object.


<pre><code><b>fun</b> <a href="alias.md#0x107a_alias_extract_tokens">extract_tokens</a>(output: &<b>mut</b> <a href="alias.md#0x107a_alias_AliasOutput">alias::AliasOutput</a>): <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../sui-framework/bag.md#0x2_bag_Bag">bag::Bag</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="alias.md#0x107a_alias_extract_tokens">extract_tokens</a>(output: &<b>mut</b> <a href="alias.md#0x107a_alias_AliasOutput">AliasOutput</a>): Option&lt;Bag&gt; {
    <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_remove_if_exists">dynamic_field::remove_if_exists</a>(&<b>mut</b> output.id, <a href="alias.md#0x107a_alias_TOKENS_NAME">TOKENS_NAME</a>)
}
</code></pre>



</details>
