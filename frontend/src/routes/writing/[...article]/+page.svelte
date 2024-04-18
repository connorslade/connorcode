<script lang="ts">
	import type { PageData } from './$types';

	import Head from '$lib/components/Head.svelte';
	import Rule from '$lib/components/Rule.svelte';
	import HtmlRenderer from '$lib/components/HtmlRenderer.svelte';
	import Breadcrumbs from '$lib/components/Breadcrumbs.svelte';

	export let data: PageData;
</script>

<Head title="Using libmpv in Rust" description="todo" />

<Breadcrumbs
	crumbs={['writing', 'minecraft', 'fast-doll']}
	links={['/writing', '/writing/minecraft', '/writing/minecraft/fast-doll']}
/>

<h1 class="title">Fast Doll</h1>

<span class="date">published 07/09/2023 &bull; 206 words &bull; 2m reading time</span>

<HtmlRenderer html={data.html} />

<Rule style="dashed" />

<h2>Comments</h2>

<p>No comments yet... You can be the first!</p>

<style lang="scss">
	.title {
		margin-bottom: 0.2em;
	}

	.date {
		margin-top: 0;
		font-size: 0.8em;
	}
</style>
