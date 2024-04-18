<script lang="ts">
	import type { PageData } from './$types';

	import Head from '$lib/components/Head.svelte';
	import Rule from '$lib/components/Rule.svelte';
	import HtmlRenderer from '$lib/components/HtmlRenderer.svelte';
	import Breadcrumbs from '$lib/components/Breadcrumbs.svelte';

	function crumbs(path: string): string[] {
		let parts = path.split('/');
		return ['writing'].concat(parts);
	}

	function crumb_links(path: string): string[] {
		let parts = path.split('/');
		let out = ['/writing'];
		for (let i = 0; i < parts.length; i++) {
			out.push(`${out[i]}/${parts[i]}`);
		}

		return out;
	}

	export let data: PageData;

	$: reading_time = (data.word_count / 3.5 / 60).toFixed(0);
</script>

<Head title="Using libmpv in Rust" description="todo" />

<Breadcrumbs crumbs={crumbs(data.info.path)} links={crumb_links(data.info.path)} />

<h1 class="title">{data.info.title}</h1>

<span class="date"
	>published 07/09/2023 &bull; {data.word_count} words &bull; {reading_time}m reading time</span
>

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
