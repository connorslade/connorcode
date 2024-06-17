<script lang="ts">
	import type { PageData } from './$types';
	import { NaiveDate } from '$lib/date';

	import { GITHUB } from '$lib/consts';
	import Link from '$lib/components/Link.svelte';
	import Rule from '$lib/components/Rule.svelte';
	import Title from '$lib/components/Title.svelte';

	import GithubLogo from 'phosphor-svelte/lib/GithubLogo';
	import LinkIcon from 'phosphor-svelte/lib/Link';
	import { PushPin } from 'phosphor-svelte';
	import Head from '$lib/components/Head.svelte';

	export let data: PageData;
	$: dates = data.projects.map((project) => new NaiveDate(project.date));
</script>

<Head
	title="Projects"
	description="List of articles on some of my more interesting or complete projects explaining what they
	do and how they work."
/>

<h1>Projects</h1>

<p>
	You can find almost all of my projects on my Github, <Link href={GITHUB}>@connorslade</Link>. Some
	of my more interesting or complete projects will be put here, with articles explaining what they
	do and how they work.
</p>

<p>
	Note that because this version of my website is still a work in progress, I have not yet added all
	of my interesting projects here.
</p>

<hr />

{#each data.projects as project, idx}
	{#if idx != 0}
		<Rule thickness="thin" />
	{/if}

	<div class="project">
		<div class="title-container">
			<a href={`/projects/${project.slug}`} class="project-link">
				<div class="left">
					<Title
						title_element="h3"
						title={project.name}
						info={`Created ${dates[idx].human_date()}`}
						title_style="margin-top: 0"
					>
						<div slot="title-end" class="pin-icon">
							{#if project.pinned}
								<PushPin size={20} color="var(--text-color)" class="push-pin" />
							{/if}
						</div>
					</Title>
				</div>
			</a>

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

		{@html project.description}
	</div>
{/each}

<style lang="scss">
	.title-container {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-top: 18.72px;
		gap: 15px;

		& .project-link {
			text-decoration: none;
			width: 100%;
		}

		& .right {
			display: flex;
			gap: 8px;
		}
	}

	.pin-icon {
		display: inline;
		margin-left: 4px;
	}

	:global(.push-pin) {
		position: absolute;
	}
</style>
