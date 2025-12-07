<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '$lib/components/ui/card';
	import { Building2, Calendar, Link, Trash2, Edit } from 'lucide-svelte';
	import type { HiringRequirement, Job } from '$lib/types';

	let {
		requirement,
		job,
		onEdit,
		onDelete
	}: {
		requirement: HiringRequirement;
		job?: Job;
		onEdit?: (requirement: HiringRequirement) => void;
		onDelete?: (id: string) => void;
	} = $props();

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}
</script>

<Card class="h-full">
	<CardHeader class="pb-3">
		<div class="flex items-start justify-between gap-2">
			<div class="flex-1 min-w-0">
				<CardTitle class="text-lg line-clamp-1">{requirement.title}</CardTitle>
				<CardDescription class="flex items-center gap-1 mt-1">
					<Building2 class="h-3 w-3" />
					{requirement.company_name}
				</CardDescription>
			</div>
			<div class="flex gap-1">
				{#if onEdit}
					<Button
						variant="ghost"
						size="icon"
						class="h-8 w-8"
						onclick={() => onEdit?.(requirement)}
					>
						<Edit class="h-4 w-4" />
					</Button>
				{/if}
				{#if onDelete}
					<Button
						variant="ghost"
						size="icon"
						class="h-8 w-8 text-destructive hover:text-destructive"
						onclick={() => onDelete?.(requirement.id)}
					>
						<Trash2 class="h-4 w-4" />
					</Button>
				{/if}
			</div>
		</div>
	</CardHeader>
	<CardContent class="space-y-3">
		{#if job}
			<div class="flex items-center gap-2 text-sm text-primary">
				<Link class="h-3 w-3" />
				<span>Linked to: {job.title}</span>
			</div>
		{/if}

		<div class="rounded-md bg-muted/50 p-3">
			<p class="text-sm whitespace-pre-wrap line-clamp-6">{requirement.requirements_text}</p>
		</div>

		<div class="flex items-center gap-1 text-xs text-muted-foreground">
			<Calendar class="h-3 w-3" />
			<span>Created {formatDate(requirement.created_at)}</span>
		</div>
	</CardContent>
</Card>
