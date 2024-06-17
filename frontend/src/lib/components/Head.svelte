<script lang="ts">
	import type { Hero } from '$lib/types';
	import { page } from '$app/stores';
	import { env } from '$env/dynamic/public';
	import { html_to_text, image_mime } from '$lib/utils';

	export let title: string;
	export let description: string;
	export let article: boolean = false;
	export let published: Date | null = null;
	export let hero: Hero | null = null;
	export let path: string | null = null;
	export let section: string | null = null;

	let txt_description = html_to_text(description);
	let _path = path ?? $page.url.pathname;
	if (_path.startsWith('/')) _path = _path.slice(1);
	if (_path.endsWith('/')) _path = _path.slice(0, -1);
</script>

<svelte:head>
	<title>{title} &mdash; Connor Code</title>
	<meta name="description" content={txt_description} />

	<meta property="og:title" content={title} />
	<meta property="og:description" content={txt_description} />
	<meta property="og:url" content="{env.PUBLIC_ADDRESS}/{_path}" />
	{#if published}
		<meta property="article:published_time" content={published.toISOString()} />
	{/if}

	<meta property="og:site_name" content="connorcode" />
	<meta property="og:locale" content="en_US" />
	<meta property="og:type" content={article ? 'article' : 'website'} />

	{#if article}
		<meta property="article:author" content="Connor Slade" />
	{/if}

	{#if section}
		<meta property="article:section" content={section} />
	{/if}

	{#if hero}
		<meta property="og:image" content="{env.PUBLIC_ADDRESS}/{_path}/{hero.image}" />
		<meta property="og:image:alt" content={hero.alt} />
		<meta property="og:image:type" content={image_mime(hero.image)} />
		<meta name="twitter:card" content="summary_large_image" />
		<meta name="twitter:image" content="{env.PUBLIC_ADDRESS}/{_path}/{hero.image}" />
	{/if}
</svelte:head>
