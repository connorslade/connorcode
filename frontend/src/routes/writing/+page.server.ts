import type { PageServerLoad } from './$types';
import type { ArticleInfo } from '$lib/types';

import { env } from '$env/dynamic/private';

export const load: PageServerLoad = async ({
	params
}): Promise<{ articles: ReadonlyArray<ArticleInfo> }> => {
	const raw_articles: ArticleInfo[] = await (
		await fetch(`${env.API_SERVER_ADDRESS}/api/writing/list`)
	).json();
	const articles = raw_articles.filter((article) => {
		let is_article = article.path.includes('/') && !article.path.startsWith('/');
		return is_article || article.hidden === false;
	});

	return { articles };
};
