<script lang="ts">
	export let title: string;
	export let description: string;
	export let article: boolean = false;
	export let published: Date | null = null;
</script>

<svelte:head>
	<title>{title} &mdash; Connor Code</title>
	<meta name="description" content={description} />

	<meta property="og:title" content={title} />
	<meta property="og:description" content={description} />
	{#if published}
		<meta property="article:published_time" content={published.toISOString()} />
	{/if}

	<meta property="og:site_name" content="connorcode" />
	<meta property="og:locale" content="en_US" />
	<meta property="og:type" content={article ? 'article' : 'website'} />
</svelte:head>
