---
title: Module `0x3::timelocked_staked_sui`
---



-  [Resource `TimelockedStakedSui`](#0x3_timelocked_staked_sui_TimelockedStakedSui)
-  [Constants](#@Constants_0)
-  [Function `create`](#0x3_timelocked_staked_sui_create)
-  [Function `unwrap_timelocked_staked_sui`](#0x3_timelocked_staked_sui_unwrap_timelocked_staked_sui)
-  [Function `pool_id`](#0x3_timelocked_staked_sui_pool_id)
-  [Function `staked_sui_amount`](#0x3_timelocked_staked_sui_staked_sui_amount)
-  [Function `stake_activation_epoch`](#0x3_timelocked_staked_sui_stake_activation_epoch)
-  [Function `expire_timestamp_ms`](#0x3_timelocked_staked_sui_expire_timestamp_ms)
-  [Function `split`](#0x3_timelocked_staked_sui_split)
-  [Function `split_staked_sui`](#0x3_timelocked_staked_sui_split_staked_sui)
-  [Function `join_staked_sui`](#0x3_timelocked_staked_sui_join_staked_sui)
-  [Function `is_equal_staking_metadata`](#0x3_timelocked_staked_sui_is_equal_staking_metadata)


<pre><code><b>use</b> <a href="../sui-framework/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/sui.md#0x2_sui">0x2::sui</a>;
<b>use</b> <a href="../sui-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
</code></pre>



<a name="0x3_timelocked_staked_sui_TimelockedStakedSui"></a>

## Resource `TimelockedStakedSui`

A self-custodial object holding the timelocked staked SUI tokens.


<pre><code><b>struct</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">TimelockedStakedSui</a> <b>has</b> store, key
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
<code>pool_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 ID of the staking pool we are staking with.
</dd>
<dt>
<code>stake_activation_epoch: u64</code>
</dt>
<dd>
 The epoch at which the stake becomes active.
</dd>
<dt>
<code>principal: <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/sui.md#0x2_sui_SUI">sui::SUI</a>&gt;</code>
</dt>
<dd>
 The staked SUI tokens.
</dd>
<dt>
<code>expire_timestamp_ms: u64</code>
</dt>
<dd>
 This is the epoch time stamp of when the lock expires.
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x3_timelocked_staked_sui_EIncompatibleStakedSui"></a>



<pre><code><b>const</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_EIncompatibleStakedSui">EIncompatibleStakedSui</a>: u64 = 12;
</code></pre>



<a name="0x3_timelocked_staked_sui_EInsufficientSuiTokenBalance"></a>



<pre><code><b>const</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_EInsufficientSuiTokenBalance">EInsufficientSuiTokenBalance</a>: u64 = 3;
</code></pre>



<a name="0x3_timelocked_staked_sui_EStakedSuiBelowThreshold"></a>



<pre><code><b>const</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_EStakedSuiBelowThreshold">EStakedSuiBelowThreshold</a>: u64 = 18;
</code></pre>



<a name="0x3_timelocked_staked_sui_MIN_STAKING_THRESHOLD"></a>

TimelockedStakedSui objects cannot be split to below this amount.


<pre><code><b>const</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>: u64 = 1000000000;
</code></pre>



<a name="0x3_timelocked_staked_sui_create"></a>

## Function `create`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_create">create</a>(pool_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, stake_activation_epoch: u64, principal: <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/sui.md#0x2_sui_SUI">sui::SUI</a>&gt;, expire_timestamp_ms: u64, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">timelocked_staked_sui::TimelockedStakedSui</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_create">create</a>(
    pool_id: ID,
    stake_activation_epoch: u64,
    principal: Balance&lt;SUI&gt;,
    expire_timestamp_ms: u64,
    ctx: &<b>mut</b> TxContext
): <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">TimelockedStakedSui</a> {
    <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">TimelockedStakedSui</a> {
        id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
        pool_id,
        stake_activation_epoch,
        principal,
        expire_timestamp_ms
    }
}
</code></pre>



</details>

<a name="0x3_timelocked_staked_sui_unwrap_timelocked_staked_sui"></a>

## Function `unwrap_timelocked_staked_sui`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_unwrap_timelocked_staked_sui">unwrap_timelocked_staked_sui</a>(staked_sui: <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">timelocked_staked_sui::TimelockedStakedSui</a>): (<a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>, u64, <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/sui.md#0x2_sui_SUI">sui::SUI</a>&gt;, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_unwrap_timelocked_staked_sui">unwrap_timelocked_staked_sui</a>(staked_sui: <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">TimelockedStakedSui</a>): (ID, u64, Balance&lt;SUI&gt;, u64) {
    <b>let</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">TimelockedStakedSui</a> {
        id,
        pool_id,
        stake_activation_epoch,
        principal,
        expire_timestamp_ms,
    } = staked_sui;
    <a href="../sui-framework/object.md#0x2_object_delete">object::delete</a>(id);
    (pool_id, stake_activation_epoch, principal, expire_timestamp_ms)
}
</code></pre>



</details>

<a name="0x3_timelocked_staked_sui_pool_id"></a>

## Function `pool_id`



<pre><code><b>public</b> <b>fun</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_pool_id">pool_id</a>(staked_sui: &<a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">timelocked_staked_sui::TimelockedStakedSui</a>): <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_pool_id">pool_id</a>(staked_sui: &<a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">TimelockedStakedSui</a>): ID { staked_sui.pool_id }
</code></pre>



</details>

<a name="0x3_timelocked_staked_sui_staked_sui_amount"></a>

## Function `staked_sui_amount`



<pre><code><b>public</b> <b>fun</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_staked_sui_amount">staked_sui_amount</a>(staked_sui: &<a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">timelocked_staked_sui::TimelockedStakedSui</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_staked_sui_amount">staked_sui_amount</a>(staked_sui: &<a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">TimelockedStakedSui</a>): u64 { staked_sui.principal.value() }
</code></pre>



</details>

<a name="0x3_timelocked_staked_sui_stake_activation_epoch"></a>

## Function `stake_activation_epoch`



<pre><code><b>public</b> <b>fun</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_stake_activation_epoch">stake_activation_epoch</a>(staked_sui: &<a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">timelocked_staked_sui::TimelockedStakedSui</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_stake_activation_epoch">stake_activation_epoch</a>(staked_sui: &<a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">TimelockedStakedSui</a>): u64 {
    staked_sui.stake_activation_epoch
}
</code></pre>



</details>

<a name="0x3_timelocked_staked_sui_expire_timestamp_ms"></a>

## Function `expire_timestamp_ms`



<pre><code><b>public</b> <b>fun</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_expire_timestamp_ms">expire_timestamp_ms</a>(staked_sui: &<a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">timelocked_staked_sui::TimelockedStakedSui</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_expire_timestamp_ms">expire_timestamp_ms</a>(staked_sui: &<a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">TimelockedStakedSui</a>): u64 {
    staked_sui.expire_timestamp_ms
}
</code></pre>



</details>

<a name="0x3_timelocked_staked_sui_split"></a>

## Function `split`

Split TimelockedStakedSui <code>self</code> to two parts, one with principal <code>split_amount</code>,
and the remaining principal is left in <code>self</code>.
All the other parameters of the TimelockedStakedSui like <code>stake_activation_epoch</code> or <code>pool_id</code> remain the same.


<pre><code><b>public</b> <b>fun</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_split">split</a>(self: &<b>mut</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">timelocked_staked_sui::TimelockedStakedSui</a>, split_amount: u64, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">timelocked_staked_sui::TimelockedStakedSui</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_split">split</a>(self: &<b>mut</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">TimelockedStakedSui</a>, split_amount: u64, ctx: &<b>mut</b> TxContext): <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">TimelockedStakedSui</a> {
    <b>let</b> original_amount = self.principal.value();
    <b>assert</b>!(split_amount &lt;= original_amount, <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_EInsufficientSuiTokenBalance">EInsufficientSuiTokenBalance</a>);
    <b>let</b> remaining_amount = original_amount - split_amount;
    // Both resulting parts should have at least <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>.
    <b>assert</b>!(remaining_amount &gt;= <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>, <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_EStakedSuiBelowThreshold">EStakedSuiBelowThreshold</a>);
    <b>assert</b>!(split_amount &gt;= <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>, <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_EStakedSuiBelowThreshold">EStakedSuiBelowThreshold</a>);
    <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">TimelockedStakedSui</a> {
        id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
        pool_id: self.pool_id,
        stake_activation_epoch: self.stake_activation_epoch,
        principal: self.principal.<a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_split">split</a>(split_amount),
        expire_timestamp_ms: self.expire_timestamp_ms,
    }
}
</code></pre>



</details>

<a name="0x3_timelocked_staked_sui_split_staked_sui"></a>

## Function `split_staked_sui`

Split the given TimelockedStakedSui to the two parts, one with principal <code>split_amount</code>,
transfer the newly split part to the sender address.


<pre><code><b>public</b> entry <b>fun</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_split_staked_sui">split_staked_sui</a>(stake: &<b>mut</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">timelocked_staked_sui::TimelockedStakedSui</a>, split_amount: u64, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_split_staked_sui">split_staked_sui</a>(stake: &<b>mut</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">TimelockedStakedSui</a>, split_amount: u64, ctx: &<b>mut</b> TxContext) {
    <a href="../sui-framework/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(<a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_split">split</a>(stake, split_amount, ctx), ctx.sender());
}
</code></pre>



</details>

<a name="0x3_timelocked_staked_sui_join_staked_sui"></a>

## Function `join_staked_sui`

Consume the staked sui <code>other</code> and add its value to <code>self</code>.
Aborts if some of the staking parameters are incompatible (pool id, stake activation epoch, etc.)


<pre><code><b>public</b> entry <b>fun</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_join_staked_sui">join_staked_sui</a>(self: &<b>mut</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">timelocked_staked_sui::TimelockedStakedSui</a>, other: <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">timelocked_staked_sui::TimelockedStakedSui</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_join_staked_sui">join_staked_sui</a>(self: &<b>mut</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">TimelockedStakedSui</a>, other: <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">TimelockedStakedSui</a>) {
    <b>assert</b>!(<a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_is_equal_staking_metadata">is_equal_staking_metadata</a>(self, &other), <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_EIncompatibleStakedSui">EIncompatibleStakedSui</a>);
    <b>let</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">TimelockedStakedSui</a> {
        id,
        pool_id: _,
        stake_activation_epoch: _,
        principal,
        expire_timestamp_ms: _,
    } = other;

    id.delete();
    self.principal.join(principal);
}
</code></pre>



</details>

<a name="0x3_timelocked_staked_sui_is_equal_staking_metadata"></a>

## Function `is_equal_staking_metadata`

Returns true if all the staking parameters of the staked sui except the principal are identical


<pre><code><b>public</b> <b>fun</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_is_equal_staking_metadata">is_equal_staking_metadata</a>(self: &<a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">timelocked_staked_sui::TimelockedStakedSui</a>, other: &<a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">timelocked_staked_sui::TimelockedStakedSui</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_is_equal_staking_metadata">is_equal_staking_metadata</a>(self: &<a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">TimelockedStakedSui</a>, other: &<a href="timelocked_staked_sui.md#0x3_timelocked_staked_sui_TimelockedStakedSui">TimelockedStakedSui</a>): bool {
    (self.pool_id == other.pool_id) &&
    (self.stake_activation_epoch == other.stake_activation_epoch) &&
    (self.expire_timestamp_ms == other.expire_timestamp_ms)
}
</code></pre>



</details>
