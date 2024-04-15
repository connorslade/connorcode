<script lang="ts">
	enum OpenIn {
		NewTab = '_blank',
		SameTab = '_self'
	}

	export let href: string;
	export let openIn: OpenIn = OpenIn.NewTab;
	export let style: string = '';
</script>

{#if openIn === OpenIn.NewTab}
	<a {href} {style} target="_blank" rel="noopener noreferrer">
		<slot />
	</a>
{:else}
	<a {href} {style}>
		<slot />
	</a>
{/if}
