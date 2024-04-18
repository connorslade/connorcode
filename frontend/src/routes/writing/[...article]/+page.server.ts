import type { PageServerLoad } from './$types';
import type { ArticleInfo } from '$lib/types';

import { API_SERVER_ADDRESS } from '$env/static/private';

type Response = {
	html: string;
	info: ArticleInfo;
};

export const load: PageServerLoad = async ({ params }): Promise<Response> => {
	const html = await (
		await fetch(`${API_SERVER_ADDRESS}/api/writing/article/${params.article}`)
	).text();
	const info: ArticleInfo = await (
		await fetch(`${API_SERVER_ADDRESS}/api/writing/article/info/${params.article}`)
	).json();

	return { info, html };
};
