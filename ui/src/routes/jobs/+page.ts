import type { PageLoad } from './$types';
import type { Job } from '$lib/types';

export const load = (async () => {
	try {
		const response = await fetch('http://localhost:8080/api/v1/jobs');
		if (!response.ok) {
			throw new Error('Failed to fetch jobs');
		}
		const jobs: Job[] = await response.json();
		return { jobs };
	} catch (error) {
		console.error('Error fetching jobs:', error);
		return { jobs: [] as Job[] };
	}
}) satisfies PageLoad;
