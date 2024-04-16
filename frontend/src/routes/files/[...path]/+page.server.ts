import type { PageServerLoad } from './$types';

type DirResponse = {
	children: [DirEntry];
};

type DirEntry = {
	path: string;
	name: string;
	is_dir: boolean;
	size: number;
	last_modified: number;
};

export const load: PageServerLoad = async ({ params }) => {
	return (await (
		await fetch(`http://localhost:8080/api/files/${params.path}`)
	).json()) as DirResponse;
};
