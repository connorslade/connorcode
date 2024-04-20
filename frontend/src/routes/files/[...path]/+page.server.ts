import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

import { API_SERVER_ADDRESS } from '$env/static/private';
import { PUBLIC_FILE_ADDRESS } from '$env/static/public';

type DirResponse = {
	children: [DirEntry];
	readme: string | null;
};

type DirEntry = {
	path: string;
	name: string;
	is_dir: boolean;
	size: number;
	last_modified: number;
};

export const load: PageServerLoad = async ({ params }) => {
	const path = `${API_SERVER_ADDRESS}/api/files/${params.path}`;
	const req = await fetch(`${path}?no_file=true`);
	const res_type = req.headers.get('X-Response-Type');

	if (res_type == 'File') throw redirect(307, `${PUBLIC_FILE_ADDRESS}/${params.path}`);
	else if (res_type == 'DirEntry') return (await req.json()) as DirResponse;
};
