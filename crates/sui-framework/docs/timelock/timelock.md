---
title: Module `0x10cf::timelock`
---

A timelock implementation.


-  [Resource `LabelerCap`](#0x10cf_timelock_LabelerCap)
-  [Resource `TimeLock`](#0x10cf_timelock_TimeLock)
-  [Constants](#@Constants_0)
-  [Function `assign_labeler_cap`](#0x10cf_timelock_assign_labeler_cap)
-  [Function `lock`](#0x10cf_timelock_lock)
-  [Function `lock_labeled`](#0x10cf_timelock_lock_labeled)
-  [Function `unlock`](#0x10cf_timelock_unlock)
-  [Function `expiration_timestamp_ms`](#0x10cf_timelock_expiration_timestamp_ms)
-  [Function `is_locked`](#0x10cf_timelock_is_locked)
-  [Function `remaining_time`](#0x10cf_timelock_remaining_time)
-  [Function `locked`](#0x10cf_timelock_locked)
-  [Function `locked_mut`](#0x10cf_timelock_locked_mut)
-  [Function `is_labeled_with`](#0x10cf_timelock_is_labeled_with)
-  [Function `labels`](#0x10cf_timelock_labels)
-  [Function `pack`](#0x10cf_timelock_pack)
-  [Function `unpack`](#0x10cf_timelock_unpack)
-  [Function `transfer`](#0x10cf_timelock_transfer)
-  [Function `check_expiration_timestamp_ms`](#0x10cf_timelock_check_expiration_timestamp_ms)


<pre><code><b>use</b> <a href="../move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../sui-framework/vec_set.md#0x2_vec_set">0x2::vec_set</a>;
</code></pre>



<a name="0x10cf_timelock_LabelerCap"></a>

## Resource `LabelerCap`

The capability allows to work with labels.


<pre><code><b>struct</b> <a href="timelock.md#0x10cf_timelock_LabelerCap">LabelerCap</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x10cf_timelock_TimeLock"></a>

## Resource `TimeLock`

<code><a href="timelock.md#0x10cf_timelock_TimeLock">TimeLock</a></code> struct that holds a locked object.


<pre><code><b>struct</b> <a href="timelock.md#0x10cf_timelock_TimeLock">TimeLock</a>&lt;T: store&gt; <b>has</b> key
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
<code>locked: T</code>
</dt>
<dd>
 The locked object.
</dd>
<dt>
<code>expiration_timestamp_ms: u64</code>
</dt>
<dd>
 This is the epoch time stamp of when the lock expires.
</dd>
<dt>
<code>labels: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../sui-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;&gt;</code>
</dt>
<dd>
 Timelock related labels.
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x10cf_timelock_ENotSystemAddress"></a>

Error code for when the sender is not @0x0, the system address.


<pre><code><b>const</b> <a href="timelock.md#0x10cf_timelock_ENotSystemAddress">ENotSystemAddress</a>: u64 = 2;
</code></pre>



<a name="0x10cf_timelock_EEmptyLabel"></a>

Error code for when the labels collection of the lock contains an empty label.


<pre><code><b>const</b> <a href="timelock.md#0x10cf_timelock_EEmptyLabel">EEmptyLabel</a>: u64 = 4;
</code></pre>



<a name="0x10cf_timelock_EEmptyLabelsCollection"></a>

Error code for when the labels collection of the lock is empty.


<pre><code><b>const</b> <a href="timelock.md#0x10cf_timelock_EEmptyLabelsCollection">EEmptyLabelsCollection</a>: u64 = 3;
</code></pre>



<a name="0x10cf_timelock_EExpireEpochIsPast"></a>

Error code for when the expire timestamp of the lock is in the past.


<pre><code><b>const</b> <a href="timelock.md#0x10cf_timelock_EExpireEpochIsPast">EExpireEpochIsPast</a>: u64 = 0;
</code></pre>



<a name="0x10cf_timelock_ENotExpiredYet"></a>

Error code for when the lock has not expired yet.


<pre><code><b>const</b> <a href="timelock.md#0x10cf_timelock_ENotExpiredYet">ENotExpiredYet</a>: u64 = 1;
</code></pre>



<a name="0x10cf_timelock_assign_labeler_cap"></a>

## Function `assign_labeler_cap`

Create and transfer a <code><a href="timelock.md#0x10cf_timelock_LabelerCap">LabelerCap</a></code> object to an authority address.
This function is called exactly once, during genesis.


<pre><code><b>public</b> <b>fun</b> <a href="timelock.md#0x10cf_timelock_assign_labeler_cap">assign_labeler_cap</a>(<b>to</b>: <b>address</b>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="timelock.md#0x10cf_timelock_assign_labeler_cap">assign_labeler_cap</a>(<b>to</b>: <b>address</b>, ctx: &<b>mut</b> TxContext) {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="timelock.md#0x10cf_timelock_ENotSystemAddress">ENotSystemAddress</a>);

    // Create a new capability.
    <b>let</b> cap = <a href="timelock.md#0x10cf_timelock_LabelerCap">LabelerCap</a> {
        id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
    };

    // Transfer the capability <b>to</b> the specified <b>address</b>.
    <a href="../sui-framework/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(cap, <b>to</b>);
}
</code></pre>



</details>

<a name="0x10cf_timelock_lock"></a>

## Function `lock`

Function to lock an object till a unix timestamp in milliseconds.


<pre><code><b>public</b> <b>fun</b> <a href="timelock.md#0x10cf_timelock_lock">lock</a>&lt;T: store&gt;(locked: T, expiration_timestamp_ms: u64, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="timelock.md#0x10cf_timelock_TimeLock">timelock::TimeLock</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="timelock.md#0x10cf_timelock_lock">lock</a>&lt;T: store&gt;(locked: T, expiration_timestamp_ms: u64, ctx: &<b>mut</b> TxContext): <a href="timelock.md#0x10cf_timelock_TimeLock">TimeLock</a>&lt;T&gt; {
    // Check that `expiration_timestamp_ms` is valid.
    <a href="timelock.md#0x10cf_timelock_check_expiration_timestamp_ms">check_expiration_timestamp_ms</a>(expiration_timestamp_ms, ctx);

    // Create a <a href="timelock.md#0x10cf_timelock">timelock</a>.
    <a href="timelock.md#0x10cf_timelock_pack">pack</a>(locked, expiration_timestamp_ms, <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(), ctx)
}
</code></pre>



</details>

<a name="0x10cf_timelock_lock_labeled"></a>

## Function `lock_labeled`

Function to lock a labeled object till a unix timestamp in milliseconds.


<pre><code><b>public</b> <b>fun</b> <a href="timelock.md#0x10cf_timelock_lock_labeled">lock_labeled</a>&lt;T: store&gt;(_: &<a href="timelock.md#0x10cf_timelock_LabelerCap">timelock::LabelerCap</a>, locked: T, expiration_timestamp_ms: u64, labels: <a href="../sui-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="timelock.md#0x10cf_timelock_TimeLock">timelock::TimeLock</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="timelock.md#0x10cf_timelock_lock_labeled">lock_labeled</a>&lt;T: store&gt;(
    _: &<a href="timelock.md#0x10cf_timelock_LabelerCap">LabelerCap</a>,
    locked: T,
    expiration_timestamp_ms: u64,
    labels: VecSet&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    ctx: &<b>mut</b> TxContext
): <a href="timelock.md#0x10cf_timelock_TimeLock">TimeLock</a>&lt;T&gt; {
    // Check that `expiration_timestamp_ms` is valid.
    <a href="timelock.md#0x10cf_timelock_check_expiration_timestamp_ms">check_expiration_timestamp_ms</a>(expiration_timestamp_ms, ctx);

    // Check that the `labels` value is valid.
    <b>assert</b>!(!labels.is_empty(), <a href="timelock.md#0x10cf_timelock_EEmptyLabelsCollection">EEmptyLabelsCollection</a>);
    <b>assert</b>!(!labels.contains(&<a href="../move-stdlib/vector.md#0x1_vector_empty">vector::empty</a>()), <a href="timelock.md#0x10cf_timelock_EEmptyLabel">EEmptyLabel</a>);

    // Create a labeled <a href="timelock.md#0x10cf_timelock">timelock</a>.
    <a href="timelock.md#0x10cf_timelock_pack">pack</a>(locked, expiration_timestamp_ms, <a href="../move-stdlib/option.md#0x1_option_some">option::some</a>(labels), ctx)
}
</code></pre>



</details>

<a name="0x10cf_timelock_unlock"></a>

## Function `unlock`

Function to unlock the object from a <code><a href="timelock.md#0x10cf_timelock_TimeLock">TimeLock</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="timelock.md#0x10cf_timelock_unlock">unlock</a>&lt;T: store&gt;(self: <a href="timelock.md#0x10cf_timelock_TimeLock">timelock::TimeLock</a>&lt;T&gt;, ctx: &<a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="timelock.md#0x10cf_timelock_unlock">unlock</a>&lt;T: store&gt;(self: <a href="timelock.md#0x10cf_timelock_TimeLock">TimeLock</a>&lt;T&gt;, ctx: &TxContext): T {
    // Unpack the <a href="timelock.md#0x10cf_timelock">timelock</a>.
    <b>let</b> (locked, expiration_timestamp_ms, labels) = <a href="timelock.md#0x10cf_timelock_unpack">unpack</a>(self);

    // Check <b>if</b> the lock <b>has</b> expired.
    <b>assert</b>!(<a href="timelock.md#0x10cf_timelock_expiration_timestamp_ms">expiration_timestamp_ms</a> &lt;= ctx.epoch_timestamp_ms(), <a href="timelock.md#0x10cf_timelock_ENotExpiredYet">ENotExpiredYet</a>);

    // Delete the labels.
    <b>if</b> (labels.is_some()) {
        <a href="../move-stdlib/option.md#0x1_option_destroy_some">option::destroy_some</a>(labels);
    }
    <b>else</b> {
        <a href="../move-stdlib/option.md#0x1_option_destroy_none">option::destroy_none</a>(labels);
    };

    locked
}
</code></pre>



</details>

<a name="0x10cf_timelock_expiration_timestamp_ms"></a>

## Function `expiration_timestamp_ms`

Function to get the expiration timestamp of a <code><a href="timelock.md#0x10cf_timelock_TimeLock">TimeLock</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="timelock.md#0x10cf_timelock_expiration_timestamp_ms">expiration_timestamp_ms</a>&lt;T: store&gt;(self: &<a href="timelock.md#0x10cf_timelock_TimeLock">timelock::TimeLock</a>&lt;T&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="timelock.md#0x10cf_timelock_expiration_timestamp_ms">expiration_timestamp_ms</a>&lt;T: store&gt;(self: &<a href="timelock.md#0x10cf_timelock_TimeLock">TimeLock</a>&lt;T&gt;): u64 {
    self.expiration_timestamp_ms
}
</code></pre>



</details>

<a name="0x10cf_timelock_is_locked"></a>

## Function `is_locked`

Function to check if a <code><a href="timelock.md#0x10cf_timelock_TimeLock">TimeLock</a></code> is locked.


<pre><code><b>public</b> <b>fun</b> <a href="timelock.md#0x10cf_timelock_is_locked">is_locked</a>&lt;T: store&gt;(self: &<a href="timelock.md#0x10cf_timelock_TimeLock">timelock::TimeLock</a>&lt;T&gt;, ctx: &<a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="timelock.md#0x10cf_timelock_is_locked">is_locked</a>&lt;T: store&gt;(self: &<a href="timelock.md#0x10cf_timelock_TimeLock">TimeLock</a>&lt;T&gt;, ctx: &TxContext): bool {
    self.<a href="timelock.md#0x10cf_timelock_remaining_time">remaining_time</a>(ctx) &gt; 0
}
</code></pre>



</details>

<a name="0x10cf_timelock_remaining_time"></a>

## Function `remaining_time`

Function to get the remaining time of a <code><a href="timelock.md#0x10cf_timelock_TimeLock">TimeLock</a></code>.
Returns 0 if the lock has expired.


<pre><code><b>public</b> <b>fun</b> <a href="timelock.md#0x10cf_timelock_remaining_time">remaining_time</a>&lt;T: store&gt;(self: &<a href="timelock.md#0x10cf_timelock_TimeLock">timelock::TimeLock</a>&lt;T&gt;, ctx: &<a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="timelock.md#0x10cf_timelock_remaining_time">remaining_time</a>&lt;T: store&gt;(self: &<a href="timelock.md#0x10cf_timelock_TimeLock">TimeLock</a>&lt;T&gt;, ctx: &TxContext): u64 {
    // Get the epoch timestamp.
    <b>let</b> current_timestamp_ms = ctx.epoch_timestamp_ms();

    // Check <b>if</b> the lock <b>has</b> expired.
    <b>if</b> (self.<a href="timelock.md#0x10cf_timelock_expiration_timestamp_ms">expiration_timestamp_ms</a> &lt; current_timestamp_ms) {
        <b>return</b> 0
    };

    // Calculate the remaining time.
    self.expiration_timestamp_ms - current_timestamp_ms
}
</code></pre>



</details>

<a name="0x10cf_timelock_locked"></a>

## Function `locked`

Function to get the locked object of a <code><a href="timelock.md#0x10cf_timelock_TimeLock">TimeLock</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="timelock.md#0x10cf_timelock_locked">locked</a>&lt;T: store&gt;(self: &<a href="timelock.md#0x10cf_timelock_TimeLock">timelock::TimeLock</a>&lt;T&gt;): &T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="timelock.md#0x10cf_timelock_locked">locked</a>&lt;T: store&gt;(self: &<a href="timelock.md#0x10cf_timelock_TimeLock">TimeLock</a>&lt;T&gt;): &T {
    &self.locked
}
</code></pre>



</details>

<a name="0x10cf_timelock_locked_mut"></a>

## Function `locked_mut`

Function to get a mutable reference to the locked object of a <code><a href="timelock.md#0x10cf_timelock_TimeLock">TimeLock</a></code>.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="timelock.md#0x10cf_timelock_locked_mut">locked_mut</a>&lt;T: store&gt;(self: &<b>mut</b> <a href="timelock.md#0x10cf_timelock_TimeLock">timelock::TimeLock</a>&lt;T&gt;): &<b>mut</b> T
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="timelock.md#0x10cf_timelock_locked_mut">locked_mut</a>&lt;T: store&gt;(self: &<b>mut</b> <a href="timelock.md#0x10cf_timelock_TimeLock">TimeLock</a>&lt;T&gt;): &<b>mut</b> T {
    &<b>mut</b> self.locked
}
</code></pre>



</details>

<a name="0x10cf_timelock_is_labeled_with"></a>

## Function `is_labeled_with`

Function to check if a <code><a href="timelock.md#0x10cf_timelock_TimeLock">TimeLock</a></code> labeled with a label.


<pre><code><b>public</b> <b>fun</b> <a href="timelock.md#0x10cf_timelock_is_labeled_with">is_labeled_with</a>&lt;T: store&gt;(self: &<a href="timelock.md#0x10cf_timelock_TimeLock">timelock::TimeLock</a>&lt;T&gt;, label: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="timelock.md#0x10cf_timelock_is_labeled_with">is_labeled_with</a>&lt;T: store&gt;(self: &<a href="timelock.md#0x10cf_timelock_TimeLock">TimeLock</a>&lt;T&gt;, label: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): bool {
    // Check <b>if</b> the labels member are initialized, <b>return</b> `<b>false</b>` <b>if</b> it is not.
    <b>if</b> (self.labels.is_some()) {
        <b>return</b> self.labels.borrow().contains(label)
    }
    <b>else</b> {
        <b>return</b> <b>false</b>
    }
}
</code></pre>



</details>

<a name="0x10cf_timelock_labels"></a>

## Function `labels`

Function to get the labels of a <code><a href="timelock.md#0x10cf_timelock_TimeLock">TimeLock</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="timelock.md#0x10cf_timelock_labels">labels</a>&lt;T: store&gt;(self: &<a href="timelock.md#0x10cf_timelock_TimeLock">timelock::TimeLock</a>&lt;T&gt;): &<a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../sui-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="timelock.md#0x10cf_timelock_labels">labels</a>&lt;T: store&gt;(self: &<a href="timelock.md#0x10cf_timelock_TimeLock">TimeLock</a>&lt;T&gt;): &Option&lt;VecSet&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;&gt; {
    &self.labels
}
</code></pre>



</details>

<a name="0x10cf_timelock_pack"></a>

## Function `pack`

An utility function to pack a <code><a href="timelock.md#0x10cf_timelock_TimeLock">TimeLock</a></code>.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="timelock.md#0x10cf_timelock_pack">pack</a>&lt;T: store&gt;(locked: T, expiration_timestamp_ms: u64, labels: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../sui-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="timelock.md#0x10cf_timelock_TimeLock">timelock::TimeLock</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="timelock.md#0x10cf_timelock_pack">pack</a>&lt;T: store&gt;(
    locked: T,
    expiration_timestamp_ms: u64,
    labels: Option&lt;VecSet&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;&gt;,
    ctx: &<b>mut</b> TxContext): <a href="timelock.md#0x10cf_timelock_TimeLock">TimeLock</a>&lt;T&gt;
{
    // Create a <a href="timelock.md#0x10cf_timelock">timelock</a>.
    <a href="timelock.md#0x10cf_timelock_TimeLock">TimeLock</a> {
        id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
        locked,
        expiration_timestamp_ms,
        labels,
    }
}
</code></pre>



</details>

<a name="0x10cf_timelock_unpack"></a>

## Function `unpack`

An utility function to unpack a <code><a href="timelock.md#0x10cf_timelock_TimeLock">TimeLock</a></code>.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="timelock.md#0x10cf_timelock_unpack">unpack</a>&lt;T: store&gt;(lock: <a href="timelock.md#0x10cf_timelock_TimeLock">timelock::TimeLock</a>&lt;T&gt;): (T, u64, <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../sui-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="timelock.md#0x10cf_timelock_unpack">unpack</a>&lt;T: store&gt;(lock: <a href="timelock.md#0x10cf_timelock_TimeLock">TimeLock</a>&lt;T&gt;): (T, u64, Option&lt;VecSet&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;&gt;) {
    // Unpack the <a href="timelock.md#0x10cf_timelock">timelock</a>.
    <b>let</b> <a href="timelock.md#0x10cf_timelock_TimeLock">TimeLock</a> {
        id,
        locked,
        expiration_timestamp_ms,
        labels,
    } = lock;

    // Delete the <a href="timelock.md#0x10cf_timelock">timelock</a>.
    <a href="../sui-framework/object.md#0x2_object_delete">object::delete</a>(id);

    (locked, expiration_timestamp_ms, labels)
}
</code></pre>



</details>

<a name="0x10cf_timelock_transfer"></a>

## Function `transfer`

An utility function to transfer a <code><a href="timelock.md#0x10cf_timelock_TimeLock">TimeLock</a></code>.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../sui-framework/transfer.md#0x2_transfer">transfer</a>&lt;T: store&gt;(lock: <a href="timelock.md#0x10cf_timelock_TimeLock">timelock::TimeLock</a>&lt;T&gt;, recipient: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../sui-framework/transfer.md#0x2_transfer">transfer</a>&lt;T: store&gt;(lock: <a href="timelock.md#0x10cf_timelock_TimeLock">TimeLock</a>&lt;T&gt;, recipient: <b>address</b>) {
    <a href="../sui-framework/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(lock, recipient);
}
</code></pre>



</details>

<a name="0x10cf_timelock_check_expiration_timestamp_ms"></a>

## Function `check_expiration_timestamp_ms`

An utility function to check that the <code>expiration_timestamp_ms</code> value is valid.


<pre><code><b>fun</b> <a href="timelock.md#0x10cf_timelock_check_expiration_timestamp_ms">check_expiration_timestamp_ms</a>(expiration_timestamp_ms: u64, ctx: &<a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="timelock.md#0x10cf_timelock_check_expiration_timestamp_ms">check_expiration_timestamp_ms</a>(expiration_timestamp_ms: u64, ctx: &TxContext) {
    // Get the epoch timestamp.
    <b>let</b> epoch_timestamp_ms = ctx.epoch_timestamp_ms();

    // Check that `expiration_timestamp_ms` is valid.
    <b>assert</b>!(expiration_timestamp_ms &gt; epoch_timestamp_ms, <a href="timelock.md#0x10cf_timelock_EExpireEpochIsPast">EExpireEpochIsPast</a>);
}
</code></pre>



</details>
