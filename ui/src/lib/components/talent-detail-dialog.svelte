<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import * as Avatar from '$lib/components/ui/avatar';
	import { Separator } from '$lib/components/ui/separator';
	import {
		MapPin,
		Briefcase,
		Mail,
		Calendar,
		ExternalLink,
		CheckCircle2
	} from 'lucide-svelte';
	import type { Talent } from '$lib/types';

	let {
		talent,
		open = $bindable(false)
	}: {
		talent: Talent;
		open: boolean;
	} = $props();

	// Parse skills from comma-separated string or array
	const skills = $derived(() => {
		if (!talent.skills) return [];
		if (Array.isArray(talent.skills)) return talent.skills;
		return (talent.skills as unknown as string).split(',').map((s) => s.trim()).filter((s) => s);
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
	<Dialog.Content class="max-h-[90vh] max-w-2xl overflow-y-auto">
		<Dialog.Header>
			<div class="flex items-start gap-4">
				<Avatar.Root class="h-20 w-20">
					{#if talent.avatar}
						<Avatar.Image src={talent.avatar} alt={talent.name} />
					{/if}
					<Avatar.Fallback class="text-xl">{talent.name.slice(0, 2).toUpperCase()}</Avatar.Fallback>
				</Avatar.Root>
				<div class="flex-1">
					<div class="flex items-center gap-2">
						<Dialog.Title class="text-xl">{talent.name}</Dialog.Title>
						{#if talent.verified}
							<Badge variant="secondary" class="gap-1">
								<CheckCircle2 class="h-3 w-3" />
								Verified
							</Badge>
						{/if}
					</div>
					<Dialog.Description class="text-base text-muted-foreground">
						@{talent.handle}
					</Dialog.Description>
					<p class="mt-1 text-sm font-medium">{talent.title}</p>
				</div>
			</div>
		</Dialog.Header>

		<div class="space-y-6 py-4">
			<!-- Key details -->
			<div class="grid gap-3 rounded-lg bg-muted/50 p-4 sm:grid-cols-2">
				{#if talent.email}
					<div class="flex items-center gap-2 text-sm">
						<Mail class="h-4 w-4 text-muted-foreground" />
						<span>{talent.email}</span>
					</div>
				{/if}
				{#if talent.location}
					<div class="flex items-center gap-2 text-sm">
						<MapPin class="h-4 w-4 text-muted-foreground" />
						<span>{talent.location}</span>
					</div>
				{/if}
				<div class="flex items-center gap-2 text-sm">
					<Briefcase class="h-4 w-4 text-muted-foreground" />
					<span>{talent.experience}</span>
				</div>
				{#if talent.created_at}
					<div class="flex items-center gap-2 text-sm text-muted-foreground">
						<Calendar class="h-4 w-4" />
						<span>Joined {formatDate(talent.created_at)}</span>
					</div>
				{/if}
			</div>

			<!-- Bio -->
			{#if talent.bio}
				<div class="space-y-2">
					<h3 class="text-sm font-semibold">About</h3>
					<p class="whitespace-pre-wrap text-sm text-muted-foreground">{talent.bio}</p>
				</div>

				<Separator />
			{/if}

			<!-- Skills -->
			{#if skills().length > 0}
				<div class="space-y-2">
					<h3 class="text-sm font-semibold">Skills</h3>
					<div class="flex flex-wrap gap-2">
						{#each skills() as skill}
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
			<Button class="w-full sm:flex-1">
				<ExternalLink class="mr-2 h-4 w-4" />
				Contact
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
