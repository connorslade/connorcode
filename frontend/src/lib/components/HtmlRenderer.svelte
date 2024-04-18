<script lang="ts">
	import { Html, isTag, Element } from 'html-svelte-parser';

	import Admonition from './Admonition.svelte';

	function process_node(node: Element): any {
		if (node.name == 'div' && node.attribs['element'] == 'admonition') {
			return {
				component: Admonition,
				props: {
					type: node.attribs['type'],
					title: node.attribs['title']
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
