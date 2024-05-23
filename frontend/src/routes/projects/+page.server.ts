import type { PageServerLoad } from './$types';
import type { ProjectInfo } from '$lib/types';

import { env } from '$env/dynamic/private';

export const load: PageServerLoad = async ({
	params
}): Promise<{ projects: ReadonlyArray<ProjectInfo> }> => {
	const projects: ProjectInfo[] = await (
		await fetch(`${env.API_SERVER_ADDRESS}/api/projects/list`)
	).json();

	projects.sort((a, b) => (a.pinned === b.pinned ? 0 : a.pinned ? -1 : 1));

	return { projects };
};
