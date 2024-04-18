import type { PageServerLoad } from './$types';

import { htmlToText } from 'html-to-text';

type Info = {
	date: string;
	description: string;
	path: string;
	tags: string[];
	title: string;
};

type Response = {
	html: string;
	info: Info;
	word_count: number;
};

export const load: PageServerLoad = async ({ params }): Promise<Response> => {
	const html = await (
		await fetch(`http://localhost:8080/api/writing/article/${params.article}`)
	).text();
	const info: Info = await (
		await fetch(`http://localhost:8080/api/writing/article/info/${params.article}`)
	).json();

	// TODO: Run this on backend so it can be cached
	let text = htmlToText(html, {
		wordwrap: false,
		ignoreImage: true,
		ignoreHref: true
	});

	return { info, html, word_count: text.split(' ').length };
};
