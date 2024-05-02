<script lang="ts">
	import * as htmlparser2 from 'htmlparser2';
	import type { ChildNode } from 'domhandler';
	import { Element } from 'domhandler';

	import Admonition from './Admonition.svelte';
	import InnerLink from './InnerLink.svelte';
	import Link from './Link.svelte';
	import Contents from './Contents.svelte';

	import { Heading } from '$lib/types';
	import HtmlRendererInner from './HtmlRendererInner.svelte';
	import { onMount } from 'svelte';

	const HEADING_TAGS = ['h1', 'h2', 'h3', 'h4', 'h5', 'h6'];
	let contents: Heading[] = [];
	let lowest_heading = 6;

	function walk_document(node: ChildNode) {
		if (!(node instanceof Element)) return;
		let element = node as Element;

		let heading = HEADING_TAGS.indexOf(node.name);
		if (heading != -1) {
			let id = (element.children[0] as any).attribs['id'];
			let text = '';
			for (let child of element.children) {
				if (child.type == 'text') {
					text = child.data;
					break;
				}
			}
			lowest_heading = Math.min(lowest_heading, heading);
			contents.push(new Heading(heading + 1, text, id));
		}

		for (let child of element.children) walk_document(child);
	}

	function process_node(node: ChildNode): any {
		if (!(node instanceof Element)) return;

		if (node.name == 'div' && node.attribs['element'] == 'admonition') {
			return {
				component: Admonition,
				props: {
					type: node.attribs['type'],
					title: node.attribs['title']
				},
				children: node.children
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
					},
					children: node.children
				};
			else if (href.startsWith('http'))
				return {
					component: Link,
					props: {
						href
					},
					children: node.children
				};
			else {
				node.attribs['href'] = `${base}/${href}`;
				return;
			}
		} else if ('src' in node.attribs) {
			node.attribs['src'] = `${base}/${node.attribs['src']}`;
		} else if (node.name == 'script') {
			let code = '';
			for (let child of node.children) if (child.type == 'text') code += child.data;

			onMount(() => {
				eval(`(() => {${code}})()`);
			});
		}
	}

	export let html: string;
	export let base: string | undefined = undefined;

	let parser = htmlparser2.parseDocument(html);
	for (let node of parser.childNodes) walk_document(node);
	contents.forEach((header) => (header.level -= lowest_heading));
</script>

<Contents {contents} />

<HtmlRendererInner nodes={parser.childNodes} element_callback={process_node} />
