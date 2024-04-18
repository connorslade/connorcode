<script lang="ts">
	type Style = 'solid' | 'dashed';
	type Thickness = 'normal' | 'thin';

	export let style: Style = 'solid';
	export let thickness: Thickness = 'normal';
</script>

{#if thickness == 'normal'}
	<hr style:border={`1px ${style}`} />
{:else if thickness == 'thin'}
	<hr style:border="none" style:border-bottom={`1px ${style}`} />
{/if}
