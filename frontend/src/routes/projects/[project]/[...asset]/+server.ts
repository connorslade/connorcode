import { API_SERVER_ADDRESS } from '$env/static/private';

import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ params }) => {
	let content = await fetch(
		`${API_SERVER_ADDRESS}/api/projects/assets/${params.project}/${params.asset}`
	);
	return new Response(content.body, {
		headers: content.headers
	});
};
