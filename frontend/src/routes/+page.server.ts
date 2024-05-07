import type { PageServerLoad } from './$types';
import type { ArticleInfo, ProjectInfo } from '$lib/types';

import { API_SERVER_ADDRESS } from '$env/static/private';

const ARTICLE_COUNT = 3;
const PROJECT_COUNT = 3;

export const load: PageServerLoad = async ({
	params
}): Promise<{ articles: ReadonlyArray<ArticleInfo>; projects: ReadonlyArray<ProjectInfo> }> => {
	const articles: ArticleInfo[] = await (
		await fetch(`${API_SERVER_ADDRESS}/api/writing/list`)
	).json();
	const projects: ProjectInfo[] = await (
		await fetch(`${API_SERVER_ADDRESS}/api/projects/list`)
	).json();

	articles.splice(ARTICLE_COUNT);
	projects.splice(PROJECT_COUNT);

	return { articles, projects };
};
