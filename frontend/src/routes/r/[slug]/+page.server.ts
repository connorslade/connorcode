import type { PageServerLoad } from './$types';
import { env } from '$env/dynamic/private';
import { error, redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ params }): Promise<{}> => {
	let content = await fetch(`${env.API_SERVER_ADDRESS}/api/redirect/${params.slug}`);
	if (content.status == 200) redirect(301, await content.text());
	else error(404, 'Redirect not found');
};
