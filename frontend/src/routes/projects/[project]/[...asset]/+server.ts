import { env } from '$env/dynamic/private';

import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ params }) => {
	let content = await fetch(
		`${env.API_SERVER_ADDRESS}/api/projects/assets/${params.project}/${params.asset}`
	);
	return new Response(content.body, {
		headers: content.headers
	});
};
