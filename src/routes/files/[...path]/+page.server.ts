import type { PageServerLoad } from './$types';
import { readdir, stat } from 'fs/promises';
import { join } from 'path';

export const load: PageServerLoad = async ({ params }) => {
	let files;
	try {
		files = await readdir(join('/home/connorslade/Documents', params.path), {
			withFileTypes: true
		});
	} catch (error) {
		return { error: 'Directory not found', code: error.code };
	}

	let out = [];
	const now = Math.floor(new Date().getTime());

	for (let file of files) {
		const info = await stat(join('/home/connorslade/Documents', params.path, file.name));
		out.push({
			path: join(params.path, file.name),
			is_dir: file.isDirectory(),
			name: file.name,
			size: info.size,
			last_modified: now - info.mtimeMs
		});
	}

	return { files: out };
};
