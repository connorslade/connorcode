<script lang="ts">
	import type { PageData } from './$types';

	import Head from '$lib/components/Head.svelte';
	import Rule from '$lib/components/Rule.svelte';
	import Title from '$lib/components/Title.svelte';

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
	you can subscribe to the <a href="writing.rss">RSS feed</a>.
</p>

<hr />

{#each data.articles as article, idx}
	{#if idx != 0}
		<Rule thickness="thin" />
	{/if}
	<div class="project">
		<a href={`writing/${article.path}`} class="project-link">
			<div class="title-container">
				<div class="left">
					<Title
						title_element="h3"
						title={article.title}
						info={`Published ${article.date}`}
						title_style="margin-top: 0"
					/>
				</div>

				<div class="right"></div>
			</div>
		</a>

		{@html article.description}
	</div>
{/each}

<style lang="scss">
	.project-link {
		text-decoration: none;
	}

	.title-container {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-top: 18.72px;
	}
</style>
