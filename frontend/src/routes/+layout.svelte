<script lang="ts">
	import { onMount } from 'svelte';
	import { navigating } from '$app/stores';
	import { dev } from '$app/environment';

	import Link from '$lib/components/Link.svelte';
	import { GITHUB, VERSION } from '$lib/consts';

	import '../styles/main.scss';

	function current_year(): number {
		return Math.max(new Date().getFullYear(), 2026);
	}

	function send_analytics(data: {
		page: string;
		referrer: string | null;
		user_agent: string | null;
	}) {
		if (dev) return;
		fetch(`/api/analytics`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(data)
		}).catch((err) => console.error('Analytics Error', err));
	}

	onMount(() => {
		send_analytics({
			page: window.location.pathname,
			referrer: document.referrer === '' ? null : document.referrer,
			user_agent: navigator.userAgent === '' ? null : navigator.userAgent
		});
	});

	let loader: HTMLDivElement;
	let loader_value = 0;
	let loader_interval: NodeJS.Timeout | null = null;

	let last_page = '';
	navigating.subscribe((navigating) => {
		if (loader != null && loader_interval == null) {
			loader.style.transition = 'none';
			loader.style.width = '0%';
			loader.style.opacity = '1';
			loader_interval = setInterval(() => {
				loader_value += 1;
				if (loader_value < 10) return;
				const k = 80,
					w = 100;
				let value = Math.min((2 * k) / (1 + Math.exp(-loader_value / w)) - k, k);
				loader.style.width = `${value}%`;
			}, 10);
		}

		navigating?.complete.then(() => {
			if (loader != null && loader_interval != null) {
				if (loader_value) loader.style.transition = 'width 1s, opacity 1s';
				loader.style.width = '100%';
				loader.style.opacity = '0';
				clearInterval(loader_interval);
				loader_interval = null;
				loader_value = 0;
			}

			let page = navigating.to?.url!.pathname!;
			if (page === last_page) return;
			last_page = page;
			send_analytics({
				page,
				referrer: navigating.from?.url.toString() ?? null,
				user_agent: navigator.userAgent === '' ? null : navigator.userAgent
			});
		});
	});
</script>

<div class="root">
	<div class="loader no-print" bind:this={loader}></div>

	<div class="nav no-print">
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
		&copy; Connor Slade {current_year()} &bull; v{VERSION}
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

	.loader {
		position: fixed;
		top: 0;
		left: 0;
		height: 2px;
		background-color: var(--code-color);
		overflow: hidden;
		transition:
			width 1s,
			opacity 1s;
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
