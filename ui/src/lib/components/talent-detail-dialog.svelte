<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import * as Avatar from '$lib/components/ui/avatar';
	import { Separator } from '$lib/components/ui/separator';
	import {
		MapPin,
		Briefcase,
		Mail,
		Calendar,
		ExternalLink,
		CheckCircle2,
		FileText,
		Eye,
		Loader2,
		Building2,
		Github,
		Linkedin,
		Twitter,
		AlertCircle,
		Sparkles,
		RefreshCw
	} from 'lucide-svelte';
	import PdfPreviewDialog from './pdf-preview-dialog.svelte';
	import type { Talent, Application, Job, ExperienceSummary } from '$lib/types';

	let {
		talent,
		open = $bindable(false)
	}: {
		talent: Talent;
		open: boolean;
	} = $props();

	// Applications state
	interface ApplicationWithJob extends Application {
		job?: Job;
	}
	let applications = $state<ApplicationWithJob[]>([]);
	let isLoadingApplications = $state(false);

	// PDF preview state
	let previewDialogOpen = $state(false);
	let previewApplicationId = $state<string>('');
	let previewFilename = $state<string>('Resume');

	// Fetch applications when dialog opens
	$effect(() => {
		if (open && talent.id) {
			fetchApplications();
		}
	});

	async function fetchApplications() {
		isLoadingApplications = true;
		try {
			const response = await fetch(`http://localhost:8080/api/v1/applications/talent/${talent.id}`);
			if (response.ok) {
				const apps: Application[] = await response.json();

				// Fetch job info for each application
				const appsWithJobs = await Promise.all(
					apps.map(async (app) => {
						try {
							const jobResponse = await fetch(`http://localhost:8080/api/v1/jobs/${app.job_id}`);
							if (jobResponse.ok) {
								const job = await jobResponse.json();
								return { ...app, job };
							}
						} catch {
							// Job fetch failed, continue without it
						}
						return app;
					})
				);

				applications = appsWithJobs;
			}
		} catch {
			// Failed to fetch applications
		} finally {
			isLoadingApplications = false;
		}
	}

	function openResumePreview(applicationId: string, filename?: string) {
		previewApplicationId = applicationId;
		previewFilename = filename || 'Resume';
		previewDialogOpen = true;
	}

	// Parse skills from comma-separated string or array
	const skills = $derived(() => {
		if (!talent.skills) return [];
		if (Array.isArray(talent.skills)) return talent.skills;
		return (talent.skills as unknown as string).split(',').map((s) => s.trim()).filter((s) => s);
	});

	// Parse resume experiences from JSON string
	const resumeExperiences = $derived(() => {
		if (!talent.resume_experiences) return [];
		try {
			return JSON.parse(talent.resume_experiences) as ExperienceSummary[];
		} catch {
			return [];
		}
	});

	// Check if there are any social links
	const hasSocialLinks = $derived(() => {
		return talent.linkedin_url || talent.x_url || talent.github_url || talent.gitlab_url;
	});

	// Check if there are any social research reports
	const hasSocialResearch = $derived(() => {
		return talent.github_report_id || talent.linkedin_report_id || talent.twitter_report_id || talent.stackoverflow_report_id;
	});

	// Check if social research is in progress
	const isResearchInProgress = $derived(() => talent.social_research_status === 'in_progress');
	const isResearchPending = $derived(() => talent.social_research_status === 'pending');
	const isResearchCompleted = $derived(() => talent.social_research_status === 'completed');
	const isResearchFailed = $derived(() => talent.social_research_status === 'failed');

	// Polling state
	let pollingInterval = $state<ReturnType<typeof setInterval> | null>(null);
	let isRefreshing = $state(false);

	// Start polling when dialog opens and research is in progress or pending
	$effect(() => {
		if (open && (isResearchInProgress() || isResearchPending())) {
			startPolling();
		} else {
			stopPolling();
		}
		return () => stopPolling();
	});

	function startPolling() {
		if (pollingInterval) return;
		pollingInterval = setInterval(async () => {
			await refreshTalentData();
		}, 5000); // Poll every 5 seconds
	}

	function stopPolling() {
		if (pollingInterval) {
			clearInterval(pollingInterval);
			pollingInterval = null;
		}
	}

	async function refreshTalentData() {
		isRefreshing = true;
		try {
			const response = await fetch(`http://localhost:8080/api/v1/talents/${talent.id}`);
			if (response.ok) {
				const updatedTalent = await response.json();
				// Update talent properties
				Object.assign(talent, updatedTalent);
				// Stop polling if research is no longer in progress or pending
				const status = updatedTalent.social_research_status;
				if (status !== 'in_progress' && status !== 'pending') {
					stopPolling();
				}
			}
		} catch {
			// Failed to refresh
		} finally {
			isRefreshing = false;
		}
	}

	// Format date
	const formatDate = (dateStr: string | undefined) => {
		if (!dateStr) return null;
		return new Date(dateStr).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	};

	// Status badge variant
	const getStatusVariant = (status: string) => {
		switch (status.toLowerCase()) {
			case 'accepted':
				return 'default';
			case 'rejected':
				return 'destructive';
			case 'reviewed':
				return 'secondary';
			default:
				return 'outline';
		}
	};
