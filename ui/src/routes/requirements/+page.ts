import type { PageLoad } from './$types';
import type { HiringRequirement, Job } from '$lib/types';

export const load: PageLoad = async ({ fetch }) => {
	const [requirementsRes, jobsRes] = await Promise.all([
		fetch('http://localhost:8080/api/v1/hiring-requirements'),
		fetch('http://localhost:8080/api/v1/jobs')
	]);

	const requirements: HiringRequirement[] = requirementsRes.ok ? await requirementsRes.json() : [];
	const jobs: Job[] = jobsRes.ok ? await jobsRes.json() : [];

	return {
		requirements,
		jobs
	};
};
