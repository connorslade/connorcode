<script lang="ts">
	import type { PageData } from './$types';

	import Head from '$lib/components/Head.svelte';
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

<Head
	title={data.info.title}
	description={data.info.description}
	article={true}
	published={new Date(data.info.date)}
/>

<Breadcrumbs crumbs={crumbs(data.info.path)} links={crumb_links(data.info.path)} />

<Title
	title={data.info.title}
	info="published {data.info.date} &bull; {reading_time}m reading time &bull; {data.info
		.views} view{data.info.views == 1 ? '' : 's'}"
/>

<HtmlRenderer html={data.html} base={`/writing/${data.info.path}/`} />

<!-- <Rule style="dashed" />

<h2>Comments</h2>

<textarea class="comment-text"></textarea>
<br />
<button>Comment</button>

<p>No comments yet... You can be the first!</p>

<style lang="scss">
	.comment-text {
		width: 100%;
		height: 75px;
		border-radius: 5px;
		background-color: var(--background-color);
		color: var(--text-color);
	}
</style> -->
