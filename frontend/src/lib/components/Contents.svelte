<script lang="ts">
	import { Heading } from '$lib/types';
	import InnerLink from './InnerLink.svelte';

	export let contents: Heading[];
</script>

<div class="contents">
	<ul>
		{#each contents as header}
			<li style:margin-left={`${header.level - 1}em`}>
				<InnerLink href={`#${header.id}`}>{header.text}</InnerLink>
			</li>
		{/each}
	</ul>
</div>

<style lang="scss">
	.contents {
		position: fixed;
		transform: translateX(-100%) translateX(-50px);
	}

    @media (width <= 1200px) {
        .contents {
            display: none;
        }
    }
</style>
