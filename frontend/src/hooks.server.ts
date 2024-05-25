import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	return await resolve(event, {
		preload: ({ type }) => type === 'css' || type === 'js' || type === 'font'
	});
};
