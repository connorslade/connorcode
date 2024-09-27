<script lang="ts">
	import type { PageData } from './$types';
	import { page } from '$app/stores';
	import Head from '$lib/components/Head.svelte';

	import Folder from 'phosphor-svelte/lib/Folder';
	import humanize_duration from 'humanize-duration';

	import { from_ext } from './filetype';
	import { human_file_size } from '$lib/utils';
	import Rule from '$lib/components/Rule.svelte';
	import HtmlRenderer from '$lib/components/HtmlRenderer.svelte';
	import * as _public from '$env/dynamic/public';

	$: path = $page.params.path;
	$: parts = path.split('/');

	function parent(depth: number): string {
		return parts.slice(0, parts.length - depth).join('/');
	}

	let current_time = new Date().getTime();

	export let data: PageData;
</script>

<Head title="Files" description="My file server." />

<h1>Files</h1>

{#if data.children == undefined}
	<p>You are being redirected.</p>
{:else if 'error' in data}
	<p>The directory <code>{path}</code> was not found.</p>
{:else}
	{#if path != ''}
		<!--TODO: Use breadcrumbs component-->
		<p>
			<a href="/files" class="breadcrumb">Files</a>{#each parts as segment, idx}
				&nbsp;»
				<a href={`/files/${parent(parts.length - idx - 1)}`} class="breadcrumb">{segment}</a>
			{/each}
		</p>
	{/if}

	{#key path}
		{#if path != ''}
			<a href={`/files/${parent(1)}`} class="file">
				<div class="name"><Folder /> ..</div>
			</a>
		{/if}
	{/key}

	{#each data.children as file}
		<a
			href={file.is_dir ? `/files/${file.path}` : `${_public.env.PUBLIC_FILE_ADDRESS}/${file.path}`}
			class="file"
		>
			<div class="name">
				<svelte:component this={file.is_dir ? Folder : from_ext(file.name.split('.')[1])} />
				{file.name}
			</div>
			<div class="size">{file.is_dir ? '' : human_file_size(file.size)}</div>
			<div class="updated">
				{humanize_duration(current_time - file.last_modified * 1000, { largest: 1, round: true })} ago
			</div>
		</a>
	{/each}

	{#if data.readme != null}
		<br />
		<Rule style="dashed" />
		<HtmlRenderer html={data.readme} />
	{/if}
{/if}

<style lang="scss">
	.file {
		display: grid;
		grid-template-columns: auto 100px 130px;
		color: var(--font-color);
		text-decoration: none;

		&:hover {
			text-decoration: underline;
		}

		& .name {
			min-width: 0;
			display: ruby;
			overflow: hidden;
			text-overflow: ellipsis;
		}
	}

	.breadcrumb {
		color: var(--font-color);
		text-decoration: none;

		&:hover {
			text-decoration: underline;
		}
	}

	@media (width <= 510px) {
		.file {
			grid-template-columns: auto;

			& .size,
			& .updated {
				display: none;
			}
		}
	}
</style>
