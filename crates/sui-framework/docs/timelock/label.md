---
title: Module `0x10cf::label`
---

A library provides implementation for working with labels.
Any object which implements the <code>key</code> ability can be tagged with labels.


-  [Constants](#@Constants_0)
-  [Function `add`](#0x10cf_label_add)
-  [Function `add_system`](#0x10cf_label_add_system)
-  [Function `remove`](#0x10cf_label_remove)
-  [Function `remove_system`](#0x10cf_label_remove_system)
-  [Function `has`](#0x10cf_label_has)
-  [Function `has_system`](#0x10cf_label_has_system)
-  [Function `has_any`](#0x10cf_label_has_any)
-  [Function `add_impl`](#0x10cf_label_add_impl)
-  [Function `remove_impl`](#0x10cf_label_remove_impl)
-  [Function `has_impl`](#0x10cf_label_has_impl)


<pre><code><b>use</b> <a href="../move-stdlib/string.md#0x1_string">0x1::string</a>;
<b>use</b> <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field">0x2::dynamic_field</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../sui-framework/vec_set.md#0x2_vec_set">0x2::vec_set</a>;
</code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="0x10cf_label_ENotSystemAddress"></a>

Sender is not @0x0 the system address.


<pre><code><b>const</b> <a href="label.md#0x10cf_label_ENotSystemAddress">ENotSystemAddress</a>: u64 = 0;
</code></pre>



<a name="0x10cf_label_LABELS_NAME"></a>

The user-defined custom labels dynamic field name.


<pre><code><b>const</b> <a href="label.md#0x10cf_label_LABELS_NAME">LABELS_NAME</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [108, 97, 98, 101, 108, 115];
</code></pre>



<a name="0x10cf_label_SYSTEM_LABELS_NAME"></a>

The system-defined labels dynamic field name.


<pre><code><b>const</b> <a href="label.md#0x10cf_label_SYSTEM_LABELS_NAME">SYSTEM_LABELS_NAME</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [115, 121, 115, 116, 101, 109, 95, 108, 97, 98, 101, 108, 115];
</code></pre>



<a name="0x10cf_label_add"></a>

## Function `add`

Add a user-defined custom label.


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x10cf_label_add">add</a>(<a href="../sui-framework/object.md#0x2_object">object</a>: &<b>mut</b> <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>, <a href="label.md#0x10cf_label">label</a>: <a href="../move-stdlib/string.md#0x1_string_String">string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x10cf_label_add">add</a>(<a href="../sui-framework/object.md#0x2_object">object</a>: &<b>mut</b> UID, <a href="label.md#0x10cf_label">label</a>: String) {
    <a href="label.md#0x10cf_label_add_impl">add_impl</a>(<a href="../sui-framework/object.md#0x2_object">object</a>, <a href="label.md#0x10cf_label_LABELS_NAME">LABELS_NAME</a>, <a href="label.md#0x10cf_label">label</a>);
}
</code></pre>



</details>

<a name="0x10cf_label_add_system"></a>

## Function `add_system`

Add a system-defined label.
Can by call only by a system transaction.


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x10cf_label_add_system">add_system</a>(<a href="../sui-framework/object.md#0x2_object">object</a>: &<b>mut</b> <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>, <a href="label.md#0x10cf_label">label</a>: <a href="../move-stdlib/string.md#0x1_string_String">string::String</a>, ctx: &<a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x10cf_label_add_system">add_system</a>(<a href="../sui-framework/object.md#0x2_object">object</a>: &<b>mut</b> UID, <a href="label.md#0x10cf_label">label</a>: String, ctx: &TxContext) {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="label.md#0x10cf_label_ENotSystemAddress">ENotSystemAddress</a>);

    <a href="label.md#0x10cf_label_add_impl">add_impl</a>(<a href="../sui-framework/object.md#0x2_object">object</a>, <a href="label.md#0x10cf_label_SYSTEM_LABELS_NAME">SYSTEM_LABELS_NAME</a>, <a href="label.md#0x10cf_label">label</a>);
}
</code></pre>



</details>

<a name="0x10cf_label_remove"></a>

## Function `remove`

Remove a user-defined custom label.


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x10cf_label_remove">remove</a>(<a href="../sui-framework/object.md#0x2_object">object</a>: &<b>mut</b> <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>, <a href="label.md#0x10cf_label">label</a>: &<a href="../move-stdlib/string.md#0x1_string_String">string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x10cf_label_remove">remove</a>(<a href="../sui-framework/object.md#0x2_object">object</a>: &<b>mut</b> UID, <a href="label.md#0x10cf_label">label</a>: &String) {
    <a href="label.md#0x10cf_label_remove_impl">remove_impl</a>(<a href="../sui-framework/object.md#0x2_object">object</a>, <a href="label.md#0x10cf_label_LABELS_NAME">LABELS_NAME</a>, <a href="label.md#0x10cf_label">label</a>);
}
</code></pre>



</details>

<a name="0x10cf_label_remove_system"></a>

## Function `remove_system`

Remove a system-defined label.
Can by call only by a system transaction.


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x10cf_label_remove_system">remove_system</a>(<a href="../sui-framework/object.md#0x2_object">object</a>: &<b>mut</b> <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>, <a href="label.md#0x10cf_label">label</a>: &<a href="../move-stdlib/string.md#0x1_string_String">string::String</a>, ctx: &<a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x10cf_label_remove_system">remove_system</a>(<a href="../sui-framework/object.md#0x2_object">object</a>: &<b>mut</b> UID, <a href="label.md#0x10cf_label">label</a>: &String, ctx: &TxContext) {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="label.md#0x10cf_label_ENotSystemAddress">ENotSystemAddress</a>);

    <a href="label.md#0x10cf_label_remove_impl">remove_impl</a>(<a href="../sui-framework/object.md#0x2_object">object</a>, <a href="label.md#0x10cf_label_SYSTEM_LABELS_NAME">SYSTEM_LABELS_NAME</a>, <a href="label.md#0x10cf_label">label</a>);
}
</code></pre>



</details>

<a name="0x10cf_label_has"></a>

## Function `has`

Check if an object is tagged with a user-defined custom label.


<pre><code><b>public</b> <b>fun</b> <b>has</b>(<a href="../sui-framework/object.md#0x2_object">object</a>: &<b>mut</b> <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>, <a href="label.md#0x10cf_label">label</a>: &<a href="../move-stdlib/string.md#0x1_string_String">string::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <b>has</b>(<a href="../sui-framework/object.md#0x2_object">object</a>: &<b>mut</b> UID, <a href="label.md#0x10cf_label">label</a>: &String): bool {
    <a href="label.md#0x10cf_label_has_impl">has_impl</a>(<a href="../sui-framework/object.md#0x2_object">object</a>, <a href="label.md#0x10cf_label_LABELS_NAME">LABELS_NAME</a>, <a href="label.md#0x10cf_label">label</a>)
}
</code></pre>



</details>

<a name="0x10cf_label_has_system"></a>

## Function `has_system`

Check if an object is tagged with a system-defined label.


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x10cf_label_has_system">has_system</a>(<a href="../sui-framework/object.md#0x2_object">object</a>: &<b>mut</b> <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>, <a href="label.md#0x10cf_label">label</a>: &<a href="../move-stdlib/string.md#0x1_string_String">string::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x10cf_label_has_system">has_system</a>(<a href="../sui-framework/object.md#0x2_object">object</a>: &<b>mut</b> UID, <a href="label.md#0x10cf_label">label</a>: &String): bool {
    <a href="label.md#0x10cf_label_has_impl">has_impl</a>(<a href="../sui-framework/object.md#0x2_object">object</a>, <a href="label.md#0x10cf_label_SYSTEM_LABELS_NAME">SYSTEM_LABELS_NAME</a>, <a href="label.md#0x10cf_label">label</a>)
}
</code></pre>



</details>

<a name="0x10cf_label_has_any"></a>

## Function `has_any`

Check if an object is tagged with an any label.


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x10cf_label_has_any">has_any</a>(<a href="../sui-framework/object.md#0x2_object">object</a>: &<b>mut</b> <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>, <a href="label.md#0x10cf_label">label</a>: &<a href="../move-stdlib/string.md#0x1_string_String">string::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x10cf_label_has_any">has_any</a>(<a href="../sui-framework/object.md#0x2_object">object</a>: &<b>mut</b> UID, <a href="label.md#0x10cf_label">label</a>: &String): bool {
    <a href="label.md#0x10cf_label_has_impl">has_impl</a>(<a href="../sui-framework/object.md#0x2_object">object</a>, <a href="label.md#0x10cf_label_LABELS_NAME">LABELS_NAME</a>, <a href="label.md#0x10cf_label">label</a>) || <a href="label.md#0x10cf_label_has_impl">has_impl</a>(<a href="../sui-framework/object.md#0x2_object">object</a>, <a href="label.md#0x10cf_label_SYSTEM_LABELS_NAME">SYSTEM_LABELS_NAME</a>, <a href="label.md#0x10cf_label">label</a>)
}
</code></pre>



</details>

<a name="0x10cf_label_add_impl"></a>

## Function `add_impl`

Add label internal utility function.


<pre><code><b>fun</b> <a href="label.md#0x10cf_label_add_impl">add_impl</a>(<a href="../sui-framework/object.md#0x2_object">object</a>: &<b>mut</b> <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>, df_name: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="label.md#0x10cf_label">label</a>: <a href="../move-stdlib/string.md#0x1_string_String">string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="label.md#0x10cf_label_add_impl">add_impl</a>(<a href="../sui-framework/object.md#0x2_object">object</a>: &<b>mut</b> UID, df_name: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="label.md#0x10cf_label">label</a>: String) {
    // Check <b>if</b> a labels collection exists.
    <b>if</b> (<a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_exists_">dynamic_field::exists_</a>(<a href="../sui-framework/object.md#0x2_object">object</a>, df_name)) {
        // Borrow the related labels collection.
        <b>let</b> labels = <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_borrow_mut">dynamic_field::borrow_mut</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, VecSet&lt;String&gt;&gt;(<a href="../sui-framework/object.md#0x2_object">object</a>, df_name);

        // Insert the <a href="label.md#0x10cf_label">label</a> into the collection.
        labels.insert(<a href="label.md#0x10cf_label">label</a>);
    } <b>else</b> {
        // Create a new labels collection.
        <b>let</b> <b>mut</b> labels = <a href="../sui-framework/vec_set.md#0x2_vec_set_empty">vec_set::empty</a>();

        // Insert the <a href="label.md#0x10cf_label">label</a> into the collection.
        labels.insert(<a href="label.md#0x10cf_label">label</a>);

        // Add the created collection <b>as</b> a dynamic field <b>to</b> the <a href="../sui-framework/object.md#0x2_object">object</a>.
        <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_add">dynamic_field::add</a>(<a href="../sui-framework/object.md#0x2_object">object</a>, df_name, labels);
    }
}
</code></pre>



</details>

<a name="0x10cf_label_remove_impl"></a>

## Function `remove_impl`

Remove label internal utility function.


<pre><code><b>fun</b> <a href="label.md#0x10cf_label_remove_impl">remove_impl</a>(<a href="../sui-framework/object.md#0x2_object">object</a>: &<b>mut</b> <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>, df_name: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="label.md#0x10cf_label">label</a>: &<a href="../move-stdlib/string.md#0x1_string_String">string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="label.md#0x10cf_label_remove_impl">remove_impl</a>(<a href="../sui-framework/object.md#0x2_object">object</a>: &<b>mut</b> UID, df_name: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="label.md#0x10cf_label">label</a>: &String) {
    // Need <b>to</b> check <b>if</b> this variable is required.
    <b>let</b> <b>mut</b> need_remove_collection = <b>false</b>;

    // Check <b>if</b> a labels collection exists.
    <b>if</b> (<a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_exists_">dynamic_field::exists_</a>(<a href="../sui-framework/object.md#0x2_object">object</a>, df_name)) {
        // Borrow the related labels collection.
        <b>let</b> labels = <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_borrow_mut">dynamic_field::borrow_mut</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, VecSet&lt;String&gt;&gt;(<a href="../sui-framework/object.md#0x2_object">object</a>, df_name);

        // Check <b>if</b> the labels collection contains the <a href="label.md#0x10cf_label">label</a>.
        <b>if</b> (labels.contains(<a href="label.md#0x10cf_label">label</a>)) {
            // Remove the <a href="label.md#0x10cf_label">label</a>.
            labels.<a href="label.md#0x10cf_label_remove">remove</a>(<a href="label.md#0x10cf_label">label</a>);

            // Remove the labels collection <b>if</b> it is empty.
            <b>if</b> (labels.is_empty()) {
                need_remove_collection = <b>true</b>;
            };
        };
    };

    // Remove the related labels collection.
    <b>if</b> (need_remove_collection) {
        <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_remove">dynamic_field::remove</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, VecSet&lt;String&gt;&gt;(<a href="../sui-framework/object.md#0x2_object">object</a>, df_name);
    }
}
</code></pre>



</details>

<a name="0x10cf_label_has_impl"></a>

## Function `has_impl`

Utility function for checking if an object is tagged with a label.


<pre><code><b>fun</b> <a href="label.md#0x10cf_label_has_impl">has_impl</a>(<a href="../sui-framework/object.md#0x2_object">object</a>: &<a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>, df_name: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="label.md#0x10cf_label">label</a>: &<a href="../move-stdlib/string.md#0x1_string_String">string::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="label.md#0x10cf_label_has_impl">has_impl</a>(<a href="../sui-framework/object.md#0x2_object">object</a>: &UID, df_name: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="label.md#0x10cf_label">label</a>: &String): bool {
    // The <a href="label.md#0x10cf_label">label</a> can not exist <b>if</b> there is no a <a href="label.md#0x10cf_label">label</a> collection.
    <b>if</b> (!<a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_exists_">dynamic_field::exists_</a>(<a href="../sui-framework/object.md#0x2_object">object</a>, df_name)) {
        <b>return</b> <b>false</b>
    };

    // Borrow the related labels collection.
    <b>let</b> labels = <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_borrow">dynamic_field::borrow</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, VecSet&lt;String&gt;&gt;(<a href="../sui-framework/object.md#0x2_object">object</a>, df_name);

    // Check <b>if</b> an <a href="../sui-framework/object.md#0x2_object">object</a> is tagged <b>with</b> a <a href="label.md#0x10cf_label">label</a>.
    labels.contains(<a href="label.md#0x10cf_label">label</a>)
}
</code></pre>



</details>
