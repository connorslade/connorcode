<script lang="ts">
	import type { PageData } from './$types';

	import Head from '$lib/components/Head.svelte';

	class NaiveDate {
		month: number;
		day: number;
		year: number;

		constructor(date: string) {
			let parts = date.split('/');
			this.month = parseInt(parts[0]);
			this.day = parseInt(parts[1]);
			this.year = parseInt(parts[2]);
		}
	}

	const MONTHS: string[] = [
		'January',
		'February',
		'March',
		'April',
		'May',
		'June',
		'July',
		'August',
		'September',
		'October',
		'November',
		'December'
	];

	export let data: PageData;

	$: dates = data.articles.map((article) => new NaiveDate(article.date));
</script>

<Head title="Writing" description="todo" />

<h1>Writing</h1>

<p>
	Here are some random things I have written about. To get notified when new posts are published,
	you can subscribe to the <a href="/">RSS feed</a>.
</p>

{#each data.articles as article, idx}
	{#if idx == 0 || dates[idx].year != dates[idx - 1].year}
		<h2>{dates[idx].year}</h2>
	{/if}

	{#if idx == 0 || dates[idx].month != dates[idx - 1].month}
		<h3>{MONTHS[dates[idx].month - 1]}</h3>
	{/if}

	<li>
		<a href={`/writing/${article.path}`}>{article.title}</a>
		&mdash; {article.description}
	</li>
{/each}

<style lang="scss">
	li {
		margin-left: 40px;
	}
</style>
