import type { PageServerLoad } from './$types';
import type { ArticleInfo } from '$lib/types';

import { API_SERVER_ADDRESS } from '$env/static/private';

export const load: PageServerLoad = async ({
	params
}): Promise<{ articles: ReadonlyArray<ArticleInfo> }> => {
	const articles: ArticleInfo[] = await (
		await fetch(`${API_SERVER_ADDRESS}/api/writing/list`)
	).json();
	return { articles };
};
