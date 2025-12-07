<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { Badge } from '$lib/components/ui/badge';
	import { Separator } from '$lib/components/ui/separator';
	import { Button } from '$lib/components/ui/button';
	import * as Tabs from '$lib/components/ui/tabs';
	import {
		Github,
		Linkedin,
		Twitter,
		Gitlab,
		ExternalLink,
		CheckCircle,
		XCircle,
		AlertCircle,
		TrendingUp,
		Star,
		Eye,
		Loader2
	} from 'lucide-svelte';
	import type { SocialMediaAnalysis, PlatformProfile } from '$lib/types';

	let {
		analysis,
		isLoading = false
	}: {
		analysis: SocialMediaAnalysis | string | null | undefined;
		isLoading?: boolean;
	} = $props();

	// Helper to get platform icon
	function getPlatformIcon(platform: string) {
		switch (platform.toLowerCase()) {
			case 'github':
				return Github;
			case 'linkedin':
				return Linkedin;
			case 'x':
			case 'twitter':
				return Twitter;
			case 'gitlab':
				return Gitlab;
			case 'stackoverflow':
				// StackOverflow icon not available, use ExternalLink
				return ExternalLink;
			default:
				return ExternalLink;
		}
	}

	// Helper to get platform color
	function getPlatformColor(platform: string) {
		switch (platform.toLowerCase()) {
			case 'github':
				return 'text-gray-600 dark:text-gray-400';
			case 'linkedin':
				return 'text-blue-600 dark:text-blue-400';
			case 'x':
			case 'twitter':
				return 'text-sky-500 dark:text-sky-400';
			case 'gitlab':
				return 'text-orange-600 dark:text-orange-400';
			case 'stackoverflow':
				return 'text-orange-500 dark:text-orange-400';
			default:
				return 'text-muted-foreground';
		}
	}

	// Parse social analysis JSON if needed
	const parsedAnalysis = $derived.by((): SocialMediaAnalysis | null => {
		if (!analysis) return null;
		// If it's a string, parse it
		if (typeof analysis === 'string') {
			try {
				return JSON.parse(analysis);
			} catch {
				return null;
			}
		}
		return analysis;
	});
</script>

