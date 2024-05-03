import type { PageServerLoad } from './$types';
import type { ArticleInfo } from '$lib/types';

import { API_SERVER_ADDRESS } from '$env/static/private';

const ARTICLE_COUNT = 3;
const PROJECT_COUNT = 0;

export const load: PageServerLoad = async ({
	params
}): Promise<{ articles: ReadonlyArray<ArticleInfo> }> => {
	const articles: ArticleInfo[] = await (
		await fetch(`${API_SERVER_ADDRESS}/api/writing/list`)
	).json();

	articles.splice(ARTICLE_COUNT);
	return { articles };
};