</script>

<Dialog.Root bind:open>
	<Dialog.Content class="max-h-[90vh]! max-w-4xl! w-[90vw]! overflow-y-auto">
		<Dialog.Header>
			<div class="flex items-start gap-4">
				<Avatar.Root class="h-20 w-20">
					{#if talent.avatar}
						<Avatar.Image src={talent.avatar} alt={talent.name} />
					{/if}
					<Avatar.Fallback class="text-xl">{talent.name.slice(0, 2).toUpperCase()}</Avatar.Fallback>
				</Avatar.Root>
				<div class="flex-1">
					<div class="flex items-center gap-2">
						<Dialog.Title class="text-xl">{talent.name}</Dialog.Title>
						{#if talent.verified}
							<Badge variant="secondary" class="gap-1">
								<CheckCircle2 class="h-3 w-3" />
								Verified
							</Badge>
						{/if}
					</div>
					<Dialog.Description class="text-base text-muted-foreground">
						@{talent.handle}
					</Dialog.Description>
					<p class="mt-1 text-sm font-medium">{talent.title}</p>
				</div>
			</div>
		</Dialog.Header>

		<div class="space-y-6 py-4">
			<!-- Key details -->
			<div class="grid gap-3 rounded-lg bg-muted/50 p-4 sm:grid-cols-2">
				{#if talent.email}
					<div class="flex items-center gap-2 text-sm">
						<Mail class="h-4 w-4 text-muted-foreground" />
						<span>{talent.email}</span>
					</div>
				{/if}
				{#if talent.location}
					<div class="flex items-center gap-2 text-sm">
						<MapPin class="h-4 w-4 text-muted-foreground" />
						<span>{talent.location}</span>
					</div>
				{/if}
				<div class="flex items-center gap-2 text-sm">
					<Briefcase class="h-4 w-4 text-muted-foreground" />
					<span>{talent.experience}</span>
				</div>
				{#if talent.created_at}
					<div class="flex items-center gap-2 text-sm text-muted-foreground">
						<Calendar class="h-4 w-4" />
						<span>Joined {formatDate(talent.created_at)}</span>
					</div>
				{/if}
			</div>

			<!-- Bio -->
			{#if talent.bio}
				<div class="space-y-2">
					<h3 class="text-sm font-semibold">About</h3>
					<p class="whitespace-pre-wrap text-sm text-muted-foreground">{talent.bio}</p>
				</div>

				<Separator />
			{/if}

			<!-- Skills -->
			{#if skills().length > 0}
				<div class="space-y-2">
					<h3 class="text-sm font-semibold">Skills</h3>
					<div class="flex flex-wrap gap-2">
						{#each skills() as skill}
							<Badge variant="outline">{skill}</Badge>
						{/each}
					</div>
				</div>
			{/if}

			<!-- Social Links (from Grok analysis) -->
			{#if hasSocialLinks()}
				<Separator />
				<div class="space-y-2">
					<h3 class="text-sm font-semibold">Social Profiles</h3>
					<div class="flex flex-wrap gap-2">
						{#if talent.linkedin_url}
							<Button variant="outline" size="sm" href={talent.linkedin_url} target="_blank" rel="noopener noreferrer">
								<Linkedin class="mr-1 h-4 w-4" />
								LinkedIn
							</Button>
						{/if}
						{#if talent.x_url}
							<Button variant="outline" size="sm" href={talent.x_url} target="_blank" rel="noopener noreferrer">
								<Twitter class="mr-1 h-4 w-4" />
								X
							</Button>
						{/if}
						{#if talent.github_url}
							<Button variant="outline" size="sm" href={talent.github_url} target="_blank" rel="noopener noreferrer">
								<Github class="mr-1 h-4 w-4" />
								GitHub
							</Button>
						{/if}
						{#if talent.gitlab_url}
							<Button variant="outline" size="sm" href={talent.gitlab_url} target="_blank" rel="noopener noreferrer">
								<ExternalLink class="mr-1 h-4 w-4" />
								GitLab
							</Button>
						{/if}
					</div>
				</div>
			{/if}

			<!-- AI Research Insights -->
			{#if talent.social_research_status || hasSocialResearch()}
				<Separator />
				<div class="space-y-3">
					<div class="flex items-center justify-between">
						<div class="flex items-center gap-2">
							<Sparkles class="h-4 w-4 text-primary" />
							<h3 class="text-sm font-semibold">AI Research Insights</h3>
						</div>
						<!-- Status indicator -->
						{#if isResearchInProgress()}
							<Badge variant="secondary" class="gap-1">
								<Loader2 class="h-3 w-3 animate-spin" />
								Analyzing...
							</Badge>
						{:else if isResearchPending()}
							<Badge variant="secondary" class="gap-1 bg-amber-500">
								<Loader2 class="h-3 w-3 animate-spin" />
								Updating...
							</Badge>
						{:else if isResearchCompleted()}
							<Badge variant="default" class="gap-1 bg-green-600">
								<CheckCircle2 class="h-3 w-3" />
								Complete
							</Badge>
						{:else if isResearchFailed()}
							<Badge variant="destructive" class="gap-1">
								<AlertCircle class="h-3 w-3" />
								Failed
							</Badge>
						{/if}
					</div>

					{#if isResearchInProgress() && !hasSocialResearch()}
						<div class="rounded-lg border border-dashed p-4">
							<div class="flex items-center gap-3">
								<div class="flex h-10 w-10 items-center justify-center rounded-full bg-primary/10">
									<Loader2 class="h-5 w-5 animate-spin text-primary" />
								</div>
								<div>
									<p class="text-sm font-medium">Researching social profiles...</p>
									<p class="text-xs text-muted-foreground">
										Analyzing GitHub, LinkedIn, and X profiles using Grok AI
									</p>
								</div>
							</div>
						</div>
					{:else if isResearchFailed()}
						<div class="rounded-lg border border-destructive/50 bg-destructive/5 p-4">
							<div class="flex items-center gap-3">
								<div class="flex h-10 w-10 items-center justify-center rounded-full bg-destructive/10">
									<AlertCircle class="h-5 w-5 text-destructive" />
								</div>
								<div class="flex-1">
									<p class="text-sm font-medium">Research failed</p>
									<p class="text-xs text-muted-foreground">
										Unable to analyze social profiles. This may be due to limited public information.
									</p>
								</div>
								<Button variant="outline" size="sm" onclick={refreshTalentData} disabled={isRefreshing}>
									{#if isRefreshing}
										<Loader2 class="mr-1 h-3 w-3 animate-spin" />
									{:else}
										<RefreshCw class="mr-1 h-3 w-3" />
									{/if}
									Retry
								</Button>
							</div>
						</div>
					{:else if hasSocialResearch()}
						<div class="grid gap-2">
							{#if talent.github_report_id}
								<div class="rounded-lg border p-3">
									<div class="flex items-center gap-2">
										<div class="flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-gray-900">
											<Github class="h-4 w-4 text-white" />
										</div>
										<div class="flex-1 min-w-0">
											<p class="text-sm font-medium">GitHub Analysis</p>
										</div>
										<Badge variant="outline" class="shrink-0 text-xs">Available</Badge>
									</div>
									{#if talent.github_tldr}
										<p class="mt-2 text-sm text-muted-foreground">{talent.github_tldr}</p>
									{:else}
										<p class="mt-2 text-xs text-muted-foreground">Code contributions & projects reviewed</p>
									{/if}
								</div>
							{/if}
							{#if talent.linkedin_report_id}
								<div class="rounded-lg border p-3">
									<div class="flex items-center gap-2">
										<div class="flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-blue-600">
											<Linkedin class="h-4 w-4 text-white" />
										</div>
										<div class="flex-1 min-w-0">
											<p class="text-sm font-medium">LinkedIn Analysis</p>
										</div>
										<Badge variant="outline" class="shrink-0 text-xs">Available</Badge>
									</div>
									{#if talent.linkedin_tldr}
										<p class="mt-2 text-sm text-muted-foreground">{talent.linkedin_tldr}</p>
									{:else}
										<p class="mt-2 text-xs text-muted-foreground">Professional experience reviewed</p>
									{/if}
								</div>
							{/if}
							{#if talent.twitter_report_id}
								<div class="rounded-lg border p-3">
									<div class="flex items-center gap-2">
										<div class="flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-black">
											<Twitter class="h-4 w-4 text-white" />
										</div>
										<div class="flex-1 min-w-0">
											<p class="text-sm font-medium">X/Twitter Analysis</p>
										</div>
										<Badge variant="outline" class="shrink-0 text-xs">Available</Badge>
									</div>
									{#if talent.twitter_tldr}
										<p class="mt-2 text-sm text-muted-foreground">{talent.twitter_tldr}</p>
									{:else}
										<p class="mt-2 text-xs text-muted-foreground">Industry engagement reviewed</p>
									{/if}
								</div>
							{/if}
						</div>
						<p class="text-xs text-muted-foreground">
							AI-generated insights are stored in the candidate's collection for review.
						</p>
					{:else}
						<p class="text-sm text-muted-foreground">
							No AI research insights available yet. Submit a resume to trigger analysis.
						</p>
					{/if}
				</div>
			{/if}

			<!-- Work Experience (from Grok analysis) -->
			{#if resumeExperiences().length > 0}
				<Separator />
				<div class="space-y-3">
					<h3 class="text-sm font-semibold">Work Experience (from Resume)</h3>
					<div class="space-y-3">
						{#each resumeExperiences() as exp, i}
							<div class="rounded-lg border p-3">
								<div class="flex items-start gap-2">
									<Building2 class="mt-0.5 h-4 w-4 text-muted-foreground" />
									<div class="flex-1">
										<div class="font-medium text-sm">{exp.role}</div>
										<div class="text-sm text-muted-foreground">{exp.company}</div>
										{#if exp.duration}
											<div class="text-xs text-muted-foreground mt-0.5">{exp.duration}</div>
										{/if}
										{#if exp.summary}
											<p class="text-sm mt-2">{exp.summary}</p>
										{/if}
									</div>
								</div>
							</div>
						{/each}
					</div>
				</div>
			{/if}

			<Separator />

			<!-- Applications -->
			<div class="space-y-3">
				<h3 class="text-sm font-semibold">Job Applications</h3>
				{#if isLoadingApplications}
					<div class="flex items-center justify-center py-4">
						<Loader2 class="h-5 w-5 animate-spin text-muted-foreground" />
					</div>
				{:else if applications.length === 0}
					<p class="text-sm text-muted-foreground">No applications yet</p>
				{:else}
					<div class="space-y-3">
						{#each applications as app (app.id)}
							<div class="rounded-lg border p-3">
								<div class="flex items-start justify-between gap-3">
									<div class="flex-1 space-y-1">
										<div class="flex items-center gap-2">
											<Building2 class="h-4 w-4 text-muted-foreground" />
											<span class="font-medium text-sm">
												{app.job?.title || 'Unknown Position'}
											</span>
											<Badge variant={getStatusVariant(app.status)} class="capitalize text-xs">
												{app.status}
											</Badge>
										</div>
										{#if app.job?.company_name}
											<p class="text-xs text-muted-foreground">{app.job.company_name}</p>
										{/if}
										<p class="text-xs text-muted-foreground">
											Applied {formatDate(app.created_at)}
										</p>
									</div>
									{#if app.has_resume}
										<Button
											variant="outline"
											size="sm"
											onclick={() => openResumePreview(app.id, app.resume_filename)}
										>
											<Eye class="mr-1 h-3 w-3" />
											View Resume
										</Button>
									{/if}
								</div>
								{#if app.has_resume && app.resume_filename}
									<div class="mt-2 flex items-center gap-1 text-xs text-muted-foreground">
										<FileText class="h-3 w-3" />
										<span>{app.resume_filename}</span>
									</div>
								{/if}
							</div>
						{/each}
					</div>
				{/if}
			</div>
		</div>

		<Dialog.Footer class="flex-col gap-2 sm:flex-row">
			<Button variant="outline" class="w-full sm:w-auto" onclick={() => (open = false)}>
				Close
			</Button>
			<Button class="w-full sm:flex-1">
				<ExternalLink class="mr-2 h-4 w-4" />
				Contact
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

<PdfPreviewDialog
	applicationId={previewApplicationId}
	filename={previewFilename}
	bind:open={previewDialogOpen}
/>
