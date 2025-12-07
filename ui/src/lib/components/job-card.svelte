<script lang="ts">
	import {
		MapPin,
		Briefcase,
		Clock,
		DollarSign,
		Star,
		MoreHorizontal,
		ExternalLink,
		Building2
	} from 'lucide-svelte';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import * as Avatar from '$lib/components/ui/avatar';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import JobDetailsDialog from './job-details-dialog.svelte';
	import type { Job } from '$lib/types';

	let { job }: { job: Job } = $props();
	let isSaved = $state<boolean | null>(null);
	let detailsDialogOpen = $state(false);

	$effect(() => {
		if (isSaved === null) {
			isSaved = job.saved ?? false;
		}
	});

	// Parse skills from comma-separated string
	const skills = $derived(
		job.skills_required ? job.skills_required.split(',').map((s) => s.trim()) : []
	);

	// Format salary
	const salaryDisplay = $derived(() => {
		if (!job.salary_min && !job.salary_max) return null;
		const currency = job.salary_currency || 'USD';
		const formatter = new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency,
			maximumFractionDigits: 0
		});
		if (job.salary_min && job.salary_max) {
			return `${formatter.format(job.salary_min)} - ${formatter.format(job.salary_max)}`;
		}
		if (job.salary_min) return `From ${formatter.format(job.salary_min)}`;
		if (job.salary_max) return `Up to ${formatter.format(job.salary_max)}`;
		return null;
	});

	// Location type badge variant
	const locationTypeVariant = $derived(() => {
		switch (job.location_type?.toLowerCase()) {
			case 'remote':
				return 'default';
			case 'hybrid':
				return 'secondary';
			default:
				return 'outline';
		}
	});

	// Experience level display
	const experienceDisplay = $derived(() => {
		const levels: Record<string, string> = {
			entry: 'Entry Level',
			mid: 'Mid Level',
			senior: 'Senior',
			lead: 'Lead / Principal'
		};
		return levels[job.experience_level?.toLowerCase()] || job.experience_level;
	});
</script>

<Card.Root class="group relative overflow-hidden transition-shadow hover:shadow-lg">
	<Card.Header class="pb-3">
		<div class="flex items-start justify-between">
			<div class="flex items-center gap-3">
				<Avatar.Root class="h-12 w-12 rounded-lg">
					{#if job.company_logo}
						<Avatar.Image src={job.company_logo} alt={job.company_name} class="rounded-lg" />
					{/if}
					<Avatar.Fallback class="rounded-lg bg-primary/10">
						<Building2 class="h-6 w-6 text-primary" />
					</Avatar.Fallback>
				</Avatar.Root>
				<div class="min-w-0 flex-1">
					<Card.Title class="truncate text-base">{job.title}</Card.Title>
					<Card.Description class="flex items-center gap-1 text-sm">
						<Building2 class="h-3 w-3" />
						{job.company_name}
					</Card.Description>
				</div>
			</div>
			<div class="flex items-center gap-1">
				<Button variant="ghost" size="icon" class="h-8 w-8" onclick={() => (isSaved = !isSaved)}>
					<Star class="h-4 w-4 {isSaved ? 'fill-yellow-400 text-yellow-400' : ''}" />
				</Button>
				<DropdownMenu.Root>
					<DropdownMenu.Trigger>
						<Button variant="ghost" size="icon" class="h-8 w-8">
							<MoreHorizontal class="h-4 w-4" />
						</Button>
					</DropdownMenu.Trigger>
					<DropdownMenu.Content align="end">
						<DropdownMenu.Item onclick={() => (detailsDialogOpen = true)}>
							<ExternalLink class="mr-2 h-4 w-4" />
							View Details
						</DropdownMenu.Item>
						<DropdownMenu.Item>Share Job</DropdownMenu.Item>
						<DropdownMenu.Separator />
						<DropdownMenu.Item class="text-destructive">Report</DropdownMenu.Item>
					</DropdownMenu.Content>
				</DropdownMenu.Root>
			</div>
		</div>
	</Card.Header>

	<Card.Content class="space-y-3">
		<!-- Badges row -->
		<div class="flex flex-wrap items-center gap-2">
			<Badge variant={locationTypeVariant()} class="text-xs capitalize">
				{job.location_type}
			</Badge>
			<Badge variant="outline" class="text-xs capitalize">
				{job.employment_type?.replace('-', ' ')}
			</Badge>
			{#if job.status === 'active'}
				<Badge variant="secondary" class="bg-green-100 text-green-700 text-xs dark:bg-green-900 dark:text-green-300">
					Active
				</Badge>
			{/if}
		</div>

		<!-- Description -->
		<p class="line-clamp-2 text-sm text-muted-foreground">{job.description}</p>

		<!-- Skills -->
		<div class="flex flex-wrap gap-2">
			{#each skills.slice(0, 4) as skill}
				<Badge variant="outline" class="text-xs">{skill}</Badge>
			{/each}
			{#if skills.length > 4}
				<Badge variant="outline" class="text-xs">+{skills.length - 4}</Badge>
			{/if}
		</div>

		<!-- Info row -->
		<div class="flex flex-wrap items-center gap-4 text-xs text-muted-foreground">
			{#if job.location}
				<span class="flex items-center gap-1">
					<MapPin class="h-3 w-3" />
					{job.location}
				</span>
			{/if}
			<span class="flex items-center gap-1">
				<Briefcase class="h-3 w-3" />
				{experienceDisplay()}
			</span>
			{#if salaryDisplay()}
				<span class="flex items-center gap-1 font-medium text-foreground">
					<DollarSign class="h-3 w-3" />
					{salaryDisplay()}
				</span>
			{/if}
		</div>
	</Card.Content>

	<Card.Footer class="flex gap-2 pt-3">
		<Button variant="outline" class="flex-1" size="sm" onclick={() => (detailsDialogOpen = true)}>
			Learn More
		</Button>
		<Button class="flex-1" size="sm">Apply Now</Button>
	</Card.Footer>
</Card.Root>

<JobDetailsDialog {job} bind:open={detailsDialogOpen} />
