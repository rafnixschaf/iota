---
title: Module `0x10cf::label`
---



-  [Resource `LabelerCap`](#0x10cf_label_LabelerCap)
-  [Struct `Label`](#0x10cf_label_Label)
-  [Constants](#@Constants_0)
-  [Function `create_labeler_cap`](#0x10cf_label_create_labeler_cap)
-  [Function `destroy_labeler_cap`](#0x10cf_label_destroy_labeler_cap)
-  [Function `is_type`](#0x10cf_label_is_type)
-  [Function `value`](#0x10cf_label_value)
-  [Function `create`](#0x10cf_label_create)
-  [Function `destroy`](#0x10cf_label_destroy)
-  [Function `destroy_opt`](#0x10cf_label_destroy_opt)
-  [Function `type_name`](#0x10cf_label_type_name)
-  [Function `clone`](#0x10cf_label_clone)
-  [Function `clone_opt`](#0x10cf_label_clone_opt)


<pre><code><b>use</b> <a href="../move-stdlib/ascii.md#0x1_ascii">0x1::ascii</a>;
<b>use</b> <a href="../move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../move-stdlib/string.md#0x1_string">0x1::string</a>;
<b>use</b> <a href="../move-stdlib/type_name.md#0x1_type_name">0x1::type_name</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../sui-framework/types.md#0x2_types">0x2::types</a>;
</code></pre>



<a name="0x10cf_label_LabelerCap"></a>

## Resource `LabelerCap`

<code><a href="label.md#0x10cf_label_LabelerCap">LabelerCap</a></code> allows to create labels of the specific type <code>L</code>.
Can be publicly transferred like any other object.


<pre><code><b>struct</b> <a href="label.md#0x10cf_label_LabelerCap">LabelerCap</a>&lt;L&gt; <b>has</b> store, key
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

<a name="0x10cf_label_Label"></a>

## Struct `Label`

<code><a href="label.md#0x10cf_label_Label">Label</a></code> is an immutable label representation.
The only way to create instances is through the <code><a href="label.md#0x10cf_label_LabelerCap">LabelerCap</a>&lt;L&gt;</code>.
Upon creation, <code>value</code> field becomes the fully qualified type name of <code>L</code>.


<pre><code><b>struct</b> <a href="label.md#0x10cf_label_Label">Label</a> <b>has</b> store
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

<a name="@Constants_0"></a>

## Constants


<a name="0x10cf_label_ENotOneTimeWitness"></a>

Error code for when a type passed to the <code>create_labeler_cap</code> function is not a one-time witness.


<pre><code><b>const</b> <a href="label.md#0x10cf_label_ENotOneTimeWitness">ENotOneTimeWitness</a>: u64 = 0;
</code></pre>



<a name="0x10cf_label_create_labeler_cap"></a>

## Function `create_labeler_cap`

Create a <code><a href="label.md#0x10cf_label_LabelerCap">LabelerCap</a></code> instance.
Can be created only by consuming a one time witness.


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x10cf_label_create_labeler_cap">create_labeler_cap</a>&lt;L: drop&gt;(witness: L, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="label.md#0x10cf_label_LabelerCap">label::LabelerCap</a>&lt;L&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x10cf_label_create_labeler_cap">create_labeler_cap</a>&lt;L: drop&gt;(witness: L, ctx: &<b>mut</b> TxContext): <a href="label.md#0x10cf_label_LabelerCap">LabelerCap</a>&lt;L&gt; {
    <b>assert</b>!(<a href="../sui-framework/types.md#0x2_types_is_one_time_witness">types::is_one_time_witness</a>(&witness), <a href="label.md#0x10cf_label_ENotOneTimeWitness">ENotOneTimeWitness</a>);

    <a href="label.md#0x10cf_label_LabelerCap">LabelerCap</a>&lt;L&gt; {
        id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
    }
}
</code></pre>



</details>

<a name="0x10cf_label_destroy_labeler_cap"></a>

## Function `destroy_labeler_cap`

Delete a <code><a href="label.md#0x10cf_label_LabelerCap">LabelerCap</a></code> instance.
If a capability is destroyed, it is impossible to add the related labels.


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x10cf_label_destroy_labeler_cap">destroy_labeler_cap</a>&lt;L&gt;(cap: <a href="label.md#0x10cf_label_LabelerCap">label::LabelerCap</a>&lt;L&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x10cf_label_destroy_labeler_cap">destroy_labeler_cap</a>&lt;L&gt;(cap: <a href="label.md#0x10cf_label_LabelerCap">LabelerCap</a>&lt;L&gt;) {
    <b>let</b> <a href="label.md#0x10cf_label_LabelerCap">LabelerCap</a>&lt;L&gt; {
        id,
    } = cap;

    <a href="../sui-framework/object.md#0x2_object_delete">object::delete</a>(id);
}
</code></pre>



</details>

<a name="0x10cf_label_is_type"></a>

## Function `is_type`

Check if a <code><a href="label.md#0x10cf_label_Label">Label</a></code> represents the type <code>L</code>.


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x10cf_label_is_type">is_type</a>&lt;L&gt;(self: &<a href="label.md#0x10cf_label_Label">label::Label</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x10cf_label_is_type">is_type</a>&lt;L&gt;(self: &<a href="label.md#0x10cf_label_Label">Label</a>): bool {
    self.value == <a href="../move-stdlib/type_name.md#0x1_type_name">type_name</a>&lt;L&gt;()
}
</code></pre>



</details>

<a name="0x10cf_label_value"></a>

## Function `value`

Function to get the value of a <code><a href="label.md#0x10cf_label_Label">Label</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x10cf_label_value">value</a>(self: &<a href="label.md#0x10cf_label_Label">label::Label</a>): &<a href="../move-stdlib/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x10cf_label_value">value</a>(self: &<a href="label.md#0x10cf_label_Label">Label</a>): &String {
    &self.value
}
</code></pre>



</details>

<a name="0x10cf_label_create"></a>

## Function `create`

Create a <code><a href="label.md#0x10cf_label_Label">Label</a></code> instance.
The created label holds a fully qualified type name with the original package IDs.
Can be called only by the related <code><a href="label.md#0x10cf_label_LabelerCap">LabelerCap</a></code> owner.


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x10cf_label_create">create</a>&lt;L&gt;(_: &<a href="label.md#0x10cf_label_LabelerCap">label::LabelerCap</a>&lt;L&gt;): <a href="label.md#0x10cf_label_Label">label::Label</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x10cf_label_create">create</a>&lt;L&gt;(_: &<a href="label.md#0x10cf_label_LabelerCap">LabelerCap</a>&lt;L&gt;): <a href="label.md#0x10cf_label_Label">Label</a> {
    <a href="label.md#0x10cf_label_Label">Label</a> {
        value: <a href="../move-stdlib/type_name.md#0x1_type_name">type_name</a>&lt;L&gt;(),
    }
}
</code></pre>



</details>

<a name="0x10cf_label_destroy"></a>

## Function `destroy`

Destroy a <code><a href="label.md#0x10cf_label_Label">Label</a></code> instance.


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x10cf_label_destroy">destroy</a>(self: <a href="label.md#0x10cf_label_Label">label::Label</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x10cf_label_destroy">destroy</a>(self: <a href="label.md#0x10cf_label_Label">Label</a>) {
    <b>let</b> <a href="label.md#0x10cf_label_Label">Label</a> {
        value: _,
    } = self;
}
</code></pre>



</details>

<a name="0x10cf_label_destroy_opt"></a>

## Function `destroy_opt`

Destroy an optional <code><a href="label.md#0x10cf_label_Label">Label</a></code> instance.


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x10cf_label_destroy_opt">destroy_opt</a>(self: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="label.md#0x10cf_label_Label">label::Label</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="label.md#0x10cf_label_destroy_opt">destroy_opt</a>(self: Option&lt;<a href="label.md#0x10cf_label_Label">Label</a>&gt;) {
    <b>if</b> (self.is_some()) {
        <a href="label.md#0x10cf_label_destroy">destroy</a>(<a href="../move-stdlib/option.md#0x1_option_destroy_some">option::destroy_some</a>(self));
    }
    <b>else</b> {
        <a href="../move-stdlib/option.md#0x1_option_destroy_none">option::destroy_none</a>(self);
    };
}
</code></pre>



</details>

<a name="0x10cf_label_type_name"></a>

## Function `type_name`

Return a fully qualified type name with the original package IDs.


<pre><code><b>fun</b> <a href="../move-stdlib/type_name.md#0x1_type_name">type_name</a>&lt;L&gt;(): <a href="../move-stdlib/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../move-stdlib/type_name.md#0x1_type_name">type_name</a>&lt;L&gt;(): String {
    <a href="../move-stdlib/string.md#0x1_string_from_ascii">string::from_ascii</a>(std::type_name::get_with_original_ids&lt;L&gt;().into_string())
}
</code></pre>



</details>

<a name="0x10cf_label_clone"></a>

## Function `clone`

Clone a <code><a href="label.md#0x10cf_label_Label">Label</a></code> instance.
It is a protected utility function, it should be impossible to clone <code><a href="label.md#0x10cf_label_Label">Label</a></code>
because it leads to unauthorized labeled objects creation.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="label.md#0x10cf_label_clone">clone</a>(self: &<a href="label.md#0x10cf_label_Label">label::Label</a>): <a href="label.md#0x10cf_label_Label">label::Label</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="label.md#0x10cf_label_clone">clone</a>(self: &<a href="label.md#0x10cf_label_Label">Label</a>): <a href="label.md#0x10cf_label_Label">Label</a> {
    <a href="label.md#0x10cf_label_Label">Label</a> {
        value: self.value,
    }
}
</code></pre>



</details>

<a name="0x10cf_label_clone_opt"></a>

## Function `clone_opt`

Clone an optional <code><a href="label.md#0x10cf_label_Label">Label</a></code> instance.
It is a protected utility function, it should be impossible to clone <code><a href="label.md#0x10cf_label_Label">Label</a></code>
because it leads to unauthorized labeled objects creation.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="label.md#0x10cf_label_clone_opt">clone_opt</a>(self: &<a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="label.md#0x10cf_label_Label">label::Label</a>&gt;): <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="label.md#0x10cf_label_Label">label::Label</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="label.md#0x10cf_label_clone_opt">clone_opt</a>(self: &Option&lt;<a href="label.md#0x10cf_label_Label">Label</a>&gt;): Option&lt;<a href="label.md#0x10cf_label_Label">Label</a>&gt; {
    <b>if</b> (self.is_some()) {
        <a href="../move-stdlib/option.md#0x1_option_some">option::some</a>(<a href="label.md#0x10cf_label_clone">clone</a>(self.borrow()))
    }
    <b>else</b> {
        <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>()
    }
}
</code></pre>



</details>
