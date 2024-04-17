<script lang="ts">
	type Style = 'solid' | 'dashed';

	export let style: Style = 'solid';
</script>

<hr style:border={`1px ${style}`} />
