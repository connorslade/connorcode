<script>
	import Admonition from '$lib/Admonition.svelte';
	import Head from '$lib/Head.svelte';
	import Link from '$lib/Link.svelte';
</script>

<Head title="Home" description="" />

<h1>Home</h1>

Hello World 👋🌎! Welcome to my personal home page.
<br /><br />

You can checkout some of my projects:

<ul>
	<li><Link href="">ridgehacks2024</Link> &mdash; Website for the 2024 Ridgehacks hackathon</li>
	<li>
		<Link href="">auto-reauth</Link> &mdash; Automatically re-authenticates your Minecraft session when
		it expires.
	</li>
</ul>

Or read some articles I have written:

<ul>
	<li>
		<a href="/writing/minecraft/list-of-minecraft-enchantments">List of Minecraft Enchantments</a>
		&mdash; Downloadable and interactive list of all minecraft enchantments
	</li>
	<li>
		<a href="/writing/afire/update-8">afire Update (V2.2.1)</a> &mdash; Updates to afire in V2.2.1
	</li>
	<li>
		<a href="/writing/tutorial/using-libmpv-in-rust">Using libmpv in Rust</a> &mdash; Compiling and linking
		with libmpv-rs in rust
	</li>
</ul>
