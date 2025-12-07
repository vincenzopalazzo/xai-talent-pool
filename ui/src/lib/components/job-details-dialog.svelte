<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import * as Avatar from '$lib/components/ui/avatar';
	import { Separator } from '$lib/components/ui/separator';
	import {
		MapPin,
		Briefcase,
		DollarSign,
		Building2,
		Clock,
		Calendar,
		ExternalLink,
		Star,
		Users,
		Sparkles,
		Loader2,
		Trophy,
		AlertCircle,
		CheckCircle2
	} from 'lucide-svelte';
	import ApplyJobDialog from './apply-job-dialog.svelte';
	import JobApplicationsDialog from './job-applications-dialog.svelte';
	import TalentDetailDialog from './talent-detail-dialog.svelte';
	import type { Job, JobMatch, Talent } from '$lib/types';

	const API_BASE = 'http://localhost:8080/api/v1';

	let {
		job,
		open = $bindable(false)
	}: {
		job: Job;
		open: boolean;
	} = $props();

	let applyDialogOpen = $state(false);
	let applicationsDialogOpen = $state(false);
	let selectedTalent = $state<Talent | null>(null);
	let talentDetailOpen = $state(false);

	// Job matches state
	let matches = $state<JobMatch[]>([]);
	let loadingMatches = $state(false);
	let generatingMatches = $state(false);
	let matchError = $state<string | null>(null);

	// Load matches when dialog opens
	$effect(() => {
		if (open && job?.id) {
			loadMatches();
		}
	});

	async function loadMatches() {
		loadingMatches = true;
		matchError = null;
		try {
			const response = await fetch(`${API_BASE}/jobs/${job.id}/matches`);
			if (response.ok) {
				matches = await response.json();
			} else {
				matches = [];
			}
		} catch (err) {
			console.error('Failed to load matches:', err);
			matches = [];
		} finally {
			loadingMatches = false;
		}
	}

	async function generateMatches() {
		generatingMatches = true;
		matchError = null;
		try {
			const response = await fetch(`${API_BASE}/jobs/${job.id}/matches/generate`, {
				method: 'POST'
			});
			if (response.ok) {
				const result = await response.json();
				matches = result.matches || [];
			} else {
				const error = await response.text();
				matchError = `Failed to generate matches: ${error}`;
			}
		} catch (err) {
			console.error('Failed to generate matches:', err);
			matchError = `Failed to generate matches: ${err}`;
		} finally {
			generatingMatches = false;
		}
	}

	function getScoreColor(score: number): string {
		if (score >= 80) return 'text-green-600 dark:text-green-400';
		if (score >= 60) return 'text-yellow-600 dark:text-yellow-400';
		return 'text-red-600 dark:text-red-400';
	}

	function getScoreBg(score: number): string {
		if (score >= 80) return 'bg-green-100 dark:bg-green-900/30';
		if (score >= 60) return 'bg-yellow-100 dark:bg-yellow-900/30';
		return 'bg-red-100 dark:bg-red-900/30';
	}

	function openTalentDetail(talent: Talent | undefined) {
		if (talent) {
			selectedTalent = talent;
			talentDetailOpen = true;
		}
	}

	// Parse skills from comma-separated string
	const skills = $derived(
		job.skills_required ? job.skills_required.split(',').map((s) => s.trim()) : []
	);

	// Format salary
	const salaryDisplay = $derived(() => {
		if (!job.salary_min && !job.salary_max) return null;
		const currency = job.salary_currency || 'USD';
		const formatter = new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency,
			maximumFractionDigits: 0
		});
		if (job.salary_min && job.salary_max) {
			return `${formatter.format(job.salary_min)} - ${formatter.format(job.salary_max)}`;
		}
		if (job.salary_min) return `From ${formatter.format(job.salary_min)}`;
		if (job.salary_max) return `Up to ${formatter.format(job.salary_max)}`;
		return null;
	});

	// Location type badge variant
	const locationTypeVariant = $derived(() => {
		switch (job.location_type?.toLowerCase()) {
			case 'remote':
				return 'default';
			case 'hybrid':
				return 'secondary';
			default:
				return 'outline';
		}
	});

	// Experience level display
	const experienceDisplay = $derived(() => {
		const levels: Record<string, string> = {
			entry: 'Entry Level',
			mid: 'Mid Level',
			senior: 'Senior',
			lead: 'Lead / Principal'
		};
		return levels[job.experience_level?.toLowerCase()] || job.experience_level;
	});

	// Format date
	const formatDate = (dateStr: string | undefined) => {
		if (!dateStr) return null;
		return new Date(dateStr).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	};
