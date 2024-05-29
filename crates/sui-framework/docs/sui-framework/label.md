---
title: Module `0x2::label`
---

A library provides implementation for working with labels.
Any object which implements the <code>key</code> ability can be tagged with labels.


-  [Resource `SystemLabelerCap`](#0x2_label_SystemLabelerCap)
-  [Struct `LabelsGuard`](#0x2_label_LabelsGuard)
-  [Constants](#@Constants_0)
-  [Function `assign_system_labeler_cap`](#0x2_label_assign_system_labeler_cap)
-  [Function `add`](#0x2_label_add)
-  [Function `remove`](#0x2_label_remove)
-  [Function `has`](#0x2_label_has)
-  [Function `add_system`](#0x2_label_add_system)
-  [Function `remove_system`](#0x2_label_remove_system)
-  [Function `has_system`](#0x2_label_has_system)
-  [Function `has_any`](#0x2_label_has_any)
-  [Function `borrow_labels_guard`](#0x2_label_borrow_labels_guard)
-  [Function `borrow_labels_guard_mut`](#0x2_label_borrow_labels_guard_mut)
-  [Function `remove_labels_guard`](#0x2_label_remove_labels_guard)
-  [Function `create_labels_guard`](#0x2_label_create_labels_guard)
-  [Function `ensure_labels_guard_is_created`](#0x2_label_ensure_labels_guard_is_created)


<pre><code><b>use</b> <a href="dynamic_field.md#0x2_dynamic_field">0x2::dynamic_field</a>;
<b>use</b> <a href="object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="vec_set.md#0x2_vec_set">0x2::vec_set</a>;
</code></pre>



<a name="0x2_label_SystemLabelerCap"></a>

## Resource `SystemLabelerCap`

The capability allows to work with system labels.


<pre><code><b>struct</b> <a href="label.md#0x2_label_SystemLabelerCap">SystemLabelerCap</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x2_label_LabelsGuard"></a>

## Struct `LabelsGuard`

<code><a href="label.md#0x2_label_LabelsGuard">LabelsGuard</a></code> protects the labels from unauthorized access.


<pre><code><b>struct</b> <a href="label.md#0x2_label_LabelsGuard">LabelsGuard</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>labels: <a href="vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>
 User-defined labels.
</dd>
<dt>
<code>system_labels: <a href="vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>
 Protected system-defined labels.
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x2_label_ENotSystemAddress"></a>

Sender is not @0x0 the system address.


<pre><code><b>const</b> <a href="label.md#0x2_label_ENotSystemAddress">ENotSystemAddress</a>: u64 = 0;
</code></pre>



<a name="0x2_label_LABELS_NAME"></a>

The <code><a href="label.md#0x2_label_LabelsGuard">LabelsGuard</a></code> dynamic field name.


<pre><code><b>const</b> <a href="label.md#0x2_label_LABELS_NAME">LABELS_NAME</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [108, 97, 98, 101, 108, 115];
</code></pre>



<a name="0x2_label_assign_system_labeler_cap"></a>

## Function `assign_system_labeler_cap`

Create and transfer a <code><a href="label.md#0x2_label_SystemLabelerCap">SystemLabelerCap</a></code> object to an address.
This function is called exactly once, during genesis.


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x2_label_assign_system_labeler_cap">assign_system_labeler_cap</a>(<b>to</b>: <b>address</b>, ctx: &<b>mut</b> <a href="tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x2_label_assign_system_labeler_cap">assign_system_labeler_cap</a>(<b>to</b>: <b>address</b>, ctx: &<b>mut</b> TxContext) {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="label.md#0x2_label_ENotSystemAddress">ENotSystemAddress</a>);

    // Create a new capability.
    <b>let</b> cap = <a href="label.md#0x2_label_SystemLabelerCap">SystemLabelerCap</a> {
        id: <a href="object.md#0x2_object_new">object::new</a>(ctx),
    };

    // Transfer the capability <b>to</b> the specified <b>address</b>.
    <a href="transfer.md#0x2_transfer_transfer">transfer::transfer</a>(cap, <b>to</b>);
}
</code></pre>



</details>

<a name="0x2_label_add"></a>

## Function `add`

Add a user-defined custom label.


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x2_label_add">add</a>(self: &<b>mut</b> <a href="label.md#0x2_label_LabelsGuard">label::LabelsGuard</a>, <a href="label.md#0x2_label">label</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x2_label_add">add</a>(self: &<b>mut</b> <a href="label.md#0x2_label_LabelsGuard">LabelsGuard</a>, <a href="label.md#0x2_label">label</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;) {
    self.labels.insert(<a href="label.md#0x2_label">label</a>);
}
</code></pre>



</details>

<a name="0x2_label_remove"></a>

## Function `remove`

Remove a user-defined custom label.


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x2_label_remove">remove</a>(self: &<b>mut</b> <a href="label.md#0x2_label_LabelsGuard">label::LabelsGuard</a>, <a href="label.md#0x2_label">label</a>: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x2_label_remove">remove</a>(self: &<b>mut</b> <a href="label.md#0x2_label_LabelsGuard">LabelsGuard</a>, <a href="label.md#0x2_label">label</a>: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;) {
    self.labels.<a href="label.md#0x2_label_remove">remove</a>(<a href="label.md#0x2_label">label</a>);
}
</code></pre>



</details>

<a name="0x2_label_has"></a>

## Function `has`

Check if an object is labeled with a user-defined custom label.


<pre><code><b>public</b> <b>fun</b> <b>has</b>(self: &<a href="label.md#0x2_label_LabelsGuard">label::LabelsGuard</a>, <a href="label.md#0x2_label">label</a>: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <b>has</b>(self: &<a href="label.md#0x2_label_LabelsGuard">LabelsGuard</a>, <a href="label.md#0x2_label">label</a>: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): bool {
    self.labels.contains(<a href="label.md#0x2_label">label</a>)
}
</code></pre>



</details>

<a name="0x2_label_add_system"></a>

## Function `add_system`

Add a system-defined label.
Can by call only by a <code><a href="label.md#0x2_label_SystemLabelerCap">SystemLabelerCap</a></code> owner.


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x2_label_add_system">add_system</a>(self: &<b>mut</b> <a href="label.md#0x2_label_LabelsGuard">label::LabelsGuard</a>, _: &<a href="label.md#0x2_label_SystemLabelerCap">label::SystemLabelerCap</a>, <a href="label.md#0x2_label">label</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x2_label_add_system">add_system</a>(self: &<b>mut</b> <a href="label.md#0x2_label_LabelsGuard">LabelsGuard</a>, _: &<a href="label.md#0x2_label_SystemLabelerCap">SystemLabelerCap</a>, <a href="label.md#0x2_label">label</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;) {
    self.system_labels.insert(<a href="label.md#0x2_label">label</a>);
}
</code></pre>



</details>

<a name="0x2_label_remove_system"></a>

## Function `remove_system`

Remove a system-defined label.
Can by call only by a <code><a href="label.md#0x2_label_SystemLabelerCap">SystemLabelerCap</a></code> owner.


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x2_label_remove_system">remove_system</a>(self: &<b>mut</b> <a href="label.md#0x2_label_LabelsGuard">label::LabelsGuard</a>, _: &<a href="label.md#0x2_label_SystemLabelerCap">label::SystemLabelerCap</a>, <a href="label.md#0x2_label">label</a>: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x2_label_remove_system">remove_system</a>(self: &<b>mut</b> <a href="label.md#0x2_label_LabelsGuard">LabelsGuard</a>, _: &<a href="label.md#0x2_label_SystemLabelerCap">SystemLabelerCap</a>, <a href="label.md#0x2_label">label</a>: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;) {
    self.system_labels.<a href="label.md#0x2_label_remove">remove</a>(<a href="label.md#0x2_label">label</a>);
}
</code></pre>



</details>

<a name="0x2_label_has_system"></a>

## Function `has_system`

Check if an object is labeled with a system-defined label.


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x2_label_has_system">has_system</a>(self: &<a href="label.md#0x2_label_LabelsGuard">label::LabelsGuard</a>, <a href="label.md#0x2_label">label</a>: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x2_label_has_system">has_system</a>(self: &<a href="label.md#0x2_label_LabelsGuard">LabelsGuard</a>, <a href="label.md#0x2_label">label</a>: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): bool {
    self.system_labels.contains(<a href="label.md#0x2_label">label</a>)
}
</code></pre>



</details>

<a name="0x2_label_has_any"></a>

## Function `has_any`

Check if an object is labeled with an any label.


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x2_label_has_any">has_any</a>(self: &<a href="label.md#0x2_label_LabelsGuard">label::LabelsGuard</a>, <a href="label.md#0x2_label">label</a>: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x2_label_has_any">has_any</a>(self: &<a href="label.md#0x2_label_LabelsGuard">LabelsGuard</a>, <a href="label.md#0x2_label">label</a>: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): bool {
    <b>has</b>(self, <a href="label.md#0x2_label">label</a>) || <a href="label.md#0x2_label_has_system">has_system</a>(self, <a href="label.md#0x2_label">label</a>)
}
</code></pre>



</details>

<a name="0x2_label_borrow_labels_guard"></a>

## Function `borrow_labels_guard`

Immutably borrow the related labels guard.
A <code><a href="label.md#0x2_label_LabelsGuard">LabelsGuard</a></code> instance will be created if it does not exist.


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x2_label_borrow_labels_guard">borrow_labels_guard</a>(<a href="object.md#0x2_object">object</a>: &<b>mut</b> <a href="object.md#0x2_object_UID">object::UID</a>): &<a href="label.md#0x2_label_LabelsGuard">label::LabelsGuard</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x2_label_borrow_labels_guard">borrow_labels_guard</a>(<a href="object.md#0x2_object">object</a>: &<b>mut</b> UID): &<a href="label.md#0x2_label_LabelsGuard">LabelsGuard</a> {
    <a href="label.md#0x2_label_ensure_labels_guard_is_created">ensure_labels_guard_is_created</a>(<a href="object.md#0x2_object">object</a>);

    <a href="dynamic_field.md#0x2_dynamic_field_borrow">dynamic_field::borrow</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="label.md#0x2_label_LabelsGuard">LabelsGuard</a>&gt;(<a href="object.md#0x2_object">object</a>, <a href="label.md#0x2_label_LABELS_NAME">LABELS_NAME</a>)
}
</code></pre>



</details>

<a name="0x2_label_borrow_labels_guard_mut"></a>

## Function `borrow_labels_guard_mut`

Mutably borrow the related labels guard.
A <code><a href="label.md#0x2_label_LabelsGuard">LabelsGuard</a></code> instance will be created if it does not exist.


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x2_label_borrow_labels_guard_mut">borrow_labels_guard_mut</a>(<a href="object.md#0x2_object">object</a>: &<b>mut</b> <a href="object.md#0x2_object_UID">object::UID</a>): &<b>mut</b> <a href="label.md#0x2_label_LabelsGuard">label::LabelsGuard</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x2_label_borrow_labels_guard_mut">borrow_labels_guard_mut</a>(<a href="object.md#0x2_object">object</a>: &<b>mut</b> UID): &<b>mut</b> <a href="label.md#0x2_label_LabelsGuard">LabelsGuard</a> {
    <a href="label.md#0x2_label_ensure_labels_guard_is_created">ensure_labels_guard_is_created</a>(<a href="object.md#0x2_object">object</a>);

    <a href="dynamic_field.md#0x2_dynamic_field_borrow_mut">dynamic_field::borrow_mut</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="label.md#0x2_label_LabelsGuard">LabelsGuard</a>&gt;(<a href="object.md#0x2_object">object</a>, <a href="label.md#0x2_label_LABELS_NAME">LABELS_NAME</a>)
}
</code></pre>



</details>

<a name="0x2_label_remove_labels_guard"></a>

## Function `remove_labels_guard`

Remove the related <code><a href="label.md#0x2_label_LabelsGuard">LabelsGuard</a></code> if it exists.
Must be called when the owned object is deleted to avoid memory leaks.


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x2_label_remove_labels_guard">remove_labels_guard</a>(<a href="object.md#0x2_object">object</a>: &<b>mut</b> <a href="object.md#0x2_object_UID">object::UID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x2_label_remove_labels_guard">remove_labels_guard</a>(<a href="object.md#0x2_object">object</a>: &<b>mut</b> UID) {
    <b>if</b> (<a href="dynamic_field.md#0x2_dynamic_field_exists_">dynamic_field::exists_</a>(<a href="object.md#0x2_object">object</a>, <a href="label.md#0x2_label_LABELS_NAME">LABELS_NAME</a>)) {
        <b>let</b> <a href="label.md#0x2_label_LabelsGuard">LabelsGuard</a> {
            labels: _,
            system_labels: _,
        } = <a href="dynamic_field.md#0x2_dynamic_field_remove">dynamic_field::remove</a>(<a href="object.md#0x2_object">object</a>, <a href="label.md#0x2_label_LABELS_NAME">LABELS_NAME</a>);
    };
}
</code></pre>



</details>

<a name="0x2_label_create_labels_guard"></a>

## Function `create_labels_guard`

Create a <code><a href="label.md#0x2_label_LabelsGuard">LabelsGuard</a></code> instance.


<pre><code><b>fun</b> <a href="label.md#0x2_label_create_labels_guard">create_labels_guard</a>(): <a href="label.md#0x2_label_LabelsGuard">label::LabelsGuard</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="label.md#0x2_label_create_labels_guard">create_labels_guard</a>(): <a href="label.md#0x2_label_LabelsGuard">LabelsGuard</a> {
    <a href="label.md#0x2_label_LabelsGuard">LabelsGuard</a> {
        labels: <a href="vec_set.md#0x2_vec_set_empty">vec_set::empty</a>(),
        system_labels: <a href="vec_set.md#0x2_vec_set_empty">vec_set::empty</a>(),
    }
}
</code></pre>



</details>

<a name="0x2_label_ensure_labels_guard_is_created"></a>

## Function `ensure_labels_guard_is_created`

Create a related <code><a href="label.md#0x2_label_LabelsGuard">LabelsGuard</a></code> instance if it does not exist.


<pre><code><b>fun</b> <a href="label.md#0x2_label_ensure_labels_guard_is_created">ensure_labels_guard_is_created</a>(<a href="object.md#0x2_object">object</a>: &<b>mut</b> <a href="object.md#0x2_object_UID">object::UID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="label.md#0x2_label_ensure_labels_guard_is_created">ensure_labels_guard_is_created</a>(<a href="object.md#0x2_object">object</a>: &<b>mut</b> UID) {
    <b>if</b> (!<a href="dynamic_field.md#0x2_dynamic_field_exists_">dynamic_field::exists_</a>(<a href="object.md#0x2_object">object</a>, <a href="label.md#0x2_label_LABELS_NAME">LABELS_NAME</a>)) {
        <a href="dynamic_field.md#0x2_dynamic_field_add">dynamic_field::add</a>(<a href="object.md#0x2_object">object</a>, <a href="label.md#0x2_label_LABELS_NAME">LABELS_NAME</a>, <a href="label.md#0x2_label_create_labels_guard">create_labels_guard</a>());
    };
}
</code></pre>



</details>
