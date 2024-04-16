<script lang="ts">
	import Head from '$lib/Head.svelte';
	import Link from '$lib/Link.svelte';

	import { VERSION, GITHUB } from '$lib/consts';

	function get_greeting(): string {
		let hour = new Date().getHours();
		if (hour >= 17) return 'Good evening';
		if (hour >= 12) return 'Good afternoon';
		if (hour >= 3) return 'Good morning';
		return 'Good evening';
	}
</script>

<Head title="Home" description="" />

<h1>Home</h1>

<p>
	{get_greeting()} 👋, I'm Connor Slade. Welcome to my little place on the internet,
	<Link href="https://connorcode.com">connorcode.com</Link> <em>version {VERSION}</em>! (Finally
	rewritten in Svelte).
</p>

<p>
	I'm interested in
	<Link href="https://photos.connorcode.com">Photography</Link>,
	<Link href="https://www.minecraft.net">Minecraft</Link>,
	<Link href={GITHUB}>Software development</Link> and some Web development.
</p>

<p>
	I started my <em>programming journey</em> around 2014 with the Arduino. Then I learned how to make
	basic websites with HTML and CSS, then how to make them interactive with JavaScript. After that, I
	got into Python and eventually started to learn much more about Linux, Docker, different
	programming languages. After trying many languages I eventually settled on <em>Rust</em> for most of
	my projects.
</p>

<h2>Contact</h2>

<p>If you have any questions or comments you can contact through:</p>
<p></p>
<ul>
	<li>Discord: <code>sigma76</code></li>
	<li>Email: <a href="mailto:connor@connorcode.com">connor@connorcode.com</a></li>
</ul>

<h2>Recent</h2>

<p>
	Here are some links to recent projects I have worked on and articles I have written. The full
	lists can be found under the projects and writing sections respectively.
</p>

<h3>Projects</h3>

<ul>
	<li><Link href="">ridgehacks2024</Link> &mdash; Website for the 2024 Ridgehacks hackathon</li>
	<li>
		<Link href="">auto-reauth</Link> &mdash; Automatically re-authenticates your Minecraft session when
		it expires.
	</li>
</ul>

<h3>Writing</h3>

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
