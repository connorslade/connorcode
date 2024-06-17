<script lang="ts">
	import type { PageData } from './$types';

	import Breadcrumbs from '$lib/components/Breadcrumbs.svelte';
	import Title from '$lib/components/Title.svelte';
	import Link from '$lib/components/Link.svelte';
	import Head from '$lib/components/Head.svelte';

	import GithubLogo from 'phosphor-svelte/lib/GithubLogo';
	import LinkIcon from 'phosphor-svelte/lib/Link';
	import HtmlRenderer from '$lib/components/HtmlRenderer.svelte';
	import { NaiveDate } from '$lib/date';

	export let data: PageData;

	let date = new NaiveDate(data.info.date);
	$: reading_time = (data.info.word_count / 3.5 / 60).toFixed(0);
</script>

<Head
	title={data.info.name}
	description={data.info.description}
	article={true}
	published={new Date(data.info.date)}
	section="projects"
	hero={data.info.hero}
/>

<Breadcrumbs
	crumbs={['projects', data.info.slug]}
	links={['/projects', `/projects/${data.info.slug}`]}
	style="margin-bottom: 0"
/>

<div class="title-container">
	<div class="title">
		<Title
			title={data.info.name}
			info="created {date.human_date()} &bull; {reading_time}m reading time &bull; {data.info
				.views} view{data.info.views == 1 ? '' : 's'}"
		/>
	</div>
	<div class="icons">
		{#if data.info.link != null}
			<Link href={data.info.link}>
				<LinkIcon size={24} color="var(--text-color)" class="icon" />
			</Link>
		{/if}
		{#if data.info.github != null}
			<Link href={data.info.github}>
				<GithubLogo size={24} color="var(--text-color)" class="icon" />
			</Link>
		{/if}
	</div>
</div>

<HtmlRenderer html={data.html} base={`/projects/${data.info.slug}/`} />

<style lang="scss">
	.title-container {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}
</style>
