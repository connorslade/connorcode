<script lang="ts">
	export let title_element: string = 'h1';
	export let title_style: string = '';

	export let title: string;
	export let info: string;
</script>

<svelte:element this={title_element} class="title" style={title_style}>{title}</svelte:element>
<span class="date">{@html info}</span>

<style lang="scss">
	.title {
		margin-bottom: 0.2em;
	}

	.date {
		margin-top: 0;
		font-size: 0.8em;
		color: var(--text-color);
	}
</style>
