<script lang="ts">
	import type { ImageSize } from '$lib/types';

	import { decode } from 'blurhash';
	import { onMount } from 'svelte';

	export let size: ImageSize;
	export let hash: string;

	export let src: string;
	export let alt: string;

	let canvas: HTMLCanvasElement;
	let image: Promise<string>;

	let aspect_ratio = `${size.width}/${size.height}`;

	const pixels = decode(hash, 32, 32);
	onMount(() => {
		image = fetch(src)
			.then((response) => response.blob())
			.then((blob) => URL.createObjectURL(blob));

		const ctx = canvas.getContext('2d');
		const imageData = ctx?.createImageData(32, 32);
		imageData?.data.set(pixels);
		if (imageData) ctx?.putImageData(imageData, 0, 0);
	});
</script>

<div class="container" style:aspect-ratio={aspect_ratio} style:max-width={`${size.width}px`}>
	{#await image then src}
		<img {src} {alt} width={`${size.width}px`} height={`${size.height}px`} />
	{/await}

	<canvas
		bind:this={canvas}
		width={32}
		height={32}
		style:aspect-ratio={aspect_ratio}
		style:max-width={`${size.width}px`}
	/>
</div>

<style lang="scss">
	canvas {
		width: 100%;
		border-radius: 5px;

		position: absolute;
		z-index: -1;
		top: 0;
		left: 0;
	}

	img {
		animation: fade-in 200ms;
	}

	.container {
		position: relative;
		margin-top: 16px;
		margin-bottom: 16px;
		width: 100%;
	}

	@keyframes fade-in {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}
</style>
