<script lang="ts">
	export let crumbs: string[];
	export let links: string[];
</script>

<p>
	{#each crumbs as crumb, idx}
		{#if idx != 0}
			&nbsp;»
		{/if}
		<a href={links[idx]} class="breadcrumb">{crumb}</a>
	{/each}
</p>

<style lang="scss">
	.breadcrumb {
		color: var(--font-color);
		text-decoration: none;

		&:hover {
			text-decoration: underline;
		}
	}
</style>
