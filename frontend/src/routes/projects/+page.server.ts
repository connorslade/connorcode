import type { PageServerLoad } from './$types';
import type { ProjectInfo } from '$lib/types';

import { API_SERVER_ADDRESS } from '$env/static/private';

export const load: PageServerLoad = async ({
	params
}): Promise<{ projects: ReadonlyArray<ProjectInfo> }> => {
	const projects: ProjectInfo[] = await (
		await fetch(`${API_SERVER_ADDRESS}/api/projects/list`)
	).json();
	return { projects };
};
