<script lang="ts">
	import {
		Search,
		Filter,
		SlidersHorizontal,
		Grid3X3,
		List,
		Plus,
		Briefcase
	} from 'lucide-svelte';
	import { invalidateAll } from '$app/navigation';
	import Sidebar from '$lib/components/sidebar.svelte';
	import JobCard from '$lib/components/job-card.svelte';
	import PostJobDialog from '$lib/components/post-job-dialog.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import type { Job } from '$lib/types';

	let { data } = $props();

	let sidebarCollapsed = $state(false);
	let searchQuery = $state('');
	let viewMode = $state<'grid' | 'list'>('grid');
	let postJobDialogOpen = $state(false);

	const jobs = $derived(data.jobs ?? []);

	async function handleJobCreated() {
		await invalidateAll();
	}

	// Location type filters
	const locationTypes = ['All', 'Remote', 'Hybrid', 'Onsite'];
	let activeLocationType = $state('All');

	// Employment type filters
	const employmentTypes = ['All Types', 'Full-time', 'Part-time', 'Contract'];
	let activeEmploymentType = $state('All Types');

	// Filter jobs based on search and filters
	const filteredJobs = $derived(() => {
		let result = jobs;

		// Search filter
		if (searchQuery) {
			const query = searchQuery.toLowerCase();
			result = result.filter(
				(job: Job) =>
					job.title.toLowerCase().includes(query) ||
					job.company_name.toLowerCase().includes(query) ||
					job.description.toLowerCase().includes(query) ||
					job.skills_required.toLowerCase().includes(query)
			);
		}

		// Location type filter
		if (activeLocationType !== 'All') {
			result = result.filter(
				(job: Job) => job.location_type.toLowerCase() === activeLocationType.toLowerCase()
			);
		}

		// Employment type filter
		if (activeEmploymentType !== 'All Types') {
			result = result.filter(
				(job: Job) => job.employment_type.toLowerCase() === activeEmploymentType.toLowerCase()
			);
		}

		return result;
	});
</script>

<div class="flex h-screen bg-background">
	<Sidebar bind:collapsed={sidebarCollapsed} />

	<main class="flex-1 overflow-hidden">
		<header class="flex h-14 items-center justify-between border-b border-border px-6">
			<div class="flex items-center gap-2">
				<Briefcase class="h-5 w-5" />
				<h1 class="text-xl font-semibold">Job Postings</h1>
			</div>
			<div class="flex items-center gap-2">
				<Button variant="outline" size="sm">
					<Filter class="mr-2 h-4 w-4" />
					Advanced Filters
				</Button>
				<Button size="sm" onclick={() => (postJobDialogOpen = true)}>
					<Plus class="mr-2 h-4 w-4" />
					Post a Job
				</Button>
			</div>
		</header>

		<div class="p-6">
			<div class="mb-6 space-y-4">
				<!-- Search and view toggle -->
				<div class="flex items-center gap-4">
					<div class="relative max-w-xl flex-1">
						<Search
							class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground"
						/>
						<Input
							type="search"
							placeholder="Search jobs by title, company, or skills..."
							class="pl-10"
							bind:value={searchQuery}
						/>
					</div>
					<div class="flex items-center gap-1 rounded-lg border border-border p-1">
						<Button
							variant={viewMode === 'grid' ? 'secondary' : 'ghost'}
							size="icon"
							class="h-8 w-8"
							onclick={() => (viewMode = 'grid')}
						>
							<Grid3X3 class="h-4 w-4" />
						</Button>
						<Button
							variant={viewMode === 'list' ? 'secondary' : 'ghost'}
							size="icon"
							class="h-8 w-8"
							onclick={() => (viewMode = 'list')}
						>
							<List class="h-4 w-4" />
						</Button>
					</div>
				</div>

				<!-- Location type filters -->
				<div class="flex items-center gap-2">
					<span class="text-sm font-medium text-muted-foreground">Location:</span>
					{#each locationTypes as locationType}
						<Button
							variant={activeLocationType === locationType ? 'default' : 'outline'}
							size="sm"
							onclick={() => (activeLocationType = locationType)}
						>
							{locationType}
						</Button>
					{/each}
				</div>

				<!-- Employment type filters -->
				<div class="flex items-center gap-2">
					<span class="text-sm font-medium text-muted-foreground">Type:</span>
					{#each employmentTypes as empType}
						<Button
							variant={activeEmploymentType === empType ? 'default' : 'outline'}
							size="sm"
							onclick={() => (activeEmploymentType = empType)}
						>
							{empType}
						</Button>
					{/each}
				</div>
			</div>

			<!-- Results header -->
			<div class="mb-4 flex items-center justify-between">
				<p class="text-sm text-muted-foreground">
					Showing <span class="font-medium text-foreground">{filteredJobs().length}</span>
					{filteredJobs().length === 1 ? 'job' : 'jobs'}
					{#if searchQuery || activeLocationType !== 'All' || activeEmploymentType !== 'All Types'}
						<button
							class="ml-2 text-primary hover:underline"
							onclick={() => {
								searchQuery = '';
								activeLocationType = 'All';
								activeEmploymentType = 'All Types';
							}}
						>
							Clear filters
						</button>
					{/if}
				</p>
				<Button variant="ghost" size="sm">
					<SlidersHorizontal class="mr-2 h-4 w-4" />
					Sort by: Newest
				</Button>
			</div>

			<!-- Jobs grid/list -->
			<ScrollArea class="h-[calc(100vh-300px)]">
				{#if filteredJobs().length > 0}
					<div
						class={viewMode === 'grid'
							? 'grid gap-4 sm:grid-cols-2 lg:grid-cols-3'
							: 'flex flex-col gap-4'}
					>
						{#each filteredJobs() as job (job.id)}
							<JobCard {job} />
						{/each}
					</div>
				{:else}
					<div class="flex flex-col items-center justify-center py-16 text-center">
						<Briefcase class="mb-4 h-12 w-12 text-muted-foreground/50" />
						<h3 class="mb-2 text-lg font-medium">No jobs found</h3>
						<p class="mb-4 text-sm text-muted-foreground">
							{#if searchQuery || activeLocationType !== 'All' || activeEmploymentType !== 'All Types'}
								Try adjusting your search or filters
							{:else}
								No job postings available at the moment
							{/if}
						</p>
						{#if searchQuery || activeLocationType !== 'All' || activeEmploymentType !== 'All Types'}
							<Button
								variant="outline"
								onclick={() => {
									searchQuery = '';
									activeLocationType = 'All';
									activeEmploymentType = 'All Types';
								}}
							>
								Clear all filters
							</Button>
						{:else}
							<Button onclick={() => (postJobDialogOpen = true)}>
								<Plus class="mr-2 h-4 w-4" />
								Post the first job
							</Button>
						{/if}
					</div>
				{/if}
			</ScrollArea>
		</div>
	</main>
</div>

<PostJobDialog bind:open={postJobDialogOpen} onSuccess={handleJobCreated} />
