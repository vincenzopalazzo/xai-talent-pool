<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import * as Select from '$lib/components/ui/select';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Loader2 } from 'lucide-svelte';

	let { open = $bindable(false), onSuccess }: { open: boolean; onSuccess?: () => void } = $props();

	let isSubmitting = $state(false);
	let error = $state<string | null>(null);

	// Form fields
	let title = $state('');
	let description = $state('');
	let companyName = $state('');
	let companyLogo = $state('');
	let location = $state('');
	let locationType = $state<string | undefined>(undefined);
	let employmentType = $state<string | undefined>(undefined);
	let salaryMin = $state('');
	let salaryMax = $state('');
	let salaryCurrency = $state<string | undefined>('USD');
	let skillsRequired = $state('');
	let experienceLevel = $state<string | undefined>(undefined);
	let expiresAt = $state('');

	const locationTypes = [
		{ value: 'remote', label: 'Remote' },
		{ value: 'onsite', label: 'Onsite' },
		{ value: 'hybrid', label: 'Hybrid' }
	];

	const employmentTypes = [
		{ value: 'full-time', label: 'Full-time' },
		{ value: 'part-time', label: 'Part-time' },
		{ value: 'contract', label: 'Contract' }
	];

	const experienceLevels = [
		{ value: 'entry', label: 'Entry Level' },
		{ value: 'mid', label: 'Mid Level' },
		{ value: 'senior', label: 'Senior' },
		{ value: 'lead', label: 'Lead / Principal' }
	];

	const currencies = [
		{ value: 'USD', label: 'USD' },
		{ value: 'EUR', label: 'EUR' },
		{ value: 'GBP', label: 'GBP' }
	];

	// Helper to get label from value
	const getLabel = (options: { value: string; label: string }[], value: string | undefined) =>
		options.find((o) => o.value === value)?.label ?? '';

	function resetForm() {
		title = '';
		description = '';
		companyName = '';
		companyLogo = '';
		location = '';
		locationType = undefined;
		employmentType = undefined;
		salaryMin = '';
		salaryMax = '';
		salaryCurrency = 'USD';
		skillsRequired = '';
		experienceLevel = undefined;
		expiresAt = '';
		error = null;
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();
		error = null;

		// Validation
		if (!title.trim()) {
			error = 'Job title is required';
			return;
		}
		if (!description.trim()) {
			error = 'Job description is required';
			return;
		}
		if (!companyName.trim()) {
			error = 'Company name is required';
			return;
		}
		if (!locationType) {
			error = 'Location type is required';
			return;
		}
		if (!employmentType) {
			error = 'Employment type is required';
			return;
		}
		if (!experienceLevel) {
			error = 'Experience level is required';
			return;
		}

		isSubmitting = true;

		try {
			const payload = {
				title: title.trim(),
				description: description.trim(),
				company_name: companyName.trim(),
				company_logo: companyLogo.trim() || null,
				location: location.trim() || null,
				location_type: locationType,
				employment_type: employmentType,
				salary_min: salaryMin ? parseInt(salaryMin) : null,
				salary_max: salaryMax ? parseInt(salaryMax) : null,
				salary_currency: salaryCurrency || null,
				skills_required: skillsRequired.trim(),
				experience_level: experienceLevel,
				expires_at: expiresAt || null
			};

			const response = await fetch('http://localhost:8080/api/v1/jobs', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(payload)
			});

			if (!response.ok) {
				const errorData = await response.text();
				throw new Error(errorData || 'Failed to create job posting');
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
			<Dialog.Title>Post a New Job</Dialog.Title>
			<Dialog.Description>
				Fill in the details below to create a new job posting. Required fields are marked with *.
			</Dialog.Description>
		</Dialog.Header>

		<form onsubmit={handleSubmit} class="space-y-6">
			{#if error}
				<div class="rounded-md bg-destructive/10 p-3 text-sm text-destructive">
					{error}
				</div>
			{/if}

			<!-- Job Details Section -->
			<div class="space-y-4">
				<h3 class="text-sm font-medium text-muted-foreground">Job Details</h3>

				<div class="space-y-2">
					<Label for="title">Job Title *</Label>
					<Input id="title" placeholder="e.g., Senior Frontend Developer" bind:value={title} />
				</div>

				<div class="space-y-2">
					<Label for="description">Description *</Label>
					<Textarea
						id="description"
						placeholder="Describe the role, responsibilities, and what you're looking for..."
						rows={4}
						bind:value={description}
					/>
				</div>

				<div class="grid gap-4 sm:grid-cols-2">
					<div class="space-y-2">
						<Label>Location Type *</Label>
						<Select.Root type="single" bind:value={locationType}>
							<Select.Trigger class="w-full">
								{getLabel(locationTypes, locationType) || 'Select location type'}
							</Select.Trigger>
							<Select.Content>
								{#each locationTypes as lt}
									<Select.Item value={lt.value} label={lt.label}>{lt.label}</Select.Item>
								{/each}
							</Select.Content>
						</Select.Root>
					</div>

					<div class="space-y-2">
						<Label for="location">Location</Label>
						<Input id="location" placeholder="e.g., San Francisco, CA" bind:value={location} />
					</div>
				</div>

				<div class="grid gap-4 sm:grid-cols-2">
					<div class="space-y-2">
						<Label>Employment Type *</Label>
						<Select.Root type="single" bind:value={employmentType}>
							<Select.Trigger class="w-full">
								{getLabel(employmentTypes, employmentType) || 'Select employment type'}
							</Select.Trigger>
							<Select.Content>
								{#each employmentTypes as et}
									<Select.Item value={et.value} label={et.label}>{et.label}</Select.Item>
								{/each}
							</Select.Content>
						</Select.Root>
					</div>

					<div class="space-y-2">
						<Label>Experience Level *</Label>
						<Select.Root type="single" bind:value={experienceLevel}>
							<Select.Trigger class="w-full">
								{getLabel(experienceLevels, experienceLevel) || 'Select experience level'}
							</Select.Trigger>
							<Select.Content>
								{#each experienceLevels as el}
									<Select.Item value={el.value} label={el.label}>{el.label}</Select.Item>
								{/each}
							</Select.Content>
						</Select.Root>
					</div>
				</div>

				<div class="space-y-2">
					<Label for="skills">Required Skills</Label>
					<Input
						id="skills"
						placeholder="e.g., React, TypeScript, Node.js (comma-separated)"
						bind:value={skillsRequired}
					/>
					<p class="text-xs text-muted-foreground">Enter skills separated by commas</p>
				</div>
			</div>

			<!-- Company Section -->
			<div class="space-y-4">
				<h3 class="text-sm font-medium text-muted-foreground">Company Information</h3>

				<div class="grid gap-4 sm:grid-cols-2">
					<div class="space-y-2">
						<Label for="companyName">Company Name *</Label>
						<Input id="companyName" placeholder="e.g., Acme Inc." bind:value={companyName} />
					</div>

					<div class="space-y-2">
						<Label for="companyLogo">Company Logo URL</Label>
						<Input id="companyLogo" placeholder="https://..." bind:value={companyLogo} />
					</div>
				</div>
			</div>

			<!-- Compensation Section -->
			<div class="space-y-4">
				<h3 class="text-sm font-medium text-muted-foreground">Compensation (Optional)</h3>

				<div class="grid gap-4 sm:grid-cols-3">
					<div class="space-y-2">
						<Label for="salaryMin">Min Salary</Label>
						<Input
							id="salaryMin"
							type="number"
							placeholder="50000"
							bind:value={salaryMin}
						/>
					</div>

					<div class="space-y-2">
						<Label for="salaryMax">Max Salary</Label>
						<Input
							id="salaryMax"
							type="number"
							placeholder="80000"
							bind:value={salaryMax}
						/>
					</div>

					<div class="space-y-2">
						<Label>Currency</Label>
						<Select.Root type="single" bind:value={salaryCurrency}>
							<Select.Trigger class="w-full">
								{getLabel(currencies, salaryCurrency) || 'USD'}
							</Select.Trigger>
							<Select.Content>
								{#each currencies as curr}
									<Select.Item value={curr.value} label={curr.label}>{curr.label}</Select.Item>
								{/each}
							</Select.Content>
						</Select.Root>
					</div>
				</div>
			</div>

			<!-- Expiration Section -->
			<div class="space-y-4">
				<h3 class="text-sm font-medium text-muted-foreground">Listing Options</h3>

				<div class="space-y-2">
					<Label for="expiresAt">Expires On</Label>
					<Input id="expiresAt" type="date" bind:value={expiresAt} />
					<p class="text-xs text-muted-foreground">Leave empty for no expiration</p>
				</div>
			</div>

			<Dialog.Footer>
				<Button type="button" variant="outline" onclick={() => (open = false)} disabled={isSubmitting}>
					Cancel
				</Button>
				<Button type="submit" disabled={isSubmitting}>
					{#if isSubmitting}
						<Loader2 class="mr-2 h-4 w-4 animate-spin" />
						Posting...
					{:else}
						Post Job
					{/if}
				</Button>
			</Dialog.Footer>
		</form>
	</Dialog.Content>
</Dialog.Root>
