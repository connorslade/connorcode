<script lang="ts">
	function handleClick(event: MouseEvent) {
		event.preventDefault();

		let target = event.target as HTMLAnchorElement;
		let anchorId = new URL(target.href).hash.slice(1);
		let anchor = document.getElementById(anchorId);
		if (anchor) anchor.scrollIntoView({ behavior: 'smooth' });
	}

	export let href: string;

	export let id: string | null = null;
	export let _class: string | null = null;
	export let aria_hidden: boolean = false;
</script>

<a {href} {id} class={_class} aria-hidden={aria_hidden} on:click={handleClick}>
	<slot />
</a>
