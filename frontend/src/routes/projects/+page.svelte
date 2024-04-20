<script lang="ts">
	import { GITHUB } from '$lib/consts';
	import Link from '$lib/components/Link.svelte';
	import Rule from '$lib/components/Rule.svelte';

	import GithubLogo from 'phosphor-svelte/lib/GithubLogo';
	import LinkIcon from 'phosphor-svelte/lib/Link';

	type Project = {
		name: string;
		description: string;
		date: string;
		link: string | null;
		github: string | null;
	};

	// TODO: Use consistant date format in projects and writing
	let projects: Project[] = [
		{
			name: 'ridgehacks2024',
			description:
				"Website for Ridge Computer Science Club's yearly hackathon, Ridgehacks! Written as a single page Svelte app.",
			date: 'February 2, 2024',
			link: 'https://ridgehacks.us',
			github: 'https://github.com/Basicprogrammer10/ridgehacks2024'
		},
		{
			name: 'dm42',
			description:
				"Program compiler for the Free42 HP42 emulator and some miscellaneous programs. Many of which are for my AP Statistics class, because I just can't be normal and use a TI-84.",
			date: 'July 16, 2023',
			link: null,
			github: 'https://github.com/Basicprogrammer10/dm42'
		}
	];
</script>

<h1>Projects</h1>

<p>
	You can find almost all of my projects on my Github, <Link href={GITHUB}>Basicprogrammer10</Link>.
	Some of my more interesting or complete projects will be put here, with articles explaining what
	they do and how they work.
</p>

<hr />

{#each projects as project, idx}
	{#if idx != 0}
		<Rule thickness="thin" />
	{/if}

	<div class="project">
		<div class="title-container">
			<div class="left">
				<h3 class="title">{project.name}</h3>
				<span class="date">Created {project.date}</span>
			</div>

			<div class="right">
				{#if project.link != null}
					<Link href={project.link}>
						<LinkIcon size={24} color="var(--text-color)" class="icon" />
					</Link>
				{/if}
				{#if project.github != null}
					<Link href={project.github}>
						<GithubLogo size={24} color="var(--text-color)" class="icon" />
					</Link>
				{/if}
			</div>
		</div>

		<p>{project.description}</p>
	</div>
{/each}

<style lang="scss">
	.title-container {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-top: 18.72px;

		& .left {
			& .title {
				margin-bottom: 0.2em;
				margin-top: 0;
			}

			& .date {
				margin-top: 0;
				font-size: 0.8em;
			}
		}
	}
</style>
