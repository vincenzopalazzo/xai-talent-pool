<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Badge } from '$lib/components/ui/badge';
	import { Separator } from '$lib/components/ui/separator';
	import { Loader2, CheckCircle2, User, Mail, Briefcase, MapPin, FileText, Upload, X } from 'lucide-svelte';
	import type { Job, Talent } from '$lib/types';

	let {
		job,
		open = $bindable(false)
	}: {
		job: Job;
		open: boolean;
	} = $props();

	type Step = 'email' | 'existing' | 'new' | 'success';
	let step = $state<Step>('email');
	let isLoading = $state(false);
	let error = $state<string | null>(null);

	// Email step
	let email = $state('');

	// Existing talent data (skills comes as comma-separated string from API)
	let existingTalent = $state<Talent | null>(null);

	// Parse skills for display
	const existingTalentSkills = $derived(() => {
		if (!existingTalent) return [];
		// Handle both array and comma-separated string formats
		if (Array.isArray(existingTalent.skills)) {
			return existingTalent.skills;
		}
		return (existingTalent.skills as unknown as string)
			?.split(',')
			.map((s) => s.trim())
			.filter((s) => s) ?? [];
	});

	// New talent form
	let name = $state('');
	let handle = $state('');
	let title = $state('');
	let location = $state('');
	let experience = $state('');
	let skills = $state('');
	let bio = $state('');

	// Resume upload
	let resumeFile = $state<File | null>(null);
	let resumeData = $state<string | null>(null);
	let coverLetter = $state('');

	// Convert file to base64
	async function handleResumeSelect(event: Event) {
		const input = event.target as HTMLInputElement;
		const file = input.files?.[0];
		if (file) {
			// Validate file type
			const allowedTypes = ['application/pdf', 'application/msword', 'application/vnd.openxmlformats-officedocument.wordprocessingml.document'];
			if (!allowedTypes.includes(file.type)) {
				error = 'Please upload a PDF or Word document';
				return;
			}
			// Validate file size (5MB max)
			if (file.size > 5 * 1024 * 1024) {
				error = 'File size must be less than 5MB';
				return;
			}
			resumeFile = file;
			// Convert to base64
			const reader = new FileReader();
			reader.onload = () => {
				const base64 = (reader.result as string).split(',')[1];
				resumeData = base64;
			};
			reader.readAsDataURL(file);
			error = null;
		}
	}

	function removeResume() {
		resumeFile = null;
		resumeData = null;
	}

	function resetForm() {
		step = 'email';
		email = '';
		existingTalent = null;
		name = '';
		handle = '';
		title = '';
		location = '';
		experience = '';
		skills = '';
		bio = '';
		resumeFile = null;
		resumeData = null;
		coverLetter = '';
		error = null;
		isLoading = false;
	}

	async function checkEmail() {
		if (!email.trim()) {
			error = 'Email is required';
			return;
		}

		if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)) {
			error = 'Please enter a valid email address';
			return;
		}

		isLoading = true;
		error = null;

		try {
			const response = await fetch(
				`http://localhost:8080/api/v1/talents/email/${encodeURIComponent(email.trim())}`
			);

			if (response.ok) {
				existingTalent = await response.json();
				step = 'existing';
			} else if (response.status === 404) {
				step = 'new';
			} else {
				throw new Error('Failed to check email');
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'An error occurred';
		} finally {
			isLoading = false;
		}
	}

	async function applyWithExisting() {
		if (!existingTalent) return;

		isLoading = true;
		error = null;

		try {
			const payload = {
				talent_id: existingTalent.id,
				job_id: job.id,
				resume_data: resumeData,
				resume_filename: resumeFile?.name || null,
				resume_content_type: resumeFile?.type || null,
				cover_letter: coverLetter.trim() || null
			};

			const response = await fetch('http://localhost:8080/api/v1/applications', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(payload)
			});

			if (!response.ok) {
				const errorData = await response.text();
				throw new Error(errorData || 'Failed to submit application');
			}

			step = 'success';
		} catch (err) {
			error = err instanceof Error ? err.message : 'An error occurred';
		} finally {
			isLoading = false;
		}
	}

	async function createAndApply() {
		if (!name.trim()) {
			error = 'Name is required';
			return;
		}
		if (!handle.trim()) {
			error = 'Handle is required';
			return;
		}
		if (!title.trim()) {
			error = 'Title is required';
			return;
		}
		if (!experience.trim()) {
			error = 'Experience level is required';
			return;
		}

		isLoading = true;
		error = null;

		try {
			// First, create the talent profile
			const talentPayload = {
				name: name.trim(),
				email: email.trim(),
				handle: handle.trim(),
				title: title.trim(),
				location: location.trim() || null,
				experience: experience.trim(),
				skills: skills.trim(),
				bio: bio.trim() || null,
				verified: false
			};

			const talentResponse = await fetch('http://localhost:8080/api/v1/talents', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(talentPayload)
			});

			if (!talentResponse.ok) {
				const errorData = await talentResponse.text();
				throw new Error(errorData || 'Failed to create profile');
			}

			const newTalent = await talentResponse.json();

			// Then, submit the application
			const applicationPayload = {
				talent_id: newTalent.id,
				job_id: job.id,
				resume_data: resumeData,
				resume_filename: resumeFile?.name || null,
				resume_content_type: resumeFile?.type || null,
				cover_letter: coverLetter.trim() || null
			};

			const appResponse = await fetch('http://localhost:8080/api/v1/applications', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(applicationPayload)
			});

			if (!appResponse.ok) {
				const errorData = await appResponse.text();
				throw new Error(errorData || 'Failed to submit application');
			}

			step = 'success';
		} catch (err) {
			error = err instanceof Error ? err.message : 'An error occurred';
		} finally {
			isLoading = false;
		}
	}