</script>

<Dialog.Root bind:open>
	<Dialog.Content class="max-h-[90vh] max-w-4xl overflow-y-auto sm:max-w-3xl md:max-w-4xl lg:max-w-5xl">
		<Dialog.Header>
			<div class="flex items-start gap-4">
				<Avatar.Root class="h-16 w-16 rounded-lg">
					{#if job.company_logo}
						<Avatar.Image src={job.company_logo} alt={job.company_name} class="rounded-lg" />
					{/if}
					<Avatar.Fallback class="rounded-lg bg-primary/10">
						<Building2 class="h-8 w-8 text-primary" />
					</Avatar.Fallback>
				</Avatar.Root>
				<div class="flex-1">
					<Dialog.Title class="text-xl">{job.title}</Dialog.Title>
					<Dialog.Description class="flex items-center gap-1 text-base">
						<Building2 class="h-4 w-4" />
						{job.company_name}
					</Dialog.Description>
				</div>
			</div>
		</Dialog.Header>

		<div class="space-y-6 py-4">
			<!-- Status badges -->
			<div class="flex flex-wrap items-center gap-2">
				<Badge variant={locationTypeVariant()} class="capitalize">
					{job.location_type}
				</Badge>
				<Badge variant="outline" class="capitalize">
					{job.employment_type?.replace('-', ' ')}
				</Badge>
				{#if job.status === 'active'}
					<Badge
						variant="secondary"
						class="bg-green-100 text-green-700 dark:bg-green-900 dark:text-green-300"
					>
						Active
					</Badge>
				{/if}
			</div>

			<!-- Key details -->
			<div class="grid gap-3 rounded-lg bg-muted/50 p-4 sm:grid-cols-2">
				{#if job.location}
					<div class="flex items-center gap-2 text-sm">
						<MapPin class="h-4 w-4 text-muted-foreground" />
						<span>{job.location}</span>
					</div>
				{/if}
				<div class="flex items-center gap-2 text-sm">
					<Briefcase class="h-4 w-4 text-muted-foreground" />
					<span>{experienceDisplay()}</span>
				</div>
				{#if salaryDisplay()}
					<div class="flex items-center gap-2 text-sm font-medium">
						<DollarSign class="h-4 w-4 text-muted-foreground" />
						<span>{salaryDisplay()}</span>
					</div>
				{/if}
				{#if job.created_at}
					<div class="flex items-center gap-2 text-sm text-muted-foreground">
						<Calendar class="h-4 w-4" />
						<span>Posted {formatDate(job.created_at)}</span>
					</div>
				{/if}
				{#if job.expires_at}
					<div class="flex items-center gap-2 text-sm text-muted-foreground">
						<Clock class="h-4 w-4" />
						<span>Expires {formatDate(job.expires_at)}</span>
					</div>
				{/if}
			</div>

			<Separator />

			<!-- Description -->
			<div class="space-y-2">
				<h3 class="text-sm font-semibold">About the Role</h3>
				<p class="whitespace-pre-wrap text-sm text-muted-foreground">{job.description}</p>
			</div>

			<!-- Skills -->
			{#if skills.length > 0}
				<div class="space-y-2">
					<h3 class="text-sm font-semibold">Required Skills</h3>
					<div class="flex flex-wrap gap-2">
						{#each skills as skill}
							<Badge variant="outline">{skill}</Badge>
						{/each}
					</div>
				</div>
			{/if}

			<Separator />

			<!-- Top Candidates Section -->
			<div class="space-y-4">
				<div class="flex items-center justify-between">
					<div class="flex items-center gap-2">
						<Trophy class="h-5 w-5 text-yellow-500" />
						<h3 class="text-sm font-semibold">Top Candidates</h3>
						{#if matches.length > 0}
							<Badge variant="secondary" class="text-xs">{matches.length} matches</Badge>
						{/if}
					</div>
					<Button
						variant="outline"
						size="sm"
						onclick={generateMatches}
						disabled={generatingMatches}
					>
						{#if generatingMatches}
							<Loader2 class="mr-2 h-4 w-4 animate-spin" />
							Analyzing...
						{:else}
							<Sparkles class="mr-2 h-4 w-4" />
							{matches.length > 0 ? 'Refresh' : 'Find Best Candidates'}
						{/if}
					</Button>
				</div>

				{#if matchError}
					<div class="flex items-center gap-2 rounded-lg bg-red-100 p-3 text-sm text-red-700 dark:bg-red-900/30 dark:text-red-400">
						<AlertCircle class="h-4 w-4" />
						<span>{matchError}</span>
					</div>
				{/if}

				{#if loadingMatches}
					<div class="flex items-center justify-center py-8">
						<Loader2 class="h-6 w-6 animate-spin text-muted-foreground" />
					</div>
				{:else if matches.length === 0}
					<div class="rounded-lg border border-dashed p-6 text-center">
						<Sparkles class="mx-auto h-8 w-8 text-muted-foreground" />
						<p class="mt-2 text-sm text-muted-foreground">
							No candidates matched yet. Click "Find Best Candidates" to use AI to find the best matches from your talent pool.
						</p>
					</div>
				{:else}
					<div class="space-y-3">
						{#each matches as match, index}
							<button
								type="button"
								class="w-full rounded-lg border p-4 text-left transition-colors hover:bg-muted/50"
								onclick={() => openTalentDetail(match.talent)}
							>
								<div class="flex items-start gap-3">
									<!-- Rank Badge -->
									<div class="flex h-8 w-8 shrink-0 items-center justify-center rounded-full {index === 0 ? 'bg-yellow-100 text-yellow-700 dark:bg-yellow-900/30 dark:text-yellow-400' : index === 1 ? 'bg-gray-200 text-gray-700 dark:bg-gray-700 dark:text-gray-300' : index === 2 ? 'bg-orange-100 text-orange-700 dark:bg-orange-900/30 dark:text-orange-400' : 'bg-muted text-muted-foreground'} text-sm font-bold">
										{match.rank}
									</div>

									<!-- Candidate Info -->
									<div class="min-w-0 flex-1">
										<div class="flex items-center gap-2">
											<Avatar.Root class="h-8 w-8">
												{#if match.talent?.avatar}
													<Avatar.Image src={match.talent.avatar} alt={match.talent?.name} />
												{/if}
												<Avatar.Fallback class="text-xs">
													{match.talent?.name?.slice(0, 2).toUpperCase() || '??'}
												</Avatar.Fallback>
											</Avatar.Root>
											<div class="min-w-0">
												<p class="truncate font-medium">{match.talent?.name || 'Unknown'}</p>
												<p class="truncate text-xs text-muted-foreground">
													{match.talent?.title || 'No title'}
												</p>
											</div>
										</div>

										<!-- Match Summary -->
										{#if match.summary}
											<p class="mt-2 line-clamp-2 text-sm text-muted-foreground">
												{match.summary}
											</p>
										{/if}

										<!-- Match Reasons -->
										{#if match.match_reasons && match.match_reasons.length > 0}
											<div class="mt-2 flex flex-wrap gap-1">
												{#each match.match_reasons.slice(0, 3) as reason}
													<Badge variant="secondary" class="text-xs">
														<CheckCircle2 class="mr-1 h-3 w-3 text-green-500" />
														{reason}
													</Badge>
												{/each}
												{#if match.match_reasons.length > 3}
													<Badge variant="outline" class="text-xs">
														+{match.match_reasons.length - 3} more
													</Badge>
												{/if}
											</div>
										{/if}
									</div>

									<!-- Score -->
									<div class="shrink-0 text-right">
										<div class="rounded-lg px-3 py-1 {getScoreBg(match.score)}">
											<span class="text-lg font-bold {getScoreColor(match.score)}">
												{Math.round(match.score)}
											</span>
											<span class="text-xs {getScoreColor(match.score)}">/100</span>
										</div>
									</div>
								</div>
							</button>
						{/each}
					</div>
				{/if}
			</div>
		</div>

		<Dialog.Footer class="flex-col gap-2 sm:flex-row">
			<Button variant="outline" class="w-full sm:w-auto" onclick={() => (open = false)}>
				Close
			</Button>
			<Button variant="secondary" class="w-full sm:w-auto" onclick={() => (applicationsDialogOpen = true)}>
				<Users class="mr-2 h-4 w-4" />
				View Applications
			</Button>
			<Button class="w-full sm:flex-1" onclick={() => (applyDialogOpen = true)}>
				<ExternalLink class="mr-2 h-4 w-4" />
				Apply Now
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

<ApplyJobDialog {job} bind:open={applyDialogOpen} />
<JobApplicationsDialog {job} bind:open={applicationsDialogOpen} />
{#if selectedTalent}
	<TalentDetailDialog talent={selectedTalent} bind:open={talentDetailOpen} />
{/if}
