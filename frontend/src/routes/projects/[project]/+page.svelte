<script lang="ts">
	import Breadcrumbs from '$lib/components/Breadcrumbs.svelte';
	import Title from '$lib/components/Title.svelte';
	import Link from '$lib/components/Link.svelte';
	import Head from '$lib/components/Head.svelte';

	import GithubLogo from 'phosphor-svelte/lib/GithubLogo';
	import LinkIcon from 'phosphor-svelte/lib/Link';
</script>

<Head title="ridgehacks2024" description="todo" />

<Breadcrumbs
	crumbs={['projects', 'ridgehacks2024']}
	links={['/projects', '']}
	style="margin-bottom: 0"
/>

<div class="title-container">
	<div class="title">
		<Title title="ridgehacks2024" info="Created February 2, 2024" />
	</div>
	<div class="icons">
		<Link href="/">
			<LinkIcon size={24} color="var(--text-color)" class="icon" />
		</Link>
		<Link href="/">
			<GithubLogo size={24} color="var(--text-color)" class="icon" />
		</Link>
	</div>
</div>

<p>
	This is the website for Ridge Computer Science Club's yearly hackathon, Ridgehacks! Written as a
	single page Svelte app.
</p>

<img src="https://i.imgur.com/ZmA2UI4.png" alt="" />

<p>
	We just decided to continue with the general design style from the previous year's websites. Which
	you can see with the following links:
</p>

<ul>
	<li><a href="/">2023</a></li>
	<li><a href="/">2022</a></li>
	<li><a href="/">2021</a></li>
</ul>

<p>
	For next year, I am hoping to make a more stylized site, maybe like some kinda 90s hacker thing.
	Ill probably take lots of inspiration from old archived GeoCities sites.
</p>

<style lang="scss">
	.title-container {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}
</style>
