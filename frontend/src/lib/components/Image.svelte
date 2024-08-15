<script lang="ts">
	import type { ImageSize } from '$lib/types';

	import BlurHashImage from './BlurHashImage.svelte';

	export let size: ImageSize | null;
	export let hash: string | null;

	export let src: string;
	export let alt: string;
</script>

{#if size && hash}
	<BlurHashImage {src} {alt} {size} {hash} />
{:else if size}
	<img {src} {alt} width={size.width} height={size.height} loading="lazy" />
{:else}
	<img {src} {alt} loading="lazy" />
{/if}
