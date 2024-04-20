import { API_SERVER_ADDRESS } from '$env/static/private';

import type { RequestHandler } from './$types';

// so cursed
export const GET: RequestHandler = async ({ params }) => {
	let content = await fetch(`${API_SERVER_ADDRESS}/api/writing/assets/${params.path}`);
	return new Response(content.body, {
		headers: content.headers
	});
};
