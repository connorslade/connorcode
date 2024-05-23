import type { PageServerLoad } from './$types';
import type { ArticleInfo } from '$lib/types';

import { env } from '$env/dynamic/private';

type Response = {
	html: string;
	info: ArticleInfo;
};

export const load: PageServerLoad = async ({ params }): Promise<Response> => {
	const path = `${params.category}/${params.article}`;
	const html = await (await fetch(`${env.API_SERVER_ADDRESS}/api/writing/article/${path}`)).text();
	const info: ArticleInfo = await (
		await fetch(`${env.API_SERVER_ADDRESS}/api/writing/info/${path}`)
	).json();

	return { info, html };
};
