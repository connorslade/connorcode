import type { Handle } from '@sveltejs/kit';

import { env } from '$env/dynamic/private';
import { dev } from '$app/environment';

export const handle: Handle = async ({ event, resolve }) => {
	if (dev && event.url.pathname.startsWith('/api')) {
		return await fetch(`${env.API_SERVER_ADDRESS}/${event.url.pathname}`, {
			method: event.request.method,
			headers: event.request.headers,
			body: event.request.body,
			duplex: 'half'
		});
	}

	return await resolve(event);
};
