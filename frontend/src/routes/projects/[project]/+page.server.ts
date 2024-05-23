import type { PageServerLoad } from './$types';
import type { ProjectInfo } from '$lib/types';

import { env } from '$env/dynamic/private';

type Response = {
	html: string;
	info: ProjectInfo;
};

export const load: PageServerLoad = async ({ params }): Promise<Response> => {
	const html = await (
		await fetch(`${env.API_SERVER_ADDRESS}/api/projects/article/${params.project}`)
	).text();
	const info: ProjectInfo = await (
		await fetch(`${env.API_SERVER_ADDRESS}/api/projects/info/${params.project}`)
	).json();

	return { info, html };
};
