<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import * as Avatar from '$lib/components/ui/avatar';
	import { Separator } from '$lib/components/ui/separator';
	import {
		MapPin,
		Briefcase,
		DollarSign,
		Building2,
		Clock,
		Calendar,
		ExternalLink,
		Star
	} from 'lucide-svelte';
	import ApplyJobDialog from './apply-job-dialog.svelte';
	import type { Job } from '$lib/types';

	let {
		job,
		open = $bindable(false)
	}: {
		job: Job;
		open: boolean;
	} = $props();

	let applyDialogOpen = $state(false);

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

	// Format date
	const formatDate = (dateStr: string | undefined) => {
		if (!dateStr) return null;
		return new Date(dateStr).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	};
</script>

<Dialog.Root bind:open>
	<Dialog.Content class="max-h-[90vh] max-w-4xl overflow-y-auto sm:max-w-3xl md:max-w-4xl lg:max-w-5xl">
		<Dialog.Header>
			<div class="flex items-start gap-4">
				<Avatar.Root class="h-16 w-16 rounded-lg">
					{#if job.company_logo}
						<Avatar.Image src={job.company_logo} alt={job.company_name} class="rounded-lg" />
					{/if}
					<Avatar.Fallback class="rounded-lg bg-primary/10">
						<Building2 class="h-8 w-8 text-primary" />
					</Avatar.Fallback>
				</Avatar.Root>
				<div class="flex-1">
					<Dialog.Title class="text-xl">{job.title}</Dialog.Title>
					<Dialog.Description class="flex items-center gap-1 text-base">
						<Building2 class="h-4 w-4" />
						{job.company_name}
					</Dialog.Description>
				</div>
			</div>
		</Dialog.Header>

		<div class="space-y-6 py-4">
			<!-- Status badges -->
			<div class="flex flex-wrap items-center gap-2">
				<Badge variant={locationTypeVariant()} class="capitalize">
					{job.location_type}
				</Badge>
				<Badge variant="outline" class="capitalize">
					{job.employment_type?.replace('-', ' ')}
				</Badge>
				{#if job.status === 'active'}
					<Badge
						variant="secondary"
						class="bg-green-100 text-green-700 dark:bg-green-900 dark:text-green-300"
					>
						Active
					</Badge>
				{/if}
			</div>

			<!-- Key details -->
			<div class="grid gap-3 rounded-lg bg-muted/50 p-4 sm:grid-cols-2">
				{#if job.location}
					<div class="flex items-center gap-2 text-sm">
						<MapPin class="h-4 w-4 text-muted-foreground" />
						<span>{job.location}</span>
					</div>
				{/if}
				<div class="flex items-center gap-2 text-sm">
					<Briefcase class="h-4 w-4 text-muted-foreground" />
					<span>{experienceDisplay()}</span>
				</div>
				{#if salaryDisplay()}
					<div class="flex items-center gap-2 text-sm font-medium">
						<DollarSign class="h-4 w-4 text-muted-foreground" />
						<span>{salaryDisplay()}</span>
					</div>
				{/if}
				{#if job.created_at}
					<div class="flex items-center gap-2 text-sm text-muted-foreground">
						<Calendar class="h-4 w-4" />
						<span>Posted {formatDate(job.created_at)}</span>
					</div>
				{/if}
				{#if job.expires_at}
					<div class="flex items-center gap-2 text-sm text-muted-foreground">
						<Clock class="h-4 w-4" />
						<span>Expires {formatDate(job.expires_at)}</span>
					</div>
				{/if}
			</div>

			<Separator />

			<!-- Description -->
			<div class="space-y-2">
				<h3 class="text-sm font-semibold">About the Role</h3>
				<p class="whitespace-pre-wrap text-sm text-muted-foreground">{job.description}</p>
			</div>

			<!-- Skills -->
			{#if skills.length > 0}
				<div class="space-y-2">
					<h3 class="text-sm font-semibold">Required Skills</h3>
					<div class="flex flex-wrap gap-2">
						{#each skills as skill}
							<Badge variant="outline">{skill}</Badge>
						{/each}
					</div>
				</div>
			{/if}
		</div>

		<Dialog.Footer class="flex-col gap-2 sm:flex-row">
			<Button variant="outline" class="w-full sm:w-auto" onclick={() => (open = false)}>
				Close
			</Button>
			<Button class="w-full sm:flex-1" onclick={() => (applyDialogOpen = true)}>
				<ExternalLink class="mr-2 h-4 w-4" />
				Apply Now
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

<ApplyJobDialog {job} bind:open={applyDialogOpen} />
