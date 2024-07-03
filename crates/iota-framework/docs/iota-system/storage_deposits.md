---
title: Module `0x3::storage_deposits`
---



-  [Struct `StorageDeposits`](#0x3_storage_deposits_StorageDeposits)
-  [Function `new`](#0x3_storage_deposits_new)
-  [Function `advance_epoch`](#0x3_storage_deposits_advance_epoch)
-  [Function `total_balance`](#0x3_storage_deposits_total_balance)


<pre><code><b>use</b> <a href="../iota-framework/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../iota-framework/iota.md#0x2_iota">0x2::iota</a>;
</code></pre>



<a name="0x3_storage_deposits_StorageDeposits"></a>

## Struct `StorageDeposits`

Struct representing the storage deposits, containing one <code>Balance</code>:
- <code>storage_balance</code> has the invariant that it's the sum of <code>storage_rebate</code> of
all objects currently stored on-chain. To maintain this invariant, the only inflow of this
balance is storage charges collected from transactions, and the only outflow is storage rebates
of transactions.


<pre><code><b>struct</b> <a href="storage_deposits.md#0x3_storage_deposits_StorageDeposits">StorageDeposits</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>storage_balance: <a href="../iota-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../iota-framework/iota.md#0x2_iota_IOTA">iota::IOTA</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_storage_deposits_new"></a>

## Function `new`

Called by <code><a href="iota_system.md#0x3_iota_system">iota_system</a></code> at genesis time.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="storage_deposits.md#0x3_storage_deposits_new">new</a>(initial_balance: <a href="../iota-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../iota-framework/iota.md#0x2_iota_IOTA">iota::IOTA</a>&gt;): <a href="storage_deposits.md#0x3_storage_deposits_StorageDeposits">storage_deposits::StorageDeposits</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="storage_deposits.md#0x3_storage_deposits_new">new</a>(initial_balance: Balance&lt;IOTA&gt;) : <a href="storage_deposits.md#0x3_storage_deposits_StorageDeposits">StorageDeposits</a> {
    <a href="storage_deposits.md#0x3_storage_deposits_StorageDeposits">StorageDeposits</a> {
        // At the beginning there's no <a href="../iota-framework/object.md#0x2_object">object</a> in the storage yet
        storage_balance: initial_balance,
    }
}
</code></pre>



</details>

<a name="0x3_storage_deposits_advance_epoch"></a>

## Function `advance_epoch`

Called by <code><a href="iota_system.md#0x3_iota_system">iota_system</a></code> at epoch change times to process the inflows and outflows of storage deposits.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="storage_deposits.md#0x3_storage_deposits_advance_epoch">advance_epoch</a>(self: &<b>mut</b> <a href="storage_deposits.md#0x3_storage_deposits_StorageDeposits">storage_deposits::StorageDeposits</a>, storage_charges: <a href="../iota-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../iota-framework/iota.md#0x2_iota_IOTA">iota::IOTA</a>&gt;, storage_rebate_amount: u64, _non_refundable_storage_fee_amount: u64): <a href="../iota-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../iota-framework/iota.md#0x2_iota_IOTA">iota::IOTA</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="storage_deposits.md#0x3_storage_deposits_advance_epoch">advance_epoch</a>(
    self: &<b>mut</b> <a href="storage_deposits.md#0x3_storage_deposits_StorageDeposits">StorageDeposits</a>,
    storage_charges: Balance&lt;IOTA&gt;,
    storage_rebate_amount: u64,
    //TODO: try the way <b>to</b> configure
    _non_refundable_storage_fee_amount: u64,
) : Balance&lt;IOTA&gt; {
    // The storage charges for the epoch come from the storage rebate of the new objects created
    // and the new storage rebates of the objects modified during the epoch so we put the charges
    // into `storage_balance`.
    self.storage_balance.join(storage_charges);

    // `storage_rebates` <b>include</b> the already refunded rebates of deleted objects and <b>old</b> rebates of modified objects and
    // should be taken out of the `storage_balance`.
    <b>let</b> storage_rebate = self.storage_balance.split(storage_rebate_amount);

    // The storage rebate <b>has</b> already been returned <b>to</b> individual transaction senders' gas coins
    // so we <b>return</b> the <a href="../iota-framework/balance.md#0x2_balance">balance</a> <b>to</b> be burnt at the very end of epoch change.
    storage_rebate
}
</code></pre>



</details>

<a name="0x3_storage_deposits_total_balance"></a>

## Function `total_balance`



<pre><code><b>public</b> <b>fun</b> <a href="storage_deposits.md#0x3_storage_deposits_total_balance">total_balance</a>(self: &<a href="storage_deposits.md#0x3_storage_deposits_StorageDeposits">storage_deposits::StorageDeposits</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="storage_deposits.md#0x3_storage_deposits_total_balance">total_balance</a>(self: &<a href="storage_deposits.md#0x3_storage_deposits_StorageDeposits">StorageDeposits</a>): u64 {
    self.storage_balance.value()
}
</code></pre>



</details>
