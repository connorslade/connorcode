<script lang="ts">
	import type { ComponentType } from 'svelte';
	import type { ChildNode } from 'domhandler';
	import { ElementType } from 'htmlparser2';

	type Component = {
		component: ComponentType;
		props: Record<string, any>;
		children: ChildNode[];
	};

	function callback_wrapper(callback: (node: ChildNode) => ChildNode | Component) {
		return (node: ChildNode) => {
			return callback(node) ?? node;
		};
	}

	export let nodes: (ChildNode | Component)[];
	export let element_callback: (node: ChildNode) => ChildNode | Component;
	export let run_callbacks: boolean = true;
</script>

<!-- just ignore this -->
{#each nodes as node}{#if 'component' in node}<svelte:component
			this={node.component}
			{...node.props}><svelte:self nodes={node.children} {element_callback} /></svelte:component
		>{:else if node.type == ElementType.Tag || node.type == ElementType.Script || node.type == ElementType.Style}{#if run_callbacks}<svelte:self
				nodes={[callback_wrapper(element_callback)(node)]}
				{element_callback}
				run_callbacks={false}
			/>{:else if node.children.length == 0}<svelte:element
				this={node.name}
				{...node.attribs}
			/>{:else}<svelte:element this={node.name} {...node.attribs}
				><svelte:self nodes={node.children} {element_callback} /></svelte:element
			>{/if}{:else if node.type == ElementType.Text}{node.data}{/if}{/each}

<!-- {#each nodes as node}
	{#if 'component' in node}
		<svelte:component this={node.component} {...node.props}>
			<svelte:self nodes={node.children} {element_callback} />
		</svelte:component>
	{:else if node.type == ElementType.Tag || node.type == ElementType.Script || node.type == ElementType.Style}
		{#if run_callbacks}
			<svelte:self
				nodes={[callback_wrapper(element_callback)(node)]}
				{element_callback}
				run_callbacks={false}
			/>
		{:else if node.children.length == 0}
			<svelte:element this={node.name} {...node.attribs} />
		{:else}
			<svelte:element this={node.name} {...node.attribs}>
				<svelte:self nodes={node.children} {element_callback} />
			</svelte:element>
		{/if}
	{:else if node.type == ElementType.Text}
		{node.data}
	{/if}
{/each} -->
