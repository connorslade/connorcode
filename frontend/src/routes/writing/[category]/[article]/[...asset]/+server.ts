import { API_SERVER_ADDRESS } from '$env/static/private';

import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ params }) => {
	let path = `${params.category}/${params.article}`;
	let content = await fetch(`${API_SERVER_ADDRESS}/api/writing/assets/${path}/${params.asset}`);
	return new Response(content.body, {
		headers: content.headers
	});
};
