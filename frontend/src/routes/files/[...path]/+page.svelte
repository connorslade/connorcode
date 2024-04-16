<script lang="ts">
	import type { PageData } from './$types';
	import { page } from '$app/stores';
	import Head from '$lib/Head.svelte';

	import { File, Folder } from 'phosphor-svelte';
	import humanize_duration from 'humanize-duration';

	function humanFileSize(size: number): string {
		var i = size == 0 ? 0 : Math.floor(Math.log(size) / Math.log(1024));
		return +(size / Math.pow(1024, i)).toFixed(2) * 1 + ' ' + ['B', 'kB', 'MB', 'GB', 'TB'][i];
	}

	export let data: PageData;
</script>

<Head title="Files" description="todo" />

<h1>Files</h1>

{#if 'error' in data}
	<p>The directory <code>{$page.params.path}</code> was not found.</p>
{:else}
	{#each data.files as file}
		<a href={`/files/${file.path}`} class="file">
			<div class="name"><svelte:component this={file.is_dir ? Folder : File} /> {file.name}</div>
			<div class="size">{file.is_dir ? '' : humanFileSize(file.size)}</div>
			<div class="updated">{humanize_duration(file.last_modified, { largest: 1 })} ago</div>
		</a>
	{/each}
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
			display: flex;
			gap: 5px;
		}
	}
</style>
