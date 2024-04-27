<script lang="ts">
	import type { PageData } from './$types';

	import Head from '$lib/components/Head.svelte';
	import Rule from '$lib/components/Rule.svelte';
	import HtmlRenderer from '$lib/components/HtmlRenderer.svelte';
	import Breadcrumbs from '$lib/components/Breadcrumbs.svelte';
	import Title from '$lib/components/Title.svelte';

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

	$: reading_time = (data.info.word_count / 3.5 / 60).toFixed(0);
</script>

<Head title={data.info.title} description={data.info.description} />

<Breadcrumbs crumbs={crumbs(data.info.path)} links={crumb_links(data.info.path)} />

<Title
	title={data.info.title}
	info={`published ${data.info.date} &bull; ${data.info.word_count} words &bull; ${reading_time}m reading time`}
/>

<HtmlRenderer html={data.html} base={`/writing/${data.info.path}/`} />

<Rule style="dashed" />

<h2>Comments</h2>

<p>No comments yet... You can be the first!</p>
