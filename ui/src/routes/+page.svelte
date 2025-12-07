<script lang="ts">
	import { Search, Filter, SlidersHorizontal, Grid3X3, List, Trash2, CheckSquare, X } from 'lucide-svelte';
	import Sidebar from '$lib/components/sidebar.svelte';
	import TalentCard from '$lib/components/talent-card.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Badge } from '$lib/components/ui/badge';
	import * as Tabs from '$lib/components/ui/tabs';
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import { Separator } from '$lib/components/ui/separator';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { Checkbox } from '$lib/components/ui/checkbox';
	import { invalidateAll } from '$app/navigation';

	let { data } = $props();

	let sidebarCollapsed = $state(false);
	let searchQuery = $state('');
	let viewMode = $state<'grid' | 'list'>('grid');
	let selectionMode = $state(false);
	let selectedTalents = $state<Set<string>>(new Set());
	let deleteDialogOpen = $state(false);
	let isDeleting = $state(false);

	const talents = $derived(data.talents ?? []);
	const selectedCount = $derived(selectedTalents.size);
	const allSelected = $derived(talents.length > 0 && selectedTalents.size === talents.length);

	const categories = ['All', 'Engineering', 'Design', 'Product', 'Data', 'Marketing'];
	let activeCategory = $state('All');

	function handleSelectionChange(id: string, selected: boolean) {
		if (selected) {
			selectedTalents.add(id);
		} else {
			selectedTalents.delete(id);
		}
		selectedTalents = new Set(selectedTalents);
	}

	function toggleSelectAll() {
		if (allSelected) {
			selectedTalents = new Set();
		} else {
			selectedTalents = new Set(talents.map((t) => t.id));
		}
	}

	function exitSelectionMode() {
		selectionMode = false;
		selectedTalents = new Set();
	}

	async function bulkDelete() {
		if (selectedTalents.size === 0) return;

		isDeleting = true;
		try {
			const response = await fetch('http://localhost:8080/api/v1/talents/bulk-delete', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({ ids: Array.from(selectedTalents) })
			});

			if (response.ok) {
				const result = await response.json();
				console.log(`Deleted ${result.deleted_count} of ${result.total_requested} talents`);
				deleteDialogOpen = false;
				exitSelectionMode();
				await invalidateAll();
			} else {
				console.error('Failed to delete talents:', response.statusText);
			}
		} catch (error) {
			console.error('Error deleting talents:', error);
		} finally {
			isDeleting = false;
		}
	}
</script>

<div class="flex h-screen bg-background">
	<Sidebar bind:collapsed={sidebarCollapsed} />

	<main class="flex-1 overflow-hidden">
		<header class="flex h-14 items-center justify-between border-b border-border px-6">
			<h1 class="text-xl font-semibold">Talent Pool</h1>
			<div class="flex items-center gap-2">
				{#if selectionMode}
					<div class="flex items-center gap-2 mr-4">
						<span class="text-sm text-muted-foreground">
							{selectedCount} selected
						</span>
						<Button
							variant="ghost"
							size="sm"
							onclick={toggleSelectAll}
						>
							{allSelected ? 'Deselect All' : 'Select All'}
						</Button>
					</div>
					<Button
						variant="destructive"
						size="sm"
						disabled={selectedCount === 0}
						onclick={() => (deleteDialogOpen = true)}
					>
						<Trash2 class="mr-2 h-4 w-4" />
						Delete Selected
					</Button>
					<Button variant="outline" size="sm" onclick={exitSelectionMode}>
						<X class="mr-2 h-4 w-4" />
						Cancel
					</Button>
				{:else}
					<Button variant="outline" size="sm" onclick={() => (selectionMode = true)}>
						<CheckSquare class="mr-2 h-4 w-4" />
						Select
					</Button>
					<Button variant="outline" size="sm">
						<Filter class="mr-2 h-4 w-4" />
						Filters
					</Button>
					<Button size="sm">Post a Job</Button>
				{/if}
			</div>
		</header>

		<div class="p-6">
			<div class="mb-6 space-y-4">
				<div class="flex items-center gap-4">
					<div class="relative flex-1 max-w-xl">
						<Search class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
						<Input
							type="search"
							placeholder="Search by name, skills, or title..."
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

				<div class="flex items-center gap-2">
					{#each categories as category}
						<Button
							variant={activeCategory === category ? 'default' : 'outline'}
							size="sm"
							onclick={() => (activeCategory = category)}
						>
							{category}
						</Button>
					{/each}
				</div>
			</div>

			<div class="flex items-center justify-between mb-4">
				<p class="text-sm text-muted-foreground">
					Showing <span class="font-medium text-foreground">{talents.length}</span> candidates
				</p>
				<Button variant="ghost" size="sm">
					<SlidersHorizontal class="mr-2 h-4 w-4" />
					Sort by: Relevance
				</Button>
			</div>

			<ScrollArea class="h-[calc(100vh-260px)]">
				<div
					class={viewMode === 'grid'
						? 'grid gap-4 sm:grid-cols-2 lg:grid-cols-3'
						: 'flex flex-col gap-4'}
				>
					{#each talents as talent (talent.id)}
						<TalentCard
							{talent}
							selectable={selectionMode}
							selected={selectedTalents.has(talent.id)}
							onSelectionChange={handleSelectionChange}
						/>
					{/each}
				</div>
			</ScrollArea>
		</div>
	</main>
</div>

<AlertDialog.Root bind:open={deleteDialogOpen}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>Delete {selectedCount} talent{selectedCount !== 1 ? 's' : ''}?</AlertDialog.Title>
			<AlertDialog.Description>
				This action cannot be undone. This will permanently delete the selected talent{selectedCount !== 1 ? 's' : ''} and remove their data from the system.
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
