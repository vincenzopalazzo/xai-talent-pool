<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { Search, Plus, ClipboardList, Filter } from 'lucide-svelte';
	import Sidebar from '$lib/components/sidebar.svelte';
	import HiringRequirementCard from '$lib/components/hiring-requirement-card.svelte';
	import CreateHiringRequirementDialog from '$lib/components/create-hiring-requirement-dialog.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import type { HiringRequirement, Job } from '$lib/types';

	let { data } = $props();

	let sidebarCollapsed = $state(false);
	let searchQuery = $state('');
	let createDialogOpen = $state(false);
	let deleteDialogOpen = $state(false);
	let requirementToDelete = $state<string | null>(null);

	const requirements = $derived(data.requirements ?? []);
	const jobs = $derived(data.jobs ?? []);

	// Create a map of job_id to Job for quick lookup
	const jobsMap = $derived(() => {
		const map = new Map<string, Job>();
		jobs.forEach((job: Job) => map.set(job.id, job));
		return map;
	});

	// Filter requirements based on search
	const filteredRequirements = $derived(() => {
		if (!searchQuery) return requirements;

		const query = searchQuery.toLowerCase();
		return requirements.filter(
			(req: HiringRequirement) =>
				req.title.toLowerCase().includes(query) ||
				req.company_name.toLowerCase().includes(query) ||
				req.requirements_text.toLowerCase().includes(query)
		);
	});

	async function handleCreated() {
		await invalidateAll();
	}

	function handleDelete(id: string) {
		requirementToDelete = id;
		deleteDialogOpen = true;
	}

	async function confirmDelete() {
		if (!requirementToDelete) return;

		try {
			const response = await fetch(
				`http://localhost:8080/api/v1/hiring-requirements/${requirementToDelete}`,
				{
					method: 'DELETE'
				}
			);

			if (response.ok) {
				await invalidateAll();
			}
		} catch (err) {
			console.error('Failed to delete requirement:', err);
		} finally {
			requirementToDelete = null;
			deleteDialogOpen = false;
		}
	}

	function handleEdit(requirement: HiringRequirement) {
		// For now, just log - could open an edit dialog
		console.log('Edit requirement:', requirement);
	}
</script>

<div class="flex h-screen bg-background">
	<Sidebar bind:collapsed={sidebarCollapsed} />

	<main class="flex-1 overflow-hidden">
		<header class="flex h-14 items-center justify-between border-b border-border px-6">
			<div class="flex items-center gap-2">
				<ClipboardList class="h-5 w-5" />
				<h1 class="text-xl font-semibold">Hiring Requirements</h1>
			</div>
			<div class="flex items-center gap-2">
				<Button variant="outline" size="sm">
					<Filter class="mr-2 h-4 w-4" />
					Filter
				</Button>
				<Button size="sm" onclick={() => (createDialogOpen = true)}>
					<Plus class="mr-2 h-4 w-4" />
					Add Requirements
				</Button>
			</div>
		</header>

		<div class="p-6">
			<div class="mb-6 space-y-4">
				<!-- Search -->
				<div class="flex items-center gap-4">
					<div class="relative max-w-xl flex-1">
						<Search
							class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground"
						/>
						<Input
							type="search"
							placeholder="Search requirements by title, company, or keywords..."
							class="pl-10"
							bind:value={searchQuery}
						/>
					</div>
				</div>
			</div>

			<!-- Results header -->
			<div class="mb-4 flex items-center justify-between">
				<p class="text-sm text-muted-foreground">
					Showing <span class="font-medium text-foreground">{filteredRequirements().length}</span>
					{filteredRequirements().length === 1 ? 'requirement' : 'requirements'}
					{#if searchQuery}
						<button
							class="ml-2 text-primary hover:underline"
							onclick={() => (searchQuery = '')}
						>
							Clear search
						</button>
					{/if}
				</p>
			</div>

			<!-- Requirements grid -->
			<ScrollArea class="h-[calc(100vh-220px)]">
				{#if filteredRequirements().length > 0}
					<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
						{#each filteredRequirements() as requirement (requirement.id)}
							<HiringRequirementCard
								{requirement}
								job={requirement.job_id ? jobsMap().get(requirement.job_id) : undefined}
								onEdit={handleEdit}
								onDelete={handleDelete}
							/>
						{/each}
					</div>
				{:else}
					<div class="flex flex-col items-center justify-center py-16 text-center">
						<ClipboardList class="mb-4 h-12 w-12 text-muted-foreground/50" />
						<h3 class="mb-2 text-lg font-medium">No hiring requirements found</h3>
						<p class="mb-4 text-sm text-muted-foreground">
							{#if searchQuery}
								Try adjusting your search
							{:else}
								Add your first hiring requirements to help match candidates
							{/if}
						</p>
						{#if searchQuery}
							<Button variant="outline" onclick={() => (searchQuery = '')}>
								Clear search
							</Button>
						{:else}
							<Button onclick={() => (createDialogOpen = true)}>
								<Plus class="mr-2 h-4 w-4" />
								Add Requirements
							</Button>
						{/if}
					</div>
				{/if}
			</ScrollArea>
		</div>
	</main>
</div>

<CreateHiringRequirementDialog bind:open={createDialogOpen} {jobs} onSuccess={handleCreated} />

<AlertDialog.Root bind:open={deleteDialogOpen}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>Delete Hiring Requirements?</AlertDialog.Title>
			<AlertDialog.Description>
				This action cannot be undone. This will permanently delete these hiring requirements.
			</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
			<AlertDialog.Action
				class="bg-destructive text-destructive-foreground hover:bg-destructive/90"
				onclick={confirmDelete}
			>
				Delete
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
