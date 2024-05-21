<script lang="ts">
	import { Lightbulb, Info, Note, Warning, ExclamationMark, Biohazard } from 'phosphor-svelte';

	class Style {
		foreground: string;
		background: string;
		icon: any;

		constructor(foreground: string, background: string, icon: any) {
			this.foreground = foreground;
			this.background = background;
			this.icon = icon;
		}
	}

	// From: https://sveltepress.site/guide/default-theme/admonitions/
	const STYLES: { [key: string]: Style } = {
		tip: new Style('#2ecc71', 'rgb(46 204 113 / 10%)', Lightbulb),
		info: new Style('#7633db', 'rgb(118 51 219 / 10%)', Info),
		note: new Style('#f1c40f', 'rgba(241, 196, 15, .06)', Note),
		warning: new Style('#e74c3c', 'rgb(231 76 60 / 10%)', Warning),
		important: new Style('#3498db', 'rgb(52 152 219 / 10%)', ExclamationMark),
		caution: new Style('#e67e22', 'rgb(230 126 34 / 10%)', Biohazard)
	};

	export let title: string;
	export let type: 'tip' | 'info' | 'note' | 'warning' | 'important' | 'caution';

	let style = STYLES[type];
</script>

<div
	class="admonition"
	style:background-color={style.background}
	style:border-color={style.foreground}
>
	<p class="title" style:color={style.foreground}>
		<svelte:component this={style.icon} size="17px" weight="fill" />
		{title}
	</p>

	<slot />
</div>

<style lang="scss">
	.admonition {
		border-left: solid 8px;
		border-radius: 5px;
		padding: 15px 30px 15px 15px;
		margin-top: 1em;
		margin-bottom: 1em;

		& > :global(:last-child) {
			margin-bottom: 0;
		}

		& .title {
			display: flex;
			gap: 5px;
			font-weight: 700;
			font-size: 14px;
			margin: 0;
		}
	}
</style>
