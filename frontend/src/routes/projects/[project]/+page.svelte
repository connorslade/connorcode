<script lang="ts">
	import Breadcrumbs from '$lib/components/Breadcrumbs.svelte';
	import Title from '$lib/components/Title.svelte';
	import Link from '$lib/components/Link.svelte';
	import Head from '$lib/components/Head.svelte';

	import GithubLogo from 'phosphor-svelte/lib/GithubLogo';
	import LinkIcon from 'phosphor-svelte/lib/Link';
	import type { PageData } from './$types';
	import HtmlRenderer from '$lib/components/HtmlRenderer.svelte';

	export let data: PageData;
</script>

<Head title="ridgehacks2024" description="todo" />

<Breadcrumbs
	crumbs={['projects', data.info.slug]}
	links={['/projects', `/projects/${data.info.slug}`]}
	style="margin-bottom: 0"
/>

<div class="title-container">
	<div class="title">
		<Title title={data.info.name} info="Created February 2, 2024" />
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

<HtmlRenderer html={data.html} base={`/project/${data.info.slug}/`} />

<style lang="scss">
	.title-container {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}
</style>
