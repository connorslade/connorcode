<script lang="ts">
	import Head from '$lib/components/Head.svelte';
</script>

<Head title="Writing" description="todo" />

<h1>Writing</h1>

<p>
	Here are some random things I have written about. To get notified when new posts are published,
	you can subscribe to the <a href="/">RSS feed</a>.
</p>

<h2>2023</h2>
<h3>September</h3>
<ul>
	<li>
		<a href="/writing/minecraft/list-of-minecraft-enchantments">List of Minecraft Enchantments</a> &mdash;
		Downloadable and interactive list of all minecraft enchantments
	</li>
</ul>

<h3>August</h3>
<ul>
	<li>
		<a href="/writing/afire/update-8">afire Update (V2.2.1)</a> &mdash; Updates to afire in V2.2.1
	</li>
</ul>

<h3>July</h3>
<ul>
	<li>
		<a href="/writing/tutorial/using-libmpv-in-rust">Using libmpv in Rust</a> &mdash; Compiling and linking
		with libmpv-rs in rust
	</li>
</ul>

<style lang="scss">
	.article {
		background: rgb(19, 19, 19);
		padding: 10px;
		border-radius: 5px;

		& .title-line {
			display: flex;
			justify-content: space-between;
		}
	}
</style>
