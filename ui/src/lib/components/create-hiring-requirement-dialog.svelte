<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import * as Select from '$lib/components/ui/select';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Loader2 } from 'lucide-svelte';
	import type { Job } from '$lib/types';

	let {
		open = $bindable(false),
		jobs = [],
		onSuccess
	}: { open: boolean; jobs: Job[]; onSuccess?: () => void } = $props();

	let isSubmitting = $state(false);
	let error = $state<string | null>(null);

	// Form fields
	let title = $state('');
	let companyName = $state('');
	let requirementsText = $state('');
	let selectedJobId = $state<string | undefined>(undefined);

	// Helper to get label from value
	const getJobLabel = (jobId: string | undefined) => {
		if (!jobId) return 'No linked job (standalone)';
		const job = jobs.find((j) => j.id === jobId);
		return job ? `${job.title} at ${job.company_name}` : 'Select a job';
	};

	function resetForm() {
		title = '';
		companyName = '';
		requirementsText = '';
		selectedJobId = undefined;
		error = null;
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();
		error = null;

		// Validation
		if (!title.trim()) {
			error = 'Title is required';
			return;
		}
		if (!companyName.trim()) {
			error = 'Company name is required';
			return;
		}
		if (!requirementsText.trim()) {
			error = 'Requirements text is required';
			return;
		}

		isSubmitting = true;

		try {
			const payload = {
				job_id: selectedJobId || null,
				title: title.trim(),
				company_name: companyName.trim(),
				requirements_text: requirementsText.trim()
			};

			const response = await fetch('http://localhost:8080/api/v1/hiring-requirements', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(payload)
			});

			if (!response.ok) {
				const errorData = await response.text();
				throw new Error(errorData || 'Failed to create hiring requirement');
			}

			resetForm();
			open = false;
			onSuccess?.();
		} catch (err) {
			error = err instanceof Error ? err.message : 'An error occurred';
		} finally {
			isSubmitting = false;
		}
	}
</script>

<Dialog.Root bind:open onOpenChange={(isOpen) => !isOpen && resetForm()}>
	<Dialog.Content class="max-h-[90vh] max-w-2xl overflow-y-auto">
		<Dialog.Header>
			<Dialog.Title>Add Hiring Requirements</Dialog.Title>
			<Dialog.Description>
				Define what you're looking for in candidates. This will help match the right talent to your
				needs.
			</Dialog.Description>
		</Dialog.Header>

		<form onsubmit={handleSubmit} class="space-y-6">
			{#if error}
				<div class="rounded-md bg-destructive/10 p-3 text-sm text-destructive">
					{error}
				</div>
			{/if}

			<div class="space-y-4">
				<div class="space-y-2">
					<Label for="title">Requirement Title *</Label>
					<Input
						id="title"
						placeholder="e.g., Senior Backend Engineer - Q1 2025"
						bind:value={title}
					/>
					<p class="text-xs text-muted-foreground">A descriptive name for these requirements</p>
				</div>

				<div class="space-y-2">
					<Label for="companyName">Company Name *</Label>
					<Input id="companyName" placeholder="e.g., xAI" bind:value={companyName} />
				</div>

				<div class="space-y-2">
					<Label>Link to Job (Optional)</Label>
					<Select.Root type="single" bind:value={selectedJobId}>
						<Select.Trigger class="w-full">
							{getJobLabel(selectedJobId)}
						</Select.Trigger>
						<Select.Content>
							<Select.Item value="" label="No linked job (standalone)"
								>No linked job (standalone)</Select.Item
							>
							{#each jobs as job}
								<Select.Item value={job.id} label={`${job.title} at ${job.company_name}`}>
									{job.title} at {job.company_name}
								</Select.Item>
							{/each}
						</Select.Content>
					</Select.Root>
					<p class="text-xs text-muted-foreground">
						Optionally link these requirements to a specific job posting
					</p>
				</div>

				<div class="space-y-2">
					<Label for="requirementsText">Requirements *</Label>
					<Textarea
						id="requirementsText"
						placeholder="Describe what you're looking for in a candidate...

Example:
- 5+ years of experience in backend development
- Strong knowledge of Python or Go
- Experience with distributed systems
- Excellent communication skills
- Passion for AI/ML"
						rows={10}
						bind:value={requirementsText}
					/>
					<p class="text-xs text-muted-foreground">
						Free-form text describing skills, experience, qualifications, and any other criteria
					</p>
				</div>
			</div>

			<Dialog.Footer>
				<Button
					type="button"
					variant="outline"
					onclick={() => (open = false)}
					disabled={isSubmitting}
				>
					Cancel
				</Button>
				<Button type="submit" disabled={isSubmitting}>
					{#if isSubmitting}
						<Loader2 class="mr-2 h-4 w-4 animate-spin" />
						Saving...
					{:else}
						Save Requirements
					{/if}
				</Button>
			</Dialog.Footer>
		</form>
	</Dialog.Content>
</Dialog.Root>
