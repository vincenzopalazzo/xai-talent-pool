/**
 * GRPO Ranking and RLHF Feedback API Service
 *
 * This service provides methods to interact with the GRPO ranking system
 * and submit RLHF feedback for candidates.
 */

import type { Talent } from '$lib/types';

// API Base URLs (configure based on environment)
const GROK_SERVICE_URL = import.meta.env.VITE_GROK_SERVICE_URL || 'http://localhost:8001';
const BACKEND_API_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080';

export interface MatchFactors {
	skills_match: number;
	experience_match: number;
	location_match: number;
	title_match: number;
	overall_fit: number;
}

export interface RankedCandidate {
	candidate: Talent;
	rank_score: number;
	rank_position: number;
	confidence: number;
	match_factors: MatchFactors;
	feedback_score?: number;
}


/**
 * Rank candidates for a job using GRPO algorithm
 */
export async function rankCandidatesForJob(
	jobId: string,
	candidates: Talent[],
	job: any
): Promise<RankedCandidate[]> {
	const response = await fetch(`${GROK_SERVICE_URL}/api/v1/ranking/rank`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({
			job,
			candidates
		})
	});

	if (!response.ok) {
		throw new Error(`Failed to rank candidates: ${response.statusText}`);
	}

	return await response.json();
}

/**
 * Get current ranking model statistics
 */
export async function getRankingStats(): Promise<{
	total_candidates: number;
	avg_confidence: number;
	model_version: string;
	current_weights: Record<string, number>;
}> {
	const response = await fetch(`${GROK_SERVICE_URL}/api/v1/ranking/stats`);

	if (!response.ok) {
		throw new Error(`Failed to get ranking stats: ${response.statusText}`);
	}

	return await response.json();
}
