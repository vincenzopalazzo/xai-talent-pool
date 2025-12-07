<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import * as Avatar from '$lib/components/ui/avatar';
	import { Separator } from '$lib/components/ui/separator';
	import {
		FileText,
		Download,
		User,
		Mail,
		Calendar,
		Loader2,
		ChevronDown,
		ChevronUp,
		ExternalLink
	} from 'lucide-svelte';
	import type { Job, Application, Talent } from '$lib/types';

	let {
		job,
		open = $bindable(false)
	}: {
		job: Job;
		open: boolean;
	} = $props();

	let applications = $state<Application[]>([]);
	let isLoading = $state(false);
	let error = $state<string | null>(null);
	let expandedCoverLetters = $state<Set<string>>(new Set());

	// Fetch applications when dialog opens
	$effect(() => {
		if (open && job.id) {
			fetchApplications();
		}
	});

	async function fetchApplications() {
		isLoading = true;
		error = null;

		try {
			const response = await fetch(`http://localhost:8080/api/v1/applications/job/${job.id}`);
			if (!response.ok) {
				throw new Error('Failed to fetch applications');
			}
			const apps: Application[] = await response.json();

			// Fetch talent info for each application
			const appsWithTalent = await Promise.all(
				apps.map(async (app) => {
					try {
						const talentResponse = await fetch(
							`http://localhost:8080/api/v1/talents/${app.talent_id}`
						);
						if (talentResponse.ok) {
							app.talent = await talentResponse.json();
						}
					} catch {
						// Talent fetch failed, continue without it
					}
					return app;
				})
			);

			applications = appsWithTalent;
		} catch (err) {
			error = err instanceof Error ? err.message : 'An error occurred';
		} finally {
			isLoading = false;
		}
	}

	function downloadResume(applicationId: string, filename: string) {
		// Open the resume download URL in a new tab
		window.open(`http://localhost:8080/api/v1/applications/${applicationId}/resume`, '_blank');
	}

	function toggleCoverLetter(id: string) {
		const newSet = new Set(expandedCoverLetters);
		if (newSet.has(id)) {
			newSet.delete(id);
		} else {
			newSet.add(id);
		}
		expandedCoverLetters = newSet;
	}

	// Format date
	const formatDate = (dateStr: string) => {
		return new Date(dateStr).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
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
	<Dialog.Content class="max-h-[90vh] max-w-3xl overflow-y-auto">
		<Dialog.Header>
			<Dialog.Title>Applications for {job.title}</Dialog.Title>
			<Dialog.Description>
				{applications.length} application{applications.length !== 1 ? 's' : ''} received
			</Dialog.Description>
		</Dialog.Header>

		<div class="py-4">
			{#if isLoading}
				<div class="flex items-center justify-center py-8">
					<Loader2 class="h-8 w-8 animate-spin text-muted-foreground" />
				</div>
			{:else if error}
				<div class="rounded-md bg-destructive/10 p-4 text-center text-sm text-destructive">
					{error}
				</div>
			{:else if applications.length === 0}
				<div class="py-8 text-center text-muted-foreground">
					<User class="mx-auto mb-2 h-12 w-12 opacity-50" />
					<p>No applications yet</p>
				</div>
			{:else}
				<div class="space-y-4">
					{#each applications as app (app.id)}
						<div class="rounded-lg border p-4">
							<div class="flex items-start justify-between gap-4">
								<div class="flex items-start gap-3">
									<Avatar.Root class="h-10 w-10">
										{#if app.talent?.avatar}
											<Avatar.Image src={app.talent.avatar} alt={app.talent.name} />
										{/if}
										<Avatar.Fallback>
											{app.talent?.name?.slice(0, 2).toUpperCase() || 'UN'}
										</Avatar.Fallback>
									</Avatar.Root>
									<div class="space-y-1">
										<div class="flex items-center gap-2">
											<p class="font-medium">{app.talent?.name || 'Unknown'}</p>
											<Badge variant={getStatusVariant(app.status)} class="capitalize text-xs">
												{app.status}
											</Badge>
										</div>
										{#if app.talent?.title}
											<p class="text-sm text-muted-foreground">{app.talent.title}</p>
										{/if}
										{#if app.talent?.email}
											<div class="flex items-center gap-1 text-xs text-muted-foreground">
												<Mail class="h-3 w-3" />
												<a href="mailto:{app.talent.email}" class="hover:underline">
													{app.talent.email}
												</a>
											</div>
										{/if}
										<div class="flex items-center gap-1 text-xs text-muted-foreground">
											<Calendar class="h-3 w-3" />
											<span>Applied {formatDate(app.created_at)}</span>
										</div>
									</div>
								</div>

								<div class="flex flex-col gap-2">
									{#if app.has_resume && app.resume_filename}
										<Button
											variant="outline"
											size="sm"
											onclick={() => downloadResume(app.id, app.resume_filename || 'resume')}
										>
											<Download class="mr-2 h-4 w-4" />
											Resume
										</Button>
									{/if}
								</div>
							</div>

							<!-- Cover Letter -->
							{#if app.cover_letter}
								<div class="mt-3">
									<button
										class="flex items-center gap-1 text-sm font-medium text-muted-foreground hover:text-foreground"
										onclick={() => toggleCoverLetter(app.id)}
									>
										{#if expandedCoverLetters.has(app.id)}
											<ChevronUp class="h-4 w-4" />
										{:else}
											<ChevronDown class="h-4 w-4" />
										{/if}
										Cover Letter
									</button>
									{#if expandedCoverLetters.has(app.id)}
										<div class="mt-2 rounded-md bg-muted/50 p-3">
											<p class="whitespace-pre-wrap text-sm">{app.cover_letter}</p>
										</div>
									{/if}
								</div>
							{/if}

							<!-- Resume filename indicator -->
							{#if app.has_resume && app.resume_filename}
								<div class="mt-3 flex items-center gap-2 text-xs text-muted-foreground">
									<FileText class="h-3 w-3" />
									<span>{app.resume_filename}</span>
								</div>
							{/if}
						</div>
					{/each}
				</div>
			{/if}
		</div>

		<Dialog.Footer>
			<Button variant="outline" onclick={() => (open = false)}>Close</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