{#if isLoading}
	<div class="flex items-center justify-center py-8">
		<Loader2 class="h-6 w-6 animate-spin text-muted-foreground" />
		<span class="ml-2 text-sm text-muted-foreground">Loading social media analysis...</span>
	</div>
{:else if parsedAnalysis}
	<div class="space-y-4">
		<!-- Overall TLDR -->
		{#if parsedAnalysis.tldr}
			<Card.Root class="border-2">
				<Card.Header class="pb-3">
					<div class="flex items-center gap-2">
						<TrendingUp class="h-5 w-5 text-primary" />
						<Card.Title class="text-lg">AI Social Media Summary</Card.Title>
					</div>
				</Card.Header>
				<Card.Content>
					<p class="text-sm leading-relaxed">{parsedAnalysis.tldr}</p>
				</Card.Content>
			</Card.Root>
		{/if}

		<!-- Discovered X Handle -->
		{#if parsedAnalysis.x_handle}
			<div class="flex items-center gap-2 rounded-lg bg-sky-50 dark:bg-sky-950/20 p-3">
				<Twitter class="h-4 w-4 text-sky-600 dark:text-sky-400" />
				<span class="text-sm font-medium">Discovered X Handle: @{parsedAnalysis.x_handle}</span>
			</div>
		{/if}

		<!-- Platform Analyses -->
		<Tabs.Root class="w-full" value={parsedAnalysis.profiles[0]?.platform}>
			<Tabs.List class="grid w-full grid-cols-2 lg:grid-cols-3">
				{#each parsedAnalysis.profiles as profile (profile.platform)}
					<Tabs.Trigger value={profile.platform} class="gap-2">
						{@const Icon = getPlatformIcon(profile.platform)}
						<Icon class="h-4 w-4" />
						{profile.platform}
						{#if profile.verified}
							<CheckCircle class="h-3 w-3 text-green-500" />
						{:else}
							<AlertCircle class="h-3 w-3 text-yellow-500" />
						{/if}
					</Tabs.Trigger>
				{/each}
			</Tabs.List>

			{#each parsedAnalysis.profiles as profile (profile.platform)}
				<Tabs.Content value={profile.platform} class="mt-4">
					<Card.Root>
						<Card.Header>
							<div class="flex items-start justify-between gap-3">
								<div class="flex items-center gap-2">
									<svelte:component
										this={getPlatformIcon(profile.platform)}
										class="h-5 w-5 {getPlatformColor(profile.platform)}"
									/>
									<div>
										<Card.Title class="flex items-center gap-2">
											{profile.platform}
											{#if profile.verified}
												<Badge variant="secondary" class="gap-1">
													<CheckCircle class="h-3 w-3" />
													Verified
												</Badge>
											{/if}
										</Card.Title>
										{#if profile.handle}
											<Card.Description class="flex items-center gap-1 mt-1">
												@{profile.handle}
												{#if profile.url}
													<Button variant="ghost" size="sm" class="h-auto p-0" onclick={() => window.open(profile.url, '_blank')}>
														<ExternalLink class="h-3 w-3" />
													</Button>
												{/if}
											</Card.Description>
										{/if}
									</div>
								</div>
							</div>
						</Card.Header>

						<Card.Content class="space-y-4">
							<!-- Platform TLDR -->
							{#if profile.tldr}
								<div class="rounded-lg bg-muted/50 p-3">
									<h4 class="text-sm font-semibold mb-2 flex items-center gap-1">
										<Star class="h-4 w-4" />
										TLDR
									</h4>
									<p class="text-sm leading-relaxed">{profile.tldr}</p>
								</div>
							{/if}

							<!-- Bio -->
							{#if profile.bio}
								<div>
									<h4 class="text-sm font-semibold mb-1">Bio</h4>
									<p class="text-sm text-muted-foreground">{profile.bio}</p>
								</div>
							{/if}

							<Separator />

							<!-- Highlights -->
							{#if profile.highlights.length > 0}
								<div>
									<h4 class="text-sm font-semibold mb-2 flex items-center gap-1">
										<Eye class="h-4 w-4" />
										Key Highlights
									</h4>
									<ul class="space-y-1">
										{#each profile.highlights as highlight}
											<li class="text-sm flex items-start gap-2">
												<span class="text-primary mt-0.5">•</span>
												<span>{highlight}</span>
											</li>
										{/each}
									</ul>
								</div>
							{/if}

							<!-- Skills -->
							{#if profile.skills.length > 0}
								<div>
									<h4 class="text-sm font-semibold mb-2">Skills</h4>
									<div class="flex flex-wrap gap-1">
										{#each profile.skills as skill}
											<Badge variant="outline" class="text-xs">{skill}</Badge>
										{/each}
									</div>
								</div>
							{/if}

							<!-- Experience Signals -->
							{#if profile.experience_signals.length > 0}
								<div>
									<h4 class="text-sm font-semibold mb-2">Seniority Indicators</h4>
									<ul class="space-y-1">
										{#each profile.experience_signals as signal}
											<li class="text-sm flex items-start gap-2">
												<TrendingUp class="h-3 w-3 text-green-500 mt-0.5" />
												<span>{signal}</span>
											</li>
										{/each}
									</ul>
								</div>
							{/if}

							<!-- Red Flags -->
							{#if profile.red_flags.length > 0}
								<div>
									<h4 class="text-sm font-semibold mb-2 flex items-center gap-1">
										<XCircle class="h-4 w-4 text-red-500" />
										Professional Concerns
									</h4>
									<ul class="space-y-1">
										{#each profile.red_flags as flag}
											<li class="text-sm flex items-start gap-2">
												<AlertCircle class="h-3 w-3 text-yellow-500 mt-0.5" />
												<span>{flag}</span>
											</li>
										{/each}
									</ul>
								</div>
							{/if}

							<!-- Recruiter Notes -->
							{#if profile.recruiter_notes.length > 0}
								<div>
									<h4 class="text-sm font-semibold mb-2">Key Recruiter Insights</h4>
									<ul class="space-y-1">
										{#each profile.recruiter_notes as note}
											<li class="text-sm flex items-start gap-2">
												<span class="text-blue-500 mt-0.5">◆</span>
												<span>{note}</span>
											</li>
										{/each}
									</ul>
								</div>
							{/if}
						</Card.Content>
					</Card.Root>
				</Tabs.Content>
			{/each}
		</Tabs.Root>

		<!-- Combined Skills -->
		{#if parsedAnalysis.combined_skills.length > 0}
			<Card.Root>
				<Card.Header>
					<Card.Title class="text-sm">All Identified Skills ({parsedAnalysis.combined_skills.length})</Card.Title>
				</Card.Header>
				<Card.Content>
					<div class="flex flex-wrap gap-1">
						{#each parsedAnalysis.combined_skills as skill}
							<Badge variant="secondary" class="text-xs">{skill}</Badge>
						{/each}
					</div>
				</Card.Content>
			</Card.Root>
		{/if}
	</div>
{:else}
	<div class="text-center py-8">
		<p class="text-sm text-muted-foreground">No social media analysis available</p>
	</div>
{/if}