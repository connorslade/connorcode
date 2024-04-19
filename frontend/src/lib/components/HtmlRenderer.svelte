<script lang="ts">
	import { Html, isTag, Element } from 'html-svelte-parser';

	import Admonition from './Admonition.svelte';
	import InnerLink from './InnerLink.svelte';
	import Link from './Link.svelte';

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
			else if (!href.startsWith('/'))
				return {
					component: Link,
					props: {
						href
					}
				};
		}
	}

	export let html: string;
</script>

<Html
	{html}
	processNode={(node) => {
		if (isTag(node)) return process_node(node);
	}}
/>
