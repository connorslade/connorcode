<script lang="ts">
	import Head from '$lib/components/Head.svelte';
	import Link from '$lib/components/Link.svelte';

	import { VERSION, GITHUB } from '$lib/consts';
	import type { PageData } from './$types';

	function get_greeting(): string {
		let hour = new Date().getHours();
		if (hour >= 17) return 'Good evening';
		if (hour >= 12) return 'Good afternoon';
		if (hour >= 3) return 'Good morning';
		return 'Good evening';
	}

	export let data: PageData;
</script>

<Head title="Home" description="" />

<h1>Home</h1>

<p>
	{get_greeting()} 👋, I'm Connor Slade. Welcome to my little place on the internet,
	<Link href="https://connorcode.com">connorcode.com</Link> <em>version {VERSION}</em>! (Finally
	rewritten in Svelte)
</p>

<p>
	I'm interested in <Link href={GITHUB}>Software development</Link>,
	<Link href="https://photos.connorcode.com">Photography</Link>,
	<Link href="https://www.minecraft.net">Minecraft</Link>, and some Web development (as you can
	see).
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

<ul class="project-list">
	{#each data.projects as project}
		<li>
			<Link href={`projects/${project.slug}`} open_in="current-tab">{project.name}</Link> &mdash; {@html project.description}
		</li>
	{/each}
</ul>

<h3>Writing</h3>

<ul class="article-list">
	{#each data.articles as article}
		<li>
			<Link href={`writing/${article.path}`} open_in="current-tab">{article.title}</Link> &mdash; {@html article.description}
		</li>
	{/each}
</ul>

<style>
	:global(.article-list p),
	:global(.project-list p) {
		display: inline;
	}

	.article-list li,
	.project-list li {
		margin-bottom: 1em;
	}
</style>
