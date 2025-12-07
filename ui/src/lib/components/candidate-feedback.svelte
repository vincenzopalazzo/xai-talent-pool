<script lang="ts">
	import { ThumbsUp, ThumbsDown } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { cn } from '$lib/utils';

	interface Props {
		talentId: string;
		jobId: string;
		expectedRank?: number;
		disabled?: boolean;
		showStats?: boolean;
		class?: string;
	}

	let {
		talentId,
		jobId,
		expectedRank,
		disabled = false,
		showStats = false,
		class: className
	}: Props = $props();

	// State for feedback
	let userFeedback = $state<'upvote' | 'downvote' | null>(null);
	let isSubmitting = $state(false);
	let feedbackStats = $state({ upvotes: 0, downvotes: 0, netScore: 0 });

	// Submit feedback to API
	async function submitFeedback(feedbackType: 'upvote' | 'downvote') {
		if (isSubmitting || disabled) return;

		isSubmitting = true;

		const previousFeedback = userFeedback;
		userFeedback = feedbackType;

		try {
			// Assume external API exists for feedback
			const response = await fetch('/api/v1/feedback', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					talent_id: talentId,
					job_id: jobId,
					feedback_type: feedbackType,
					expected_rank: expectedRank
				})
			});

			if (!response.ok) {
				throw new Error('Failed to submit feedback');
			}

			// Update stats if enabled
			if (showStats) {
				await loadStats();
			}
		} catch (error) {
			console.error('Error submitting feedback:', error);
			userFeedback = previousFeedback;
		} finally {
			isSubmitting = false;
		}
	}

	// Load feedback stats
	async function loadStats() {
		try {
			const response = await fetch(
				`/api/v1/feedback/talent/${talentId}/stats?job_id=${jobId}`
			);

			if (response.ok) {
				const data = await response.json();
				feedbackStats = {
					upvotes: data.upvotes,
					downvotes: data.downvotes,
					netScore: data.net_score
				};
			}
		} catch (error) {
			console.error('Error loading feedback stats:', error);
		}
	}

	// Toggle feedback
	function toggleFeedback(type: 'upvote' | 'downvote') {
		if (userFeedback === type) {
			// Cancel feedback (would need delete endpoint)
			userFeedback = null;
		} else {
			submitFeedback(type);
		}
	}

	// Load stats on mount if enabled
	$effect(() => {
		if (showStats) {
			loadStats();
		}
	});
</script>

<div class={cn('flex items-center gap-2', className)}>
	<div class="flex items-center gap-1">
		<Tooltip.Root>
			<Tooltip.Trigger>
				<Button
					variant="ghost"
					size="icon"
					class={cn(
						'h-8 w-8 transition-colors',
						userFeedback === 'upvote' && 'bg-green-100 text-green-600 hover:bg-green-200 dark:bg-green-900/30 dark:text-green-400'
					)}
					disabled={disabled || isSubmitting}
					onclick={() => toggleFeedback('upvote')}
				>
					<ThumbsUp class="h-4 w-4" />
				</Button>
			</Tooltip.Trigger>
			<Tooltip.Content>
				<p>Better than expected</p>
			</Tooltip.Content>
		</Tooltip.Root>

		{#if showStats && feedbackStats.upvotes > 0}
			<span class="text-xs text-green-600 dark:text-green-400 font-medium">
				{feedbackStats.upvotes}
			</span>
		{/if}
	</div>

	<div class="flex items-center gap-1">
		<Tooltip.Root>
			<Tooltip.Trigger>
				<Button
					variant="ghost"
					size="icon"
					class={cn(
						'h-8 w-8 transition-colors',
						userFeedback === 'downvote' && 'bg-red-100 text-red-600 hover:bg-red-200 dark:bg-red-900/30 dark:text-red-400'
					)}
					disabled={disabled || isSubmitting}
					onclick={() => toggleFeedback('downvote')}
				>
					<ThumbsDown class="h-4 w-4" />
				</Button>
			</Tooltip.Trigger>
			<Tooltip.Content>
				<p>Worse than expected</p>
			</Tooltip.Content>
		</Tooltip.Root>

		{#if showStats && feedbackStats.downvotes > 0}
			<span class="text-xs text-red-600 dark:text-red-400 font-medium">
				{feedbackStats.downvotes}
			</span>
		{/if}
	</div>

	{#if showStats && (feedbackStats.upvotes > 0 || feedbackStats.downvotes > 0)}
		<div class="ml-1 flex items-center gap-1">
			<span class="text-xs text-muted-foreground">•</span>
			<span
				class={cn(
					'text-xs font-medium',
					feedbackStats.netScore > 0 && 'text-green-600 dark:text-green-400',
					feedbackStats.netScore < 0 && 'text-red-600 dark:text-red-400',
					feedbackStats.netScore === 0 && 'text-muted-foreground'
				)}
			>
				{feedbackStats.netScore > 0 ? '+' : ''}{feedbackStats.netScore}
			</span>
		</div>
	{/if}
</div>
