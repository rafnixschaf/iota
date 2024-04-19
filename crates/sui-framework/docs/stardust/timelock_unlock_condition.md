---
title: Module `0x107a::timelock_unlock_condition`
---



-  [Struct `TimelockUnlockCondition`](#0x107a_timelock_unlock_condition_TimelockUnlockCondition)
-  [Constants](#@Constants_0)
-  [Function `unlock`](#0x107a_timelock_unlock_condition_unlock)
-  [Function `is_timelocked`](#0x107a_timelock_unlock_condition_is_timelocked)


<pre><code><b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
</code></pre>



<a name="0x107a_timelock_unlock_condition_TimelockUnlockCondition"></a>

## Struct `TimelockUnlockCondition`

The Stardust timelock unlock condition.


<pre><code><b>struct</b> <a href="timelock_unlock_condition.md#0x107a_timelock_unlock_condition_TimelockUnlockCondition">TimelockUnlockCondition</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>unix_time: u32</code>
</dt>
<dd>
 The unix time (seconds since Unix epoch) starting from which the output can be consumed.
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x107a_timelock_unlock_condition_ETimelockNotExpired"></a>

The timelock is not expired error.


<pre><code><b>const</b> <a href="timelock_unlock_condition.md#0x107a_timelock_unlock_condition_ETimelockNotExpired">ETimelockNotExpired</a>: u64 = 0;
</code></pre>



<a name="0x107a_timelock_unlock_condition_unlock"></a>

## Function `unlock`

Check the unlock condition.


<pre><code><b>public</b> <b>fun</b> <a href="timelock_unlock_condition.md#0x107a_timelock_unlock_condition_unlock">unlock</a>(condition: &<a href="timelock_unlock_condition.md#0x107a_timelock_unlock_condition_TimelockUnlockCondition">timelock_unlock_condition::TimelockUnlockCondition</a>, ctx: &<a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="timelock_unlock_condition.md#0x107a_timelock_unlock_condition_unlock">unlock</a>(condition: &<a href="timelock_unlock_condition.md#0x107a_timelock_unlock_condition_TimelockUnlockCondition">TimelockUnlockCondition</a>, ctx: &TxContext) {
    <b>assert</b>!(!<a href="timelock_unlock_condition.md#0x107a_timelock_unlock_condition_is_timelocked">is_timelocked</a>(condition, ctx), <a href="timelock_unlock_condition.md#0x107a_timelock_unlock_condition_ETimelockNotExpired">ETimelockNotExpired</a>);
}
</code></pre>



</details>

<a name="0x107a_timelock_unlock_condition_is_timelocked"></a>

## Function `is_timelocked`

Check if the output is locked by the <code>Timelock</code> condition.


<pre><code><b>public</b> <b>fun</b> <a href="timelock_unlock_condition.md#0x107a_timelock_unlock_condition_is_timelocked">is_timelocked</a>(condition: &<a href="timelock_unlock_condition.md#0x107a_timelock_unlock_condition_TimelockUnlockCondition">timelock_unlock_condition::TimelockUnlockCondition</a>, ctx: &<a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="timelock_unlock_condition.md#0x107a_timelock_unlock_condition_is_timelocked">is_timelocked</a>(condition: &<a href="timelock_unlock_condition.md#0x107a_timelock_unlock_condition_TimelockUnlockCondition">TimelockUnlockCondition</a>, ctx: &TxContext): bool {
    condition.unix_time &gt; ((<a href="../sui-framework/tx_context.md#0x2_tx_context_epoch_timestamp_ms">tx_context::epoch_timestamp_ms</a>(ctx) / 1000) <b>as</b> u32)
}
</code></pre>



</details>
