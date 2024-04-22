---
title: Module `0x107a::nft`
---



-  [Resource `Nft`](#0x107a_nft_Nft)
-  [Function `destroy`](#0x107a_nft_destroy)
-  [Function `sender`](#0x107a_nft_sender)
-  [Function `metadata`](#0x107a_nft_metadata)
-  [Function `tag`](#0x107a_nft_tag)
-  [Function `immutable_issuer`](#0x107a_nft_immutable_issuer)
-  [Function `immutable_metadata`](#0x107a_nft_immutable_metadata)


<pre><code><b>use</b> <a href="irc27.md#0x107a_irc27">0x107a::irc27</a>;
<b>use</b> <a href="../move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
</code></pre>



<a name="0x107a_nft_Nft"></a>

## Resource `Nft`

The Stardust NFT representation.


<pre><code><b>struct</b> <a href="nft.md#0x107a_nft_Nft">Nft</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>
 The Nft's ID is nested from Stardust.
</dd>
<dt>
<code>sender: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
 The sender feature.
</dd>
<dt>
<code>metadata: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>
 The metadata feature.
</dd>
<dt>
<code>tag: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>
 The tag feature.
</dd>
<dt>
<code>immutable_issuer: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
 The immutable issuer feature.
</dd>
<dt>
<code>immutable_metadata: <a href="irc27.md#0x107a_irc27_Irc27Metadata">irc27::Irc27Metadata</a></code>
</dt>
<dd>
 The immutable metadata feature.
</dd>
</dl>


</details>

<a name="0x107a_nft_destroy"></a>

## Function `destroy`

Permanently destroy an <code><a href="nft.md#0x107a_nft_Nft">Nft</a></code> object.


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x107a_nft_destroy">destroy</a>(<a href="nft.md#0x107a_nft">nft</a>: <a href="nft.md#0x107a_nft_Nft">nft::Nft</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x107a_nft_destroy">destroy</a>(<a href="nft.md#0x107a_nft">nft</a>: <a href="nft.md#0x107a_nft_Nft">Nft</a>) {
    <b>let</b> <a href="nft.md#0x107a_nft_Nft">Nft</a> {
        id: id,
        sender: _,
        metadata: _,
        tag: _,
        immutable_issuer: _,
        immutable_metadata: immutable_metadata,
    } = <a href="nft.md#0x107a_nft">nft</a>;

    <a href="irc27.md#0x107a_irc27_destroy">irc27::destroy</a>(immutable_metadata);

    <a href="../sui-framework/object.md#0x2_object_delete">object::delete</a>(id);
}
</code></pre>



</details>

<a name="0x107a_nft_sender"></a>

## Function `sender`

Get the NFT's <code>sender</code>.


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x107a_nft_sender">sender</a>(<a href="nft.md#0x107a_nft">nft</a>: &<a href="nft.md#0x107a_nft_Nft">nft::Nft</a>): &<a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x107a_nft_sender">sender</a>(<a href="nft.md#0x107a_nft">nft</a>: &<a href="nft.md#0x107a_nft_Nft">Nft</a>): &Option&lt;<b>address</b>&gt; {
    &<a href="nft.md#0x107a_nft">nft</a>.sender
}
</code></pre>



</details>

<a name="0x107a_nft_metadata"></a>

## Function `metadata`

Get the NFT's <code>metadata</code>.


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x107a_nft_metadata">metadata</a>(<a href="nft.md#0x107a_nft">nft</a>: &<a href="nft.md#0x107a_nft_Nft">nft::Nft</a>): &<a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x107a_nft_metadata">metadata</a>(<a href="nft.md#0x107a_nft">nft</a>: &<a href="nft.md#0x107a_nft_Nft">Nft</a>): &Option&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt; {
    &<a href="nft.md#0x107a_nft">nft</a>.metadata
}
</code></pre>



</details>

<a name="0x107a_nft_tag"></a>

## Function `tag`

Get the NFT's <code>tag</code>.


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x107a_nft_tag">tag</a>(<a href="nft.md#0x107a_nft">nft</a>: &<a href="nft.md#0x107a_nft_Nft">nft::Nft</a>): &<a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x107a_nft_tag">tag</a>(<a href="nft.md#0x107a_nft">nft</a>: &<a href="nft.md#0x107a_nft_Nft">Nft</a>): &Option&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt; {
    &<a href="nft.md#0x107a_nft">nft</a>.tag
}
</code></pre>



</details>

<a name="0x107a_nft_immutable_issuer"></a>

## Function `immutable_issuer`

Get the NFT's <code>immutable_sender</code>.


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x107a_nft_immutable_issuer">immutable_issuer</a>(<a href="nft.md#0x107a_nft">nft</a>: &<a href="nft.md#0x107a_nft_Nft">nft::Nft</a>): &<a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x107a_nft_immutable_issuer">immutable_issuer</a>(<a href="nft.md#0x107a_nft">nft</a>: &<a href="nft.md#0x107a_nft_Nft">Nft</a>): &Option&lt;<b>address</b>&gt; {
    &<a href="nft.md#0x107a_nft">nft</a>.immutable_issuer
}
</code></pre>



</details>

<a name="0x107a_nft_immutable_metadata"></a>

## Function `immutable_metadata`

Get the NFT's <code>immutable_metadata</code>.


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x107a_nft_immutable_metadata">immutable_metadata</a>(<a href="nft.md#0x107a_nft">nft</a>: &<a href="nft.md#0x107a_nft_Nft">nft::Nft</a>): &<a href="irc27.md#0x107a_irc27_Irc27Metadata">irc27::Irc27Metadata</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x107a_nft_immutable_metadata">immutable_metadata</a>(<a href="nft.md#0x107a_nft">nft</a>: &<a href="nft.md#0x107a_nft_Nft">Nft</a>): &Irc27Metadata {
    &<a href="nft.md#0x107a_nft">nft</a>.immutable_metadata
}
</code></pre>



</details>
