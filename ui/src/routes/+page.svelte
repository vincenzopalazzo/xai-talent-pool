<script lang="ts">
	import { Search, Filter, SlidersHorizontal, Grid3X3, List } from 'lucide-svelte';
	import Sidebar from '$lib/components/sidebar.svelte';
	import TalentCard from '$lib/components/talent-card.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Badge } from '$lib/components/ui/badge';
	import * as Tabs from '$lib/components/ui/tabs';
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import { Separator } from '$lib/components/ui/separator';

	let { data } = $props();

	let sidebarCollapsed = $state(false);
	let searchQuery = $state('');
	let viewMode = $state<'grid' | 'list'>('grid');

	const talents = $derived(data.talents ?? []);

	const categories = ['All', 'Engineering', 'Design', 'Product', 'Data', 'Marketing'];
	let activeCategory = $state('All');

	$effect(() => {
		console.log('Search query:', searchQuery);
	});
</script>

<div class="flex h-screen bg-background">
	<Sidebar bind:collapsed={sidebarCollapsed} />

	<main class="flex-1 overflow-hidden">
		<header class="flex h-14 items-center justify-between border-b border-border px-6">
			<h1 class="text-xl font-semibold">Talent Pool</h1>
			<div class="flex items-center gap-2">
				<Button variant="outline" size="sm">
					<Filter class="mr-2 h-4 w-4" />
					Filters
				</Button>
				<Button size="sm">Post a Job</Button>
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
						<TalentCard {talent} />
					{/each}
				</div>
			</ScrollArea>
		</div>
	</main>
</div>
