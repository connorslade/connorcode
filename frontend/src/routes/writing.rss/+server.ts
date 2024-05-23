import { env } from '$env/dynamic/private';

import type { RequestHandler } from './$types';

export const GET: RequestHandler = async () => {
	let content = await fetch(`${env.API_SERVER_ADDRESS}/api/writing/rss`);
	return new Response(content.body, {
		headers: content.headers
	});
};
