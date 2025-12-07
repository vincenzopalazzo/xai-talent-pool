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

export interface FeedbackStats {
	talent_id: string;
	job_id?: string;
	upvotes: number;
	downvotes: number;
	net_score: number;
	total_feedback: number;
}

export interface Feedback {
	id: string;
	talent_id: string;
	job_id: string;
	recruiter_id?: string;
	feedback_type: 'upvote' | 'downvote';
	expected_rank?: number;
	actual_performance: string;
	notes?: string;
	created_at: string;
}

/**
 * Rank candidates for a job using GRPO algorithm
 */
export async function rankCandidatesForJob(
	jobId: string,
	candidates: Talent[],
	job: any,
	useFeedback: boolean = true,
	feedbackData?: Feedback[]
): Promise<RankedCandidate[]> {
	const response = await fetch(`${GROK_SERVICE_URL}/api/v1/ranking/rank`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({
			job,
			candidates,
			feedback_data: feedbackData,
			use_feedback: useFeedback
		})
	});

	if (!response.ok) {
		throw new Error(`Failed to rank candidates: ${response.statusText}`);
	}

	return await response.json();
}

/**
 * Submit RLHF feedback for a candidate
 */
export async function submitFeedback(
	talentId: string,
	jobId: string,
	feedbackType: 'upvote' | 'downvote',
	expectedRank?: number,
	recruiterId?: string,
	notes?: string
): Promise<Feedback> {
	// Assumes external feedback API endpoint
	const response = await fetch(`${BACKEND_API_URL}/api/v1/feedback`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({
			talent_id: talentId,
			job_id: jobId,
			feedback_type: feedbackType,
			expected_rank: expectedRank,
			recruiter_id: recruiterId,
			notes
		})
	});

	if (!response.ok) {
		throw new Error(`Failed to submit feedback: ${response.statusText}`);
	}

	return await response.json();
}

/**
 * Get feedback statistics for a candidate
 */
export async function getFeedbackStats(
	talentId: string,
	jobId?: string
): Promise<FeedbackStats> {
	const url = new URL(`${BACKEND_API_URL}/api/v1/feedback/talent/${talentId}/stats`);
	if (jobId) {
		url.searchParams.set('job_id', jobId);
	}

	const response = await fetch(url.toString());

	if (!response.ok) {
		throw new Error(`Failed to get feedback stats: ${response.statusText}`);
	}

	return await response.json();
}

/**
 * Get all feedback for a job (used for GRPO training)
 */
export async function getFeedbackForJob(jobId: string): Promise<Feedback[]> {
	const response = await fetch(`${BACKEND_API_URL}/api/v1/feedback/job/${jobId}`);

	if (!response.ok) {
		throw new Error(`Failed to get job feedback: ${response.statusText}`);
	}

	return await response.json();
}

/**
 * Update GRPO model weights based on feedback batch
 */
export async function updateRankingWeights(feedbackBatch: Feedback[]): Promise<{
	status: string;
	message: string;
	current_weights: Record<string, number>;
	model_version: string;
}> {
	const response = await fetch(`${GROK_SERVICE_URL}/api/v1/ranking/update-weights`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({
			feedback_batch: feedbackBatch
		})
	});

	if (!response.ok) {
		throw new Error(`Failed to update weights: ${response.statusText}`);
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
