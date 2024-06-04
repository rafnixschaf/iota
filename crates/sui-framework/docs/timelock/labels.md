---
title: Module `0x10cf::labels`
---



-  [Resource `LabelerCap`](#0x10cf_labels_LabelerCap)
-  [Struct `Label`](#0x10cf_labels_Label)
-  [Struct `Labels`](#0x10cf_labels_Labels)
-  [Struct `LabelsBuilder`](#0x10cf_labels_LabelsBuilder)
-  [Constants](#@Constants_0)
-  [Function `create_labeler_cap`](#0x10cf_labels_create_labeler_cap)
-  [Function `destroy_labeler_cap`](#0x10cf_labels_destroy_labeler_cap)
-  [Function `from_type`](#0x10cf_labels_from_type)
-  [Function `type_name`](#0x10cf_labels_type_name)
-  [Function `is_empty`](#0x10cf_labels_is_empty)
-  [Function `contains`](#0x10cf_labels_contains)
-  [Function `destroy`](#0x10cf_labels_destroy)
-  [Function `clone`](#0x10cf_labels_clone)
-  [Function `create_builder`](#0x10cf_labels_create_builder)
-  [Function `with_label`](#0x10cf_labels_with_label)
-  [Function `into_labels`](#0x10cf_labels_into_labels)


<pre><code><b>use</b> <a href="../move-stdlib/ascii.md#0x1_ascii">0x1::ascii</a>;
<b>use</b> <a href="../move-stdlib/string.md#0x1_string">0x1::string</a>;
<b>use</b> <a href="../move-stdlib/type_name.md#0x1_type_name">0x1::type_name</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../sui-framework/types.md#0x2_types">0x2::types</a>;
<b>use</b> <a href="../sui-framework/vec_set.md#0x2_vec_set">0x2::vec_set</a>;
</code></pre>



<a name="0x10cf_labels_LabelerCap"></a>

## Resource `LabelerCap`

<code><a href="labels.md#0x10cf_labels_LabelerCap">LabelerCap</a></code> allows to insert labels of the specific type <code>T</code>.
Can be publicly transferred.


<pre><code><b>struct</b> <a href="labels.md#0x10cf_labels_LabelerCap">LabelerCap</a>&lt;T&gt; <b>has</b> store, key
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

<a name="0x10cf_labels_Label"></a>

## Struct `Label`

<code><a href="labels.md#0x10cf_labels_Label">Label</a></code> is an internal label representation.


<pre><code><b>struct</b> <a href="labels.md#0x10cf_labels_Label">Label</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>value: <a href="../move-stdlib/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>
 A fully qualified type name with the original package IDs.
</dd>
</dl>


</details>

<a name="0x10cf_labels_Labels"></a>

## Struct `Labels`

<code><a href="labels.md#0x10cf_labels_Labels">Labels</a></code> is an immutable labels set.
Protects the held labels from being changed and copied.


<pre><code><b>struct</b> <a href="labels.md#0x10cf_labels_Labels">Labels</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="labels.md#0x10cf_labels">labels</a>: <a href="../sui-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<a href="labels.md#0x10cf_labels_Label">labels::Label</a>&gt;</code>
</dt>
<dd>
 The protected labels collection.
</dd>
</dl>


</details>

<a name="0x10cf_labels_LabelsBuilder"></a>

## Struct `LabelsBuilder`

<code><a href="labels.md#0x10cf_labels_LabelsBuilder">LabelsBuilder</a></code> helps to build a <code><a href="labels.md#0x10cf_labels_Labels">Labels</a></code> instance.


<pre><code><b>struct</b> <a href="labels.md#0x10cf_labels_LabelsBuilder">LabelsBuilder</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="labels.md#0x10cf_labels">labels</a>: <a href="../sui-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<a href="labels.md#0x10cf_labels_Label">labels::Label</a>&gt;</code>
</dt>
<dd>
 The labels collection.
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x10cf_labels_ELabelAlreadyExists"></a>

Error code for when a label already exists in the collection.


<pre><code><b>const</b> <a href="labels.md#0x10cf_labels_ELabelAlreadyExists">ELabelAlreadyExists</a>: u64 = 1;
</code></pre>



<a name="0x10cf_labels_ENotOneTimeWitness"></a>

Error code for when a type passed to the <code>create_labeler_cap</code> function is not a one-time witness.


<pre><code><b>const</b> <a href="labels.md#0x10cf_labels_ENotOneTimeWitness">ENotOneTimeWitness</a>: u64 = 0;
</code></pre>



<a name="0x10cf_labels_create_labeler_cap"></a>

## Function `create_labeler_cap`

Create a <code><a href="labels.md#0x10cf_labels_LabelerCap">LabelerCap</a></code> instance.
Can be created only by consuming a one time witness.


<pre><code><b>public</b> <b>fun</b> <a href="labels.md#0x10cf_labels_create_labeler_cap">create_labeler_cap</a>&lt;T: drop&gt;(witness: T, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="labels.md#0x10cf_labels_LabelerCap">labels::LabelerCap</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="labels.md#0x10cf_labels_create_labeler_cap">create_labeler_cap</a>&lt;T: drop&gt;(witness: T, ctx: &<b>mut</b> TxContext): <a href="labels.md#0x10cf_labels_LabelerCap">LabelerCap</a>&lt;T&gt; {
    <b>assert</b>!(<a href="../sui-framework/types.md#0x2_types_is_one_time_witness">types::is_one_time_witness</a>(&witness), <a href="labels.md#0x10cf_labels_ENotOneTimeWitness">ENotOneTimeWitness</a>);

    <a href="labels.md#0x10cf_labels_LabelerCap">LabelerCap</a>&lt;T&gt; {
        id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
    }
}
</code></pre>



</details>

<a name="0x10cf_labels_destroy_labeler_cap"></a>

## Function `destroy_labeler_cap`

Delete a <code><a href="labels.md#0x10cf_labels_LabelerCap">LabelerCap</a></code> instance.
If a capability is destroyed, it is impossible to add the related labels.


<pre><code><b>public</b> <b>fun</b> <a href="labels.md#0x10cf_labels_destroy_labeler_cap">destroy_labeler_cap</a>&lt;T: drop&gt;(cap: <a href="labels.md#0x10cf_labels_LabelerCap">labels::LabelerCap</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="labels.md#0x10cf_labels_destroy_labeler_cap">destroy_labeler_cap</a>&lt;T: drop&gt;(cap: <a href="labels.md#0x10cf_labels_LabelerCap">LabelerCap</a>&lt;T&gt;) {
    <b>let</b> <a href="labels.md#0x10cf_labels_LabelerCap">LabelerCap</a>&lt;T&gt; {
        id,
    } = cap;

    <a href="../sui-framework/object.md#0x2_object_delete">object::delete</a>(id);
}
</code></pre>



</details>

<a name="0x10cf_labels_from_type"></a>

## Function `from_type`

Create a <code><a href="labels.md#0x10cf_labels_Label">Label</a></code> instance.
The created label holds a fully qualified type name with the original package IDs.


<pre><code><b>fun</b> <a href="labels.md#0x10cf_labels_from_type">from_type</a>&lt;T&gt;(): <a href="labels.md#0x10cf_labels_Label">labels::Label</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="labels.md#0x10cf_labels_from_type">from_type</a>&lt;T&gt;(): <a href="labels.md#0x10cf_labels_Label">Label</a> {
    <a href="labels.md#0x10cf_labels_Label">Label</a> {
        value: <a href="../move-stdlib/type_name.md#0x1_type_name">type_name</a>&lt;T&gt;(),
    }
}
</code></pre>



</details>

<a name="0x10cf_labels_type_name"></a>

## Function `type_name`

Return a fully qualified type name with the original package IDs.


<pre><code><b>fun</b> <a href="../move-stdlib/type_name.md#0x1_type_name">type_name</a>&lt;T&gt;(): <a href="../move-stdlib/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../move-stdlib/type_name.md#0x1_type_name">type_name</a>&lt;T&gt;(): String {
    <a href="../move-stdlib/string.md#0x1_string_from_ascii">string::from_ascii</a>(std::type_name::get_with_original_ids&lt;T&gt;().into_string())
}
</code></pre>



</details>

<a name="0x10cf_labels_is_empty"></a>

## Function `is_empty`

Return true if <code>self</code> has 0 elements, false otherwise.


<pre><code><b>public</b> <b>fun</b> <a href="labels.md#0x10cf_labels_is_empty">is_empty</a>(self: &<a href="labels.md#0x10cf_labels_Labels">labels::Labels</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="labels.md#0x10cf_labels_is_empty">is_empty</a>(self: &<a href="labels.md#0x10cf_labels_Labels">Labels</a>): bool {
    self.<a href="labels.md#0x10cf_labels">labels</a>.<a href="labels.md#0x10cf_labels_is_empty">is_empty</a>()
}
</code></pre>



</details>

<a name="0x10cf_labels_contains"></a>

## Function `contains`

Return true if <code>self</code> contains a label, false otherwise.


<pre><code><b>public</b> <b>fun</b> <a href="labels.md#0x10cf_labels_contains">contains</a>&lt;T&gt;(self: &<a href="labels.md#0x10cf_labels_Labels">labels::Labels</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="labels.md#0x10cf_labels_contains">contains</a>&lt;T&gt;(self: &<a href="labels.md#0x10cf_labels_Labels">Labels</a>): bool {
    self.<a href="labels.md#0x10cf_labels">labels</a>.<a href="labels.md#0x10cf_labels_contains">contains</a>(&<a href="labels.md#0x10cf_labels_from_type">from_type</a>&lt;T&gt;())
}
</code></pre>



</details>

<a name="0x10cf_labels_destroy"></a>

## Function `destroy`

Destroy a <code><a href="labels.md#0x10cf_labels_Labels">Labels</a></code> instance.


<pre><code><b>public</b> <b>fun</b> <a href="labels.md#0x10cf_labels_destroy">destroy</a>(self: <a href="labels.md#0x10cf_labels_Labels">labels::Labels</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="labels.md#0x10cf_labels_destroy">destroy</a>(self: <a href="labels.md#0x10cf_labels_Labels">Labels</a>) {
    <b>let</b> <a href="labels.md#0x10cf_labels_Labels">Labels</a> {
        <a href="labels.md#0x10cf_labels">labels</a>: _,
    } = self;
}
</code></pre>



</details>

<a name="0x10cf_labels_clone"></a>

## Function `clone`

Clone a <code><a href="labels.md#0x10cf_labels_Labels">Labels</a></code> instance.
It is a protected utility function, it should be impossible to clone <code><a href="labels.md#0x10cf_labels_Labels">Labels</a></code>
because it leads to unauthorized labeled objects creation.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="labels.md#0x10cf_labels_clone">clone</a>(self: &<a href="labels.md#0x10cf_labels_Labels">labels::Labels</a>): <a href="labels.md#0x10cf_labels_Labels">labels::Labels</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="labels.md#0x10cf_labels_clone">clone</a>(self: &<a href="labels.md#0x10cf_labels_Labels">Labels</a>): <a href="labels.md#0x10cf_labels_Labels">Labels</a> {
    <a href="labels.md#0x10cf_labels_Labels">Labels</a> {
        <a href="labels.md#0x10cf_labels">labels</a>: self.<a href="labels.md#0x10cf_labels">labels</a>,
    }
}
</code></pre>



</details>

<a name="0x10cf_labels_create_builder"></a>

## Function `create_builder`

Create a <code><a href="labels.md#0x10cf_labels_LabelsBuilder">LabelsBuilder</a></code> instance.


<pre><code><b>public</b> <b>fun</b> <a href="labels.md#0x10cf_labels_create_builder">create_builder</a>(): <a href="labels.md#0x10cf_labels_LabelsBuilder">labels::LabelsBuilder</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="labels.md#0x10cf_labels_create_builder">create_builder</a>(): <a href="labels.md#0x10cf_labels_LabelsBuilder">LabelsBuilder</a> {
    <a href="labels.md#0x10cf_labels_LabelsBuilder">LabelsBuilder</a> {
        <a href="labels.md#0x10cf_labels">labels</a>: <a href="../sui-framework/vec_set.md#0x2_vec_set_empty">vec_set::empty</a>(),
    }
}
</code></pre>



</details>

<a name="0x10cf_labels_with_label"></a>

## Function `with_label`

Add a label into <code>self</code>. Can be called only by the related <code><a href="labels.md#0x10cf_labels_LabelerCap">LabelerCap</a></code> owner.
Aborts if the label is already present in <code>self</code>.


<pre><code><b>public</b> <b>fun</b> <a href="labels.md#0x10cf_labels_with_label">with_label</a>&lt;T&gt;(self: <a href="labels.md#0x10cf_labels_LabelsBuilder">labels::LabelsBuilder</a>, _: &<a href="labels.md#0x10cf_labels_LabelerCap">labels::LabelerCap</a>&lt;T&gt;): <a href="labels.md#0x10cf_labels_LabelsBuilder">labels::LabelsBuilder</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="labels.md#0x10cf_labels_with_label">with_label</a>&lt;T&gt;(<b>mut</b> self: <a href="labels.md#0x10cf_labels_LabelsBuilder">LabelsBuilder</a>, _: &<a href="labels.md#0x10cf_labels_LabelerCap">LabelerCap</a>&lt;T&gt;): <a href="labels.md#0x10cf_labels_LabelsBuilder">LabelsBuilder</a> {
    <b>let</b> label = <a href="labels.md#0x10cf_labels_from_type">from_type</a>&lt;T&gt;();

    <b>assert</b>!(!self.<a href="labels.md#0x10cf_labels">labels</a>.<a href="labels.md#0x10cf_labels_contains">contains</a>(&label), <a href="labels.md#0x10cf_labels_ELabelAlreadyExists">ELabelAlreadyExists</a>);

    self.<a href="labels.md#0x10cf_labels">labels</a>.insert(label);

    self
}
</code></pre>



</details>

<a name="0x10cf_labels_into_labels"></a>

## Function `into_labels`

Transform a <code><a href="labels.md#0x10cf_labels_LabelsBuilder">LabelsBuilder</a></code> instance into <code><a href="labels.md#0x10cf_labels_Labels">Labels</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="labels.md#0x10cf_labels_into_labels">into_labels</a>(self: <a href="labels.md#0x10cf_labels_LabelsBuilder">labels::LabelsBuilder</a>): <a href="labels.md#0x10cf_labels_Labels">labels::Labels</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="labels.md#0x10cf_labels_into_labels">into_labels</a>(self: <a href="labels.md#0x10cf_labels_LabelsBuilder">LabelsBuilder</a>): <a href="labels.md#0x10cf_labels_Labels">Labels</a> {
    <b>let</b> <a href="labels.md#0x10cf_labels_LabelsBuilder">LabelsBuilder</a> {
        <a href="labels.md#0x10cf_labels">labels</a>,
    } = self;

    <a href="labels.md#0x10cf_labels_Labels">Labels</a> {
        <a href="labels.md#0x10cf_labels">labels</a>,
    }
}
</code></pre>



</details>
