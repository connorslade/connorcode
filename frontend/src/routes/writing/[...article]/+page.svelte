<script lang="ts">
	import Head from '$lib/Head.svelte';
</script>

<Head title="Using libmpv in Rust" description="todo" />

<!-- <p>article: {$page.params.article}</p> -->

<!-- <p><a href="">writing</a> » <a href="">tutorial</a> » <a href="">using-libmpv-in-rust</a></p> -->

<h1 class="title">Using libmpv in Rust</h1>

<span class="date">published 07/09/2023 &bull; 206 words &bull; 2m reading time</span>

<p>
	I recently finish a project I had been wanting to work on for a long time (<a href=""
		>video-presenter</a
	>). It allows you to define cuepoints on a video and kinda use it like a slideshow, skipping to
	different cuepoints and waiting before continuing past one. Anyway, to support a wide selection of
	video formats, I decided to use libmpv (<a href="">source</a>).
</p>

<style lang="scss">
	.title {
		margin-bottom: 0.2em;
	}

	.date {
		margin-top: 0;
		font-size: 0.8em;
	}
</style>
