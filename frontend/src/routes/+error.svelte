<script lang="ts">
	import { page } from '$app/stores';
	import Link from '$lib/components/Link.svelte';
	import Head from '$lib/components/Head.svelte';
	import { SITE_REPO } from '$lib/consts';

	function report_address(title: string, body: string): string {
		return `https://github.com/${SITE_REPO}/issues/new?title=${encodeURIComponent(title)}&body=${encodeURIComponent(body)}`;
	}
</script>

<Head title={`Error ${$page.status}`} description="There was an error loading this page." />

<h1>Error {$page.status} &mdash; {$page.error?.message}</h1>

{#if $page.status == 404}
	<p>
		Looks like the page <code>{$page.url.pathname}</code> was not found.
	</p>
{/if}

<p>
	If you believe that this error should be fixed, feel free to submit a <Link
		href={report_address(
			`Error ${$page.status} on \`${$page.url.pathname}\``,
			'Describe the error here.'
		)}>bug report</Link
	>.
</p>
