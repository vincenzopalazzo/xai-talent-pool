<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import * as Avatar from '$lib/components/ui/avatar';
	import { Separator } from '$lib/components/ui/separator';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { Checkbox } from '$lib/components/ui/checkbox';
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
		Trash2,
		CheckSquare,
		X
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

	// Delete confirmation state
	let deleteConfirmOpen = $state(false);
	let applicationToDelete = $state<string | null>(null);
	let isDeletingApplication = $state(false);

	// Bulk delete state
	let selectionMode = $state(false);
	let selectedApplications = $state<Set<string>>(new Set());
	let bulkDeleteDialogOpen = $state(false);
	let isBulkDeleting = $state(false);

	const selectedCount = $derived(selectedApplications.size);
	const allSelected = $derived(applications.length > 0 && selectedApplications.size === applications.length);

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

	function confirmDeleteApplication(applicationId: string) {
		applicationToDelete = applicationId;
		deleteConfirmOpen = true;
	}

	async function deleteApplication() {
		if (!applicationToDelete) return;

		isDeletingApplication = true;
		try {
			const response = await fetch(
				`http://localhost:8080/api/v1/applications/${applicationToDelete}`,
				{
					method: 'DELETE'
				}
			);

			if (response.ok || response.status === 204) {
				// Remove the deleted application from the list
				applications = applications.filter((app) => app.id !== applicationToDelete);
				deleteConfirmOpen = false;
				applicationToDelete = null;
			} else {
				console.error('Failed to delete application:', response.statusText);
			}
		} catch (error) {
			console.error('Error deleting application:', error);
		} finally {
			isDeletingApplication = false;
		}
	}

	// Bulk delete functions
	function handleSelectionChange(id: string, checked: boolean | 'indeterminate') {
		if (typeof checked === 'boolean') {
			if (checked) {
				selectedApplications.add(id);
			} else {
				selectedApplications.delete(id);
			}
			selectedApplications = new Set(selectedApplications);
		}
	}

	function toggleSelectAll() {
		if (allSelected) {
			selectedApplications = new Set();
		} else {
			selectedApplications = new Set(applications.map((app) => app.id));
		}
	}

	function exitSelectionMode() {
		selectionMode = false;
		selectedApplications = new Set();
	}

	async function bulkDeleteApplications() {
		if (selectedApplications.size === 0) return;

		isBulkDeleting = true;
		try {
			const response = await fetch('http://localhost:8080/api/v1/applications/bulk-delete', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({ ids: Array.from(selectedApplications) })
			});

			if (response.ok) {
				const result = await response.json();
				console.log(`Deleted ${result.deleted_count} of ${result.total_requested} applications`);
				bulkDeleteDialogOpen = false;
				exitSelectionMode();
				await fetchApplications();
			} else {
				console.error('Failed to delete applications:', response.statusText);
			}
		} catch (err) {
			console.error('Error deleting applications:', err);
		} finally {
			isBulkDeleting = false;
		}
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
	<Dialog.Content class="!max-h-[95vh] !max-w-6xl !w-[95vw] sm:!max-w-6xl overflow-hidden flex flex-col">
		<!-- Header Section -->
		<Dialog.Header class="flex-shrink-0 border-b pb-6">
			<div class="flex items-start gap-6">
				<Avatar.Root class="h-24 w-24 ring-4 ring-background shadow-lg">
					{#if talent.avatar}
						<Avatar.Image src={talent.avatar} alt={talent.name} />
					{/if}
					<Avatar.Fallback class="text-2xl font-semibold">{talent.name.slice(0, 2).toUpperCase()}</Avatar.Fallback>
				</Avatar.Root>
				<div class="flex-1 space-y-2">
					<div class="flex items-center gap-3">
						<Dialog.Title class="text-2xl font-bold">{talent.name}</Dialog.Title>
						{#if talent.verified}
							<Badge variant="secondary" class="gap-1 px-2 py-1">
								<CheckCircle2 class="h-3.5 w-3.5" />
								Verified
							</Badge>
						{/if}
					</div>
					<Dialog.Description class="text-base text-muted-foreground">
						@{talent.handle}
					</Dialog.Description>
					<p class="text-lg font-medium text-foreground">{talent.title}</p>

					<!-- Quick info row -->
					<div class="flex flex-wrap items-center gap-4 pt-2 text-sm text-muted-foreground">
						{#if talent.email}
							<div class="flex items-center gap-1.5">
								<Mail class="h-4 w-4" />
								<span>{talent.email}</span>
							</div>
						{/if}
						{#if talent.location}
							<div class="flex items-center gap-1.5">
								<MapPin class="h-4 w-4" />
								<span>{talent.location}</span>
							</div>
						{/if}
						<div class="flex items-center gap-1.5">
							<Briefcase class="h-4 w-4" />
							<span>{talent.experience}</span>
						</div>
						{#if talent.created_at}
							<div class="flex items-center gap-1.5">
								<Calendar class="h-4 w-4" />
								<span>Joined {formatDate(talent.created_at)}</span>
							</div>
						{/if}
					</div>
				</div>
			</div>
		</Dialog.Header>

		<!-- Scrollable Content -->
		<div class="flex-1 overflow-y-auto py-6 px-2">
			<div class="grid gap-8 lg:grid-cols-3">
				<!-- Left Column: About, Skills, Social -->
				<div class="space-y-6">
					<!-- Bio -->
					{#if talent.bio}
						<div class="space-y-3">
							<h3 class="text-lg font-semibold">About</h3>
							<p class="whitespace-pre-wrap text-base text-muted-foreground leading-relaxed">{talent.bio}</p>
						</div>
					{/if}

					<!-- Skills -->
					{#if skills().length > 0}
						<div class="space-y-3">
							<h3 class="text-lg font-semibold">Skills</h3>
							<div class="flex flex-wrap gap-2">
								{#each skills() as skill}
									<Badge variant="outline" class="px-4 py-1.5 text-sm">{skill}</Badge>
								{/each}
							</div>
						</div>
					{/if}

					<!-- Social Links -->
					{#if hasSocialLinks()}
						<div class="space-y-3">
							<h3 class="text-lg font-semibold">Social Profiles</h3>
							<div class="flex flex-wrap gap-3">
								{#if talent.linkedin_url}
									<Button variant="outline" href={talent.linkedin_url} target="_blank" rel="noopener noreferrer">
										<Linkedin class="mr-2 h-4 w-4" />
										LinkedIn
									</Button>
								{/if}
								{#if talent.x_url}
									<Button variant="outline" href={talent.x_url} target="_blank" rel="noopener noreferrer">
										<Twitter class="mr-2 h-4 w-4" />
										X
									</Button>
								{/if}
								{#if talent.github_url}
									<Button variant="outline" href={talent.github_url} target="_blank" rel="noopener noreferrer">
										<Github class="mr-2 h-4 w-4" />
										GitHub
									</Button>
								{/if}
								{#if talent.gitlab_url}
									<Button variant="outline" href={talent.gitlab_url} target="_blank" rel="noopener noreferrer">
										<ExternalLink class="mr-2 h-4 w-4" />
										GitLab
									</Button>
								{/if}
							</div>
						</div>
					{/if}
				</div>

				<!-- Middle Column: Work Experience -->
				<div class="space-y-6 lg:col-span-2">
					<!-- Work Experience -->
					{#if resumeExperiences().length > 0}
						<div class="space-y-4">
							<h3 class="text-lg font-semibold">Work Experience</h3>
							<div class="grid gap-4 md:grid-cols-2">
								{#each resumeExperiences() as exp}
									<div class="rounded-lg border bg-card p-5">
										<div class="flex items-start gap-4">
											<div class="rounded-full bg-muted p-3">
												<Building2 class="h-5 w-5 text-muted-foreground" />
											</div>
											<div class="flex-1 space-y-2">
												<div class="text-lg font-semibold">{exp.role}</div>
												<div class="text-base text-muted-foreground">{exp.company}</div>
												{#if exp.duration}
													<div class="text-sm text-muted-foreground">{exp.duration}</div>
												{/if}
												{#if exp.summary}
													<p class="text-sm mt-3 leading-relaxed text-muted-foreground">{exp.summary}</p>
												{/if}
											</div>
										</div>
									</div>
								{/each}
							</div>
						</div>
					{/if}
				</div>
			</div>

			<!-- Full-width Applications Section -->
			<Separator class="my-8" />

			<!-- Applications -->
			<div class="space-y-4">
				<div class="flex items-center justify-between">
					<h3 class="text-lg font-semibold">Job Applications</h3>
					{#if applications.length > 0}
						<div class="flex items-center gap-2">
							{#if selectionMode}
								<span class="text-xs text-muted-foreground">
									{selectedCount} selected
								</span>
								<Button variant="ghost" size="sm" onclick={toggleSelectAll}>
									{allSelected ? 'Deselect All' : 'Select All'}
								</Button>
								<Button
									variant="destructive"
									size="sm"
									disabled={selectedCount === 0}
									onclick={() => (bulkDeleteDialogOpen = true)}
								>
									<Trash2 class="mr-1 h-3 w-3" />
									Delete
								</Button>
								<Button variant="outline" size="sm" onclick={exitSelectionMode}>
									<X class="h-3 w-3" />
								</Button>
							{:else}
								<Button variant="outline" size="sm" onclick={() => (selectionMode = true)}>
									<CheckSquare class="mr-1 h-3 w-3" />
									Select
								</Button>
							{/if}
						</div>
					{/if}
				</div>
				{#if isLoadingApplications}
					<div class="flex items-center justify-center py-4">
						<Loader2 class="h-5 w-5 animate-spin text-muted-foreground" />
					</div>
				{:else if applications.length === 0}
					<p class="text-base text-muted-foreground">No applications yet</p>
				{:else}
					<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
						{#each applications as app (app.id)}
							<div class="rounded-lg border p-4 {selectedApplications.has(app.id) ? 'ring-2 ring-primary' : ''}">
								<div class="flex items-start gap-3">
									{#if selectionMode}
										<Checkbox
											checked={selectedApplications.has(app.id)}
											onCheckedChange={(checked) => handleSelectionChange(app.id, checked)}
											class="mt-1 h-5 w-5"
										/>
									{/if}
									<div class="flex-1 space-y-2">
										<div class="flex items-center gap-2 flex-wrap">
											<Building2 class="h-5 w-5 text-muted-foreground" />
											<span class="font-semibold text-base">
												{app.job?.title || 'Unknown Position'}
											</span>
										</div>
										<Badge variant={getStatusVariant(app.status)} class="capitalize">
											{app.status}
										</Badge>
										{#if app.job?.company_name}
											<p class="text-sm text-muted-foreground">{app.job.company_name}</p>
										{/if}
										<p class="text-sm text-muted-foreground">
											Applied {formatDate(app.created_at)}
										</p>
										{#if app.has_resume && app.resume_filename}
											<div class="flex items-center gap-1.5 text-sm text-muted-foreground">
												<FileText class="h-4 w-4" />
												<span>{app.resume_filename}</span>
											</div>
										{/if}
										{#if !selectionMode}
											<div class="flex gap-2 pt-2">
												{#if app.has_resume}
													<Button
														variant="outline"
														size="sm"
														onclick={() => openResumePreview(app.id, app.resume_filename)}
													>
														<Eye class="mr-1.5 h-4 w-4" />
														View Resume
													</Button>
												{/if}
												<Button
													variant="outline"
													size="sm"
													onclick={() => confirmDeleteApplication(app.id)}
												>
													<Trash2 class="h-4 w-4" />
												</Button>
											</div>
										{/if}
									</div>
								</div>
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

<!-- Delete Confirmation Dialog -->
<Dialog.Root bind:open={deleteConfirmOpen}>
	<Dialog.Content class="sm:max-w-md">
		<Dialog.Header>
			<Dialog.Title>Delete Application</Dialog.Title>
			<Dialog.Description>
				Are you sure you want to delete this application? This action cannot be undone.
			</Dialog.Description>
		</Dialog.Header>
		<Dialog.Footer class="flex-col gap-2 sm:flex-row">
			<Button
				variant="outline"
				class="w-full sm:w-auto"
				onclick={() => {
					deleteConfirmOpen = false;
					applicationToDelete = null;
				}}
				disabled={isDeletingApplication}
			>
				Cancel
			</Button>
			<Button
				variant="destructive"
				class="w-full sm:flex-1"
				onclick={deleteApplication}
				disabled={isDeletingApplication}
			>
				{#if isDeletingApplication}
					<Loader2 class="mr-2 h-4 w-4 animate-spin" />
					Deleting...
				{:else}
					<Trash2 class="mr-2 h-4 w-4" />
					Delete Application
				{/if}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

<!-- Bulk Delete Confirmation Dialog -->
<AlertDialog.Root bind:open={bulkDeleteDialogOpen}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>Delete {selectedCount} application{selectedCount !== 1 ? 's' : ''}?</AlertDialog.Title>
			<AlertDialog.Description>
				This action cannot be undone. This will permanently delete the selected application{selectedCount !== 1 ? 's' : ''} for {talent.name}.
			</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<AlertDialog.Cancel disabled={isBulkDeleting}>Cancel</AlertDialog.Cancel>
			<AlertDialog.Action
				onclick={bulkDeleteApplications}
				disabled={isBulkDeleting}
				class="bg-destructive text-destructive-foreground hover:bg-destructive/90"
			>
				{isBulkDeleting ? 'Deleting...' : 'Delete'}
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
