<script lang="ts">
	import Link from '$lib/Link.svelte';
	import { GITHUB, VERSION } from '$lib/consts';

	import '../styles/main.scss';
</script>

<div class="root">
	<div class="nav">
		<a class="name" href="/">Connor Slade</a>

		<div class="links">
			<a href="/">home</a>
			&bull;
			<a href="/projects">projects</a>
			&bull;
			<a href="/writing">writing</a>
			&bull;
			<a href="/files">files</a>
			&bull;
			<Link href={GITHUB}>github</Link>
		</div>
	</div>

	<div class="content">
		<slot />
	</div>

	<hr class="footer-rule" />

	<div class="footer">
		&copy; Connor Slade 2024 &bull; v{VERSION}
		&bull;
		<Link href={GITHUB}>Github</Link>
	</div>
</div>

<style lang="scss">
	.root {
		max-width: 40rem;
		margin-left: auto;
		margin-right: auto;
	}

	.nav {
		display: flex;
		justify-content: space-between;
		margin-top: 2rem;
		flex-wrap: wrap;

		& .name {
			margin: 0;
			font-size: 1.2em;
			color: var(--alternate-text-color);
			text-decoration: none;

			&:hover {
				text-decoration: underline;
			}
		}
	}

	.footer-rule {
		margin-top: 2em;
		margin-bottom: 1.5em;
	}

	.footer {
		margin-bottom: 1.5em;
	}

	@media (width <= 510px) {
		.nav {
			align-items: center;
			flex-flow: column;

			& .name {
				margin-bottom: 1rem;
			}
		}
	}
</style>
