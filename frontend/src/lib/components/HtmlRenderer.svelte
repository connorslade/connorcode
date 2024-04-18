<script lang="ts">
	import { SvelteComponent, onMount } from 'svelte';
	import Admonition from './Admonition.svelte';

	function init() {
		let to_destroy: SvelteComponent[] = [];

		let blockquotes = document.querySelectorAll('blockquote');
		for (let blockquote of blockquotes) {
			let match = blockquote.childNodes[1].textContent?.match(/\[(.*)\] (.*)/);
			if (match == null || match == undefined) continue;

			let type = match[1];
			let title = match[2];

			let content = '';
			for (let i = 2; i < blockquote.childNodes.length; i++) {
				let value = blockquote.childNodes[i] as HTMLQuoteElement;
				if (value.outerHTML == undefined) continue;
				content += value.outerHTML;
			}

			let container = document.createElement('div');
			blockquote.replaceWith(container);
			let admonition = new Admonition({
				target: container,
				props: {
					title,
					type,
					content
				}
			});

			to_destroy.push(admonition);
		}

		return () => {
			for (let component of to_destroy) component.$destroy();
		};
	}

	onMount(init);

	export let html: string;
</script>

<!-- {@html html} -->
{@html `<Admonition type="tip" title="It works?">Hello</Admonition>`}
