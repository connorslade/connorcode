<script lang="ts">
	import { Html, isTag, Element } from 'html-svelte-parser';

	import Admonition from './Admonition.svelte';
	import InnerLink from './InnerLink.svelte';
	import Link from './Link.svelte';
	import { onMount } from 'svelte';

	let src_scripts: string[] = [];
	let inline_scripts: string[] = [];

	function process_node(node: Element): any {
		if (node.name == 'div' && node.attribs['element'] == 'admonition') {
			return {
				component: Admonition,
				props: {
					type: node.attribs['type'],
					title: node.attribs['title']
				}
			};
		} else if (node.name == 'a') {
			let href = node.attribs['href'];
			if (href.startsWith('#'))
				return {
					component: InnerLink,
					props: {
						href,
						id: node.attribs['id'],
						_class: node.attribs['class'],
						aria_hidden: node.attribs['aria-hidden']
					}
				};
			else if (href.startsWith('http'))
				return {
					component: Link,
					props: {
						href
					}
				};
			else if (!href.startsWith('/')) node.attribs['href'] = base + href;
		} else if (node.name == 'script') {
			if ('src' in node.attribs) src_scripts.push(node.attribs['src']);
			else inline_scripts.push((node.children[0] as any).data);
			return;
		} else if ('src' in node.attribs && !node.attribs['src'].startsWith('http')) {
			node.attribs['src'] = base + node.attribs['src'];
		}
	}

	onMount(() => {
		(window as any).BASE_PATH = base;

		for (let src of src_scripts) {
			let script = document.createElement('script');
			script.src = base + src;
			document.body.appendChild(script);
		}

		for (let script of inline_scripts) {
			let script_tag = document.createElement('script');
			script_tag.text = `(() => {${script}})()`;
			document.body.appendChild(script_tag);
		}
	});

	export let html: string;
	export let base: string | undefined = undefined;
</script>

<Html
	{html}
	processNode={(node) => {
		if (isTag(node)) return process_node(node);
	}}
/>
