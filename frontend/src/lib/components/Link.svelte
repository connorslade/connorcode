<script lang="ts">
	type OpenIn = 'new-tab' | 'current-tab';

	export let href: string;
	export let open_in: OpenIn = 'new-tab';
	export let style: string = '';
</script>

{#if open_in === 'new-tab'}
	<a {href} {style} target="_blank" rel="noopener noreferrer">
		<slot />
	</a>
{:else}
	<a {href} {style}>
		<slot />
	</a>
{/if}
