<script lang="ts">
	import { MapPin, Briefcase, Star, MoreHorizontal, ExternalLink } from 'lucide-svelte';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import * as Avatar from '$lib/components/ui/avatar';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import TalentDetailDialog from './talent-detail-dialog.svelte';
	import CandidateFeedback from './candidate-feedback.svelte';
	import type { Talent } from '$lib/types';

	interface Props {
		talent: Talent;
		jobId?: string;
		rankPosition?: number;
		showFeedback?: boolean;
	}

	let { talent, jobId, rankPosition, showFeedback = false }: Props = $props();
	let isSaved = $state<boolean | null>(null);
	let detailDialogOpen = $state(false);

	// Parse skills from comma-separated string or array
	const skills = $derived(() => {
		if (!talent.skills) return [];
		if (Array.isArray(talent.skills)) return talent.skills;
		return (talent.skills as unknown as string).split(',').map((s: string) => s.trim()).filter((s: string) => s);
	});

	$effect(() => {
		if (isSaved === null) {
			isSaved = talent.saved ?? false;
		}
	});
</script>

<Card.Root class="group relative overflow-hidden transition-shadow hover:shadow-lg">
	<Card.Header class="pb-3">
		<div class="flex items-start justify-between">
			<div class="flex items-center gap-3">
				<Avatar.Root class="h-12 w-12">
					<Avatar.Image src={talent.avatar} alt={talent.name} />
					<Avatar.Fallback>{talent.name.slice(0, 2).toUpperCase()}</Avatar.Fallback>
				</Avatar.Root>
				<div>
					<div class="flex items-center gap-2">
						<Card.Title class="text-base">{talent.name}</Card.Title>
						{#if talent.verified}
							<Badge variant="secondary" class="h-5 px-1.5 text-xs">
								Verified
							</Badge>
						{/if}
					</div>
					<Card.Description class="text-sm">@{talent.handle}</Card.Description>
				</div>
			</div>
			<div class="flex items-center gap-1">
				<Button
					variant="ghost"
					size="icon"
					class="h-8 w-8"
					onclick={() => (isSaved = !isSaved)}
				>
					<Star class="h-4 w-4 {isSaved ? 'fill-yellow-400 text-yellow-400' : ''}" />
				</Button>
				<DropdownMenu.Root>
					<DropdownMenu.Trigger>
						<Button variant="ghost" size="icon" class="h-8 w-8">
							<MoreHorizontal class="h-4 w-4" />
						</Button>
					</DropdownMenu.Trigger>
					<DropdownMenu.Content align="end">
						<DropdownMenu.Item>
							<ExternalLink class="mr-2 h-4 w-4" />
							View Profile
						</DropdownMenu.Item>
						<DropdownMenu.Item>Share</DropdownMenu.Item>
						<DropdownMenu.Separator />
						<DropdownMenu.Item class="text-destructive">Report</DropdownMenu.Item>
					</DropdownMenu.Content>
				</DropdownMenu.Root>
			</div>
		</div>
	</Card.Header>
	<Card.Content class="space-y-3">
		<p class="text-sm font-medium">{talent.title}</p>
		<p class="line-clamp-2 text-sm text-muted-foreground">{talent.bio}</p>
		<div class="flex flex-wrap gap-2">
			{#each skills().slice(0, 4) as skill}
				<Badge variant="outline" class="text-xs">{skill}</Badge>
			{/each}
			{#if skills().length > 4}
				<Badge variant="outline" class="text-xs">+{skills().length - 4}</Badge>
			{/if}
		</div>
		<div class="flex items-center gap-4 text-xs text-muted-foreground">
			{#if talent.location}
				<span class="flex items-center gap-1">
					<MapPin class="h-3 w-3" />
					{talent.location}
				</span>
			{/if}
			<span class="flex items-center gap-1">
				<Briefcase class="h-3 w-3" />
				{talent.experience}
			</span>
		</div>
	</Card.Content>
	<Card.Footer class="flex flex-col gap-2 pt-3">
		{#if showFeedback && jobId}
			<div class="flex items-center justify-between border-t pt-2 -mx-6 px-6 -mb-2">
				<div class="flex items-center gap-2">
					{#if rankPosition}
						<Badge variant="secondary" class="text-xs">
							Rank #{rankPosition}
						</Badge>
					{/if}
					<span class="text-xs text-muted-foreground">Feedback:</span>
				</div>
				<CandidateFeedback
					talentId={talent.id}
					{jobId}
					expectedRank={rankPosition}
					showStats={true}
				/>
			</div>
		{/if}
		<div class="flex gap-2">
			<Button variant="outline" class="flex-1" size="sm" onclick={() => (detailDialogOpen = true)}>
				Show More
			</Button>
			<Button class="flex-1" size="sm">Contact</Button>
		</div>
	</Card.Footer>
</Card.Root>

<TalentDetailDialog {talent} bind:open={detailDialogOpen} />
