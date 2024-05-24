<script lang="ts">
	import type { PageData } from './$types';

	import Head from '$lib/components/Head.svelte';
	import HtmlRenderer from '$lib/components/HtmlRenderer.svelte';
	import Breadcrumbs from '$lib/components/Breadcrumbs.svelte';
	import Title from '$lib/components/Title.svelte';

	export let data: PageData;

	$: reading_time = (data.info.word_count / 3.5 / 60).toFixed(0);
</script>

<Head title={data.info.title} description={data.info.description} />

<Breadcrumbs
	crumbs={['writing', data.info.path]}
	links={['/writing', `/writing/${data.info.path}`]}
/>

<Title
	title={data.info.title}
	info={`published ${data.info.date} &bull; ${data.info.word_count} words &bull; ${reading_time}m reading time`}
/>

<HtmlRenderer html={data.html} base={`/writing/${data.info.path}/`} />
