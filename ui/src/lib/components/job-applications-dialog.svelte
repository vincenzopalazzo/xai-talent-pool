<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import * as Avatar from '$lib/components/ui/avatar';
	import { Separator } from '$lib/components/ui/separator';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { Checkbox } from '$lib/components/ui/checkbox';
	import {
		FileText,
		Eye,
		User,
		Mail,
		Calendar,
		Loader2,
		ChevronDown,
		ChevronUp,
		ExternalLink,
		Trash2,
		CheckSquare,
		X,
		GripVertical
	} from 'lucide-svelte';
	import PdfPreviewDialog from './pdf-preview-dialog.svelte';
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

	// Selection state
	let selectionMode = $state(false);
	let selectedApplications = $state<Set<string>>(new Set());
	let deleteDialogOpen = $state(false);
	let isDeleting = $state(false);

	const selectedCount = $derived(selectedApplications.size);
	const allSelected = $derived(applications.length > 0 && selectedApplications.size === applications.length);

	// PDF preview state
	let previewDialogOpen = $state(false);
	let previewApplicationId = $state<string>('');
	let previewFilename = $state<string>('Resume');

	// Drag-and-drop state
	let draggedIndex = $state<number | null>(null);
	let dragOverIndex = $state<number | null>(null);
	let isSavingReorder = $state(false);

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

	function openResumePreview(applicationId: string, filename: string) {
		previewApplicationId = applicationId;
		previewFilename = filename || 'Resume';
		previewDialogOpen = true;
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

	// Selection functions
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

	async function bulkDelete() {
		if (selectedApplications.size === 0) return;

		isDeleting = true;
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
				deleteDialogOpen = false;
				exitSelectionMode();
				await fetchApplications();
			} else {
				console.error('Failed to delete applications:', response.statusText);
			}
		} catch (err) {
			console.error('Error deleting applications:', err);
		} finally {
			isDeleting = false;
		}
	}

	// Drag-and-drop handlers
	function handleDragStart(index: number) {
		draggedIndex = index;
	}

	function handleDragOver(event: DragEvent, index: number) {
		event.preventDefault();
		dragOverIndex = index;
	}

	function handleDragEnd() {
		draggedIndex = null;
		dragOverIndex = null;
	}

	async function handleDrop(event: DragEvent, dropIndex: number) {
		event.preventDefault();

		if (draggedIndex === null || draggedIndex === dropIndex) {
			handleDragEnd();
			return;
		}

		// Capture before order
		const beforeOrder = applications.map((app) => app.talent_id);

		// Reorder applications array
		const newApplications = [...applications];
		const [draggedApp] = newApplications.splice(draggedIndex, 1);
		newApplications.splice(dropIndex, 0, draggedApp);
		applications = newApplications;

		// Capture after order
		const afterOrder = applications.map((app) => app.talent_id);

		handleDragEnd();

		// Save reorder event to backend
		await saveReorderEvent(beforeOrder, afterOrder, draggedApp.talent_id);
	}

	async function saveReorderEvent(
		beforeOrder: string[],
		afterOrder: string[],
		movedTalentId: string
	) {
		isSavingReorder = true;

		try {
			const response = await fetch('http://localhost:8080/api/v1/reorder', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					job_id: job.id,
					before_order: beforeOrder,
					after_order: afterOrder,
					moved_talent_id: movedTalentId
				})
			});

			if (!response.ok) {
				console.error('Failed to save reorder event');
			} else {
				const result = await response.json();
				console.log(`Reorder saved: ${result.preferences_created} preferences created`);
			}
		} catch (err) {
			console.error('Error saving reorder:', err);
		} finally {
			isSavingReorder = false;
		}
	}
</script>

<Dialog.Root bind:open>
	<Dialog.Content class="max-h-[90vh] max-w-3xl overflow-y-auto">
		<Dialog.Header>
			<div class="flex items-center justify-between">
				<div>
					<Dialog.Title>Applications for {job.title}</Dialog.Title>
					<Dialog.Description>
						{applications.length} application{applications.length !== 1 ? 's' : ''} received
					</Dialog.Description>
				</div>
				{#if applications.length > 0}
					<div class="flex items-center gap-2">
						{#if selectionMode}
							<span class="text-sm text-muted-foreground">
								{selectedCount} selected
							</span>
							<Button variant="ghost" size="sm" onclick={toggleSelectAll}>
								{allSelected ? 'Deselect All' : 'Select All'}
							</Button>
							<Button
								variant="destructive"
								size="sm"
								disabled={selectedCount === 0}
								onclick={() => (deleteDialogOpen = true)}
							>
								<Trash2 class="mr-2 h-4 w-4" />
								Delete
							</Button>
							<Button variant="outline" size="sm" onclick={exitSelectionMode}>
								<X class="h-4 w-4" />
							</Button>
						{:else}
							<Button variant="outline" size="sm" onclick={() => (selectionMode = true)}>
								<CheckSquare class="mr-2 h-4 w-4" />
								Select
							</Button>
						{/if}
					</div>
				{/if}
			</div>
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
					{#each applications as app, index (app.id)}
						<div
							class="rounded-lg border p-4 transition-all {draggedIndex === index
								? 'opacity-50'
								: ''} {dragOverIndex === index || selectedApplications.has(app.id) ? 'ring-2 ring-primary' : ''}"
							draggable={!selectionMode}
							ondragstart={() => !selectionMode && handleDragStart(index)}
							ondragover={(e) => !selectionMode && handleDragOver(e, index)}
							ondragend={() => !selectionMode && handleDragEnd()}
							ondrop={(e) => !selectionMode && handleDrop(e, index)}
						>
							<div class="flex items-start justify-between gap-4">
								<div class="flex items-start gap-3">
									{#if selectionMode}
										<Checkbox
											checked={selectedApplications.has(app.id)}
											onCheckedChange={(checked) => handleSelectionChange(app.id, checked)}
											class="mt-1 h-5 w-5"
										/>
									{:else}
										<!-- Drag handle -->
										<div
											class="cursor-grab active:cursor-grabbing text-muted-foreground hover:text-foreground"
											aria-label="Drag to reorder"
										>
											<GripVertical class="h-5 w-5" />
										</div>
									{/if}
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
											onclick={() => openResumePreview(app.id, app.resume_filename || 'resume')}
										>
											<Eye class="mr-2 h-4 w-4" />
											View Resume
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

<PdfPreviewDialog
	applicationId={previewApplicationId}
	filename={previewFilename}
	bind:open={previewDialogOpen}
/>

<AlertDialog.Root bind:open={deleteDialogOpen}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>Delete {selectedCount} application{selectedCount !== 1 ? 's' : ''}?</AlertDialog.Title>
			<AlertDialog.Description>
				This action cannot be undone. This will permanently delete the selected application{selectedCount !== 1 ? 's' : ''}.
			</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<AlertDialog.Cancel disabled={isDeleting}>Cancel</AlertDialog.Cancel>
			<AlertDialog.Action
				onclick={bulkDelete}
				disabled={isDeleting}
				class="bg-destructive text-destructive-foreground hover:bg-destructive/90"
			>
				{isDeleting ? 'Deleting...' : 'Delete'}
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
