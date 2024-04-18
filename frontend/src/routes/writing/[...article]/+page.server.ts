import type { PageServerLoad } from './$types';

type Info = {
	date: string;
	description: string;
	path: string;
	tags: string[];
	title: string;
	word_count: number;
};

type Response = {
	html: string;
	info: Info;
};

export const load: PageServerLoad = async ({ params }): Promise<Response> => {
	const html = await (
		await fetch(`http://localhost:8080/api/writing/article/${params.article}`)
	).text();
	const info: Info = await (
		await fetch(`http://localhost:8080/api/writing/article/info/${params.article}`)
	).json();

	return { info, html };
};
