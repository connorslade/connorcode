import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params }) => {
	const path = `http://localhost:8080/api/writing/article/${params.article}`;
	const html = await(await fetch(path)).text();
	return { html };
};
