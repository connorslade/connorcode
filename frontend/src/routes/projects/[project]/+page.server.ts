import type { PageServerLoad } from './$types';
import type { ProjectInfo } from '$lib/types';

import { API_SERVER_ADDRESS } from '$env/static/private';

type Response = {
	html: string;
	info: ProjectInfo;
};

export const load: PageServerLoad = async ({ params }): Promise<Response> => {
	const html = await (
		await fetch(`${API_SERVER_ADDRESS}/api/projects/article/${params.project}`)
	).text();
	const info: ProjectInfo = await (
		await fetch(`${API_SERVER_ADDRESS}/api/projects/info/${params.project}`)
	).json();

	return { info, html };
};