</script>

<Dialog.Root bind:open onOpenChange={(isOpen) => !isOpen && resetForm()}>
	<Dialog.Content class="max-w-lg">
		<Dialog.Header>
			<Dialog.Title>
				{#if step === 'success'}
					Application Submitted
				{:else}
					Apply for {job.title}
				{/if}
			</Dialog.Title>
			<Dialog.Description>
				{#if step === 'email'}
					Enter your email to apply for this position at {job.company_name}.
				{:else if step === 'existing'}
					We found your profile. Review your information and confirm your application.
				{:else if step === 'new'}
					Create your talent profile to apply for this position.
				{:else if step === 'success'}
					Your application has been submitted successfully.
				{/if}
			</Dialog.Description>
		</Dialog.Header>

		{#if error}
			<div class="rounded-md bg-destructive/10 p-3 text-sm text-destructive">
				{error}
			</div>
		{/if}

		{#if step === 'email'}
			<div class="space-y-4 py-4">
				<div class="space-y-2">
					<Label for="email">Email Address</Label>
					<Input
						id="email"
						type="email"
						placeholder="you@example.com"
						bind:value={email}
						onkeydown={(e) => e.key === 'Enter' && checkEmail()}
					/>
				</div>
			</div>

			<Dialog.Footer>
				<Button variant="outline" onclick={() => (open = false)}>Cancel</Button>
				<Button onclick={checkEmail} disabled={isLoading}>
					{#if isLoading}
						<Loader2 class="mr-2 h-4 w-4 animate-spin" />
						Checking...
					{:else}
						Continue
					{/if}
				</Button>
			</Dialog.Footer>
		{:else if step === 'existing' && existingTalent}
			<div class="space-y-4 py-4">
				<div class="rounded-lg bg-muted/50 p-4">
					<div class="flex items-start gap-4">
						<div class="flex h-12 w-12 items-center justify-center rounded-full bg-primary/10">
							<User class="h-6 w-6 text-primary" />
						</div>
						<div class="flex-1 space-y-1">
							<p class="font-medium">{existingTalent.name}</p>
							<p class="text-sm text-muted-foreground">{existingTalent.title}</p>
							<div class="flex flex-wrap items-center gap-2 pt-1 text-xs text-muted-foreground">
								<span class="flex items-center gap-1">
									<Mail class="h-3 w-3" />
									{existingTalent.email}
								</span>
								{#if existingTalent.location}
									<span class="flex items-center gap-1">
										<MapPin class="h-3 w-3" />
										{existingTalent.location}
									</span>
								{/if}
								<span class="flex items-center gap-1">
									<Briefcase class="h-3 w-3" />
									{existingTalent.experience}
								</span>
							</div>
						</div>
					</div>
					{#if existingTalentSkills().length > 0}
						<div class="mt-3 flex flex-wrap gap-1">
							{#each existingTalentSkills().slice(0, 5) as skill}
								<Badge variant="outline" class="text-xs">{skill}</Badge>
							{/each}
							{#if existingTalentSkills().length > 5}
								<Badge variant="outline" class="text-xs">+{existingTalentSkills().length - 5}</Badge>
							{/if}
						</div>
					{/if}
				</div>

				<p class="text-sm text-muted-foreground">
					Not you?{' '}
					<button
						class="text-primary hover:underline"
						onclick={() => {
							email = '';
							step = 'email';
						}}
					>
						Use a different email
					</button>
				</p>

				<Separator class="my-4" />

				<!-- Resume Upload -->
				<div class="space-y-2">
					<Label for="resume-existing">Resume (Optional)</Label>
					<input
						id="resume-existing"
						type="file"
						accept=".pdf,.doc,.docx,application/pdf,application/msword,application/vnd.openxmlformats-officedocument.wordprocessingml.document"
						class="hidden"
						onchange={handleResumeSelect}
					/>
					{#if resumeFile}
						<div class="flex items-center gap-2 rounded-md border p-2">
							<FileText class="h-4 w-4 text-muted-foreground" />
							<span class="flex-1 truncate text-sm">{resumeFile.name}</span>
							<button
								type="button"
								class="rounded-full p-1 hover:bg-muted"
								onclick={removeResume}
							>
								<X class="h-4 w-4" />
							</button>
						</div>
					{:else}
						<Button
							variant="outline"
							class="w-full"
							onclick={() => document.getElementById('resume-existing')?.click()}
						>
							<Upload class="mr-2 h-4 w-4" />
							Upload Resume
						</Button>
					{/if}
					<p class="text-xs text-muted-foreground">PDF or Word document, max 5MB</p>
				</div>

				<!-- Cover Letter -->
				<div class="space-y-2">
					<Label for="cover-letter-existing">Cover Letter (Optional)</Label>
					<Textarea
						id="cover-letter-existing"
						placeholder="Tell us why you're interested in this position..."
						rows={3}
						bind:value={coverLetter}
					/>
				</div>
			</div>

			<Dialog.Footer>
				<Button variant="outline" onclick={() => (open = false)}>Cancel</Button>
				<Button onclick={applyWithExisting} disabled={isLoading}>
					{#if isLoading}
						<Loader2 class="mr-2 h-4 w-4 animate-spin" />
						Submitting...
					{:else}
						Confirm Application
					{/if}
				</Button>
			</Dialog.Footer>
		{:else if step === 'new'}
			<div class="max-h-[60vh] space-y-4 overflow-y-auto py-4">
				<div class="grid gap-4 sm:grid-cols-2">
					<div class="space-y-2">
						<Label for="name">Full Name *</Label>
						<Input id="name" placeholder="John Doe" bind:value={name} />
					</div>
					<div class="space-y-2">
						<Label for="handle">Handle *</Label>
						<Input id="handle" placeholder="@johndoe" bind:value={handle} />
					</div>
				</div>

				<div class="space-y-2">
					<Label for="title">Professional Title *</Label>
					<Input id="title" placeholder="Senior Software Engineer" bind:value={title} />
				</div>

				<div class="grid gap-4 sm:grid-cols-2">
					<div class="space-y-2">
						<Label for="location">Location</Label>
						<Input id="location" placeholder="San Francisco, CA" bind:value={location} />
					</div>
					<div class="space-y-2">
						<Label for="experience">Experience Level *</Label>
						<Input id="experience" placeholder="5+ years" bind:value={experience} />
					</div>
				</div>

				<div class="space-y-2">
					<Label for="skills">Skills</Label>
					<Input
						id="skills"
						placeholder="React, TypeScript, Node.js (comma-separated)"
						bind:value={skills}
					/>
					<p class="text-xs text-muted-foreground">Enter skills separated by commas</p>
				</div>

				<div class="space-y-2">
					<Label for="bio">Bio</Label>
					<Textarea id="bio" placeholder="Tell us about yourself..." rows={3} bind:value={bio} />
				</div>

				<Separator />

				<!-- Resume Upload -->
				<div class="space-y-2">
					<Label for="resume-new">Resume (Optional)</Label>
					<input
						id="resume-new"
						type="file"
						accept=".pdf,.doc,.docx,application/pdf,application/msword,application/vnd.openxmlformats-officedocument.wordprocessingml.document"
						class="hidden"
						onchange={handleResumeSelect}
					/>
					{#if resumeFile}
						<div class="flex items-center gap-2 rounded-md border p-2">
							<FileText class="h-4 w-4 text-muted-foreground" />
							<span class="flex-1 truncate text-sm">{resumeFile.name}</span>
							<button
								type="button"
								class="rounded-full p-1 hover:bg-muted"
								onclick={removeResume}
							>
								<X class="h-4 w-4" />
							</button>
						</div>
					{:else}
						<Button
							variant="outline"
							class="w-full"
							onclick={() => document.getElementById('resume-new')?.click()}
						>
							<Upload class="mr-2 h-4 w-4" />
							Upload Resume
						</Button>
					{/if}
					<p class="text-xs text-muted-foreground">PDF or Word document, max 5MB</p>
				</div>

				<!-- Cover Letter -->
				<div class="space-y-2">
					<Label for="cover-letter-new">Cover Letter (Optional)</Label>
					<Textarea
						id="cover-letter-new"
						placeholder="Tell us why you're interested in this position..."
						rows={3}
						bind:value={coverLetter}
					/>
				</div>
			</div>

			<Dialog.Footer>
				<Button variant="outline" onclick={() => (step = 'email')}>Back</Button>
				<Button onclick={createAndApply} disabled={isLoading}>
					{#if isLoading}
						<Loader2 class="mr-2 h-4 w-4 animate-spin" />
						Creating Profile...
					{:else}
						Create Profile & Apply
					{/if}
				</Button>
			</Dialog.Footer>
		{:else if step === 'success'}
			<div class="flex flex-col items-center py-8 text-center">
				<div class="mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-green-100 dark:bg-green-900">
					<CheckCircle2 class="h-8 w-8 text-green-600 dark:text-green-400" />
				</div>
				<h3 class="mb-2 text-lg font-medium">Application Submitted!</h3>
				<p class="mb-4 text-sm text-muted-foreground">
					Your application for <span class="font-medium">{job.title}</span> at{' '}
					<span class="font-medium">{job.company_name}</span> has been submitted.
				</p>
			</div>

			<Dialog.Footer>
				<Button class="w-full" onclick={() => (open = false)}>Done</Button>
			</Dialog.Footer>
		{/if}
	</Dialog.Content>
</Dialog.Root>
