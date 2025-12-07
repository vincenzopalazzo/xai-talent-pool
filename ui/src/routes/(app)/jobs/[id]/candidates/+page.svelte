<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import TalentCard from '$lib/components/talent-card.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import * as Card from '$lib/components/ui/card';
	import { RefreshCw } from 'lucide-svelte';
	import {
		rankCandidatesForJob,
		type RankedCandidate
	} from '$lib/services/ranking';
	import type { Talent } from '$lib/types';

	// Example: Get job ID from URL
	const jobId = $derived($page.params.id);

	// State
	let rankedCandidates = $state<RankedCandidate[]>([]);
	let isLoading = $state(true);
	let isReranking = $state(false);
	let job = $state<any>(null);

	/**
	 * Load and rank candidates for the job
	 */
	async function loadAndRankCandidates() {
		if (!jobId) return;

		isLoading = true;

		// Fetch job details from your API
		const jobResponse = await fetch(`/api/v1/jobs/${jobId}`);
		job = await jobResponse.json();

		// Fetch all candidates/talents from your API
		// In a real scenario, you might filter candidates by application status or other criteria
		const candidatesResponse = await fetch(`/api/v1/talents`);
		const allCandidates: Talent[] = await candidatesResponse.json();

		// Rank candidates using GRPO algorithm
		rankedCandidates = await rankCandidatesForJob(jobId, allCandidates, job);

		isLoading = false;
	}

	/**
	 * Rerank candidates
	 */
	async function rerank() {
		isReranking = true;
		await loadAndRankCandidates();
		isReranking = false;
	}

	// Load candidates on mount
	onMount(() => {
		loadAndRankCandidates();
	});
</script>

<div class="container mx-auto py-8">
	<div class="mb-6">
		<div class="flex items-center justify-between">
			<div>
				<h1 class="text-3xl font-bold">Ranked Candidates</h1>
				{#if job}
					<p class="text-muted-foreground mt-1">{job.title} at {job.company_name}</p>
				{/if}
			</div>
			<div class="flex items-center gap-2">
				<Button variant="outline" size="sm" onclick={rerank} disabled={isReranking}>
					<RefreshCw class="h-4 w-4 mr-2 {isReranking ? 'animate-spin' : ''}" />
					Re-rank
				</Button>
			</div>
		</div>
	</div>

	{#if isLoading}
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
			{#each Array(6) as _}
				<div class="animate-pulse">
					<Card.Root>
						<Card.Header>
							<div class="h-12 bg-muted rounded"></div>
						</Card.Header>
						<Card.Content>
							<div class="space-y-2">
								<div class="h-4 bg-muted rounded w-3/4"></div>
								<div class="h-4 bg-muted rounded w-1/2"></div>
							</div>
						</Card.Content>
					</Card.Root>
				</div>
			{/each}
		</div>
	{:else if rankedCandidates.length === 0}
		<Card.Root>
			<Card.Content class="py-12 text-center">
				<p class="text-muted-foreground">No candidates found for this position.</p>
			</Card.Content>
		</Card.Root>
	{:else}
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
			{#each rankedCandidates as { candidate, rank_position, confidence, match_factors } (candidate.id)}
				<div class="relative">
					{#if rank_position <= 3}
						<div class="absolute -top-2 -left-2 z-10">
							<Badge
								variant={rank_position === 1 ? 'default' : 'secondary'}
								class="h-8 w-8 rounded-full flex items-center justify-center p-0"
							>
								{rank_position}
							</Badge>
						</div>
					{/if}
					<TalentCard talent={candidate} />
					<div class="mt-2 text-xs text-muted-foreground">
						<div class="flex justify-between">
							<span>Skills: {(match_factors.skills_match * 100).toFixed(0)}%</span>
							<span>Confidence: {(confidence * 100).toFixed(0)}%</span>
						</div>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>
