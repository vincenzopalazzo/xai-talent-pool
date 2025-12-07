import type { PageLoad } from './$types';
import type { Talent } from '$lib/types';

export const load = (async () => {
  try {
    const response = await fetch('http://localhost:8080/api/v1/talents');
    if (!response.ok) {
      throw new Error('Failed to fetch talents');
    }
    const talents: Talent[] = await response.json();
    return { talents };
  } catch (error) {
    console.error('Error fetching talents:', error);
    return { talents: [] as Talent[] };
  }
}) satisfies PageLoad;