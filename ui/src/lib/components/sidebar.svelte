<script lang="ts">
	import { page } from '$app/stores';
	import { Home, Users, Settings, Menu, Briefcase, Star, ClipboardList } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import { Separator } from '$lib/components/ui/separator';

	let { collapsed = $bindable(false) } = $props();

	const navItems = [
		{ icon: Home, label: 'Dashboard', href: '/dashboard' },
		{ icon: Users, label: 'Talent Pool', href: '/' },
		{ icon: Briefcase, label: 'Jobs', href: '/jobs' },
		{ icon: ClipboardList, label: 'Requirements', href: '/requirements' },
		{ icon: Star, label: 'Saved', href: '/saved' },
		{ icon: Settings, label: 'Settings', href: '/settings' }
	];

	// Determine active state based on current path
	const isActive = (href: string) => {
		const currentPath = $page.url.pathname;
		if (href === '/') {
			return currentPath === '/';
		}
		return currentPath.startsWith(href);
	};
</script>

<aside
	class="flex h-screen flex-col border-r border-border bg-sidebar transition-all duration-300 {collapsed
		? 'w-16'
		: 'w-64'}"
>
	<div class="flex h-14 items-center justify-between px-4">
		{#if !collapsed}
			<div class="flex items-center gap-2">
				<div class="flex h-8 w-8 items-center justify-center rounded-md bg-foreground text-background">
					<span class="text-lg font-bold">X</span>
				</div>
				<span class="text-lg font-semibold">Talent Pool</span>
			</div>
		{:else}
			<div class="flex h-8 w-8 items-center justify-center rounded-md bg-foreground text-background mx-auto">
				<span class="text-lg font-bold">X</span>
			</div>
		{/if}
		<Button variant="ghost" size="icon" class="h-8 w-8 {collapsed ? 'hidden' : ''}" onclick={() => (collapsed = !collapsed)}>
			<Menu class="h-4 w-4" />
		</Button>
	</div>

	<Separator />

	<nav class="flex-1 space-y-1 p-2">
		{#each navItems as item}
			<a
				href={item.href}
				class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors hover:bg-sidebar-accent {isActive(item.href)
					? 'bg-sidebar-accent text-sidebar-accent-foreground'
					: 'text-sidebar-foreground'} {collapsed ? 'justify-center' : ''}"
			>
				<item.icon class="h-5 w-5 shrink-0" />
				{#if !collapsed}
					<span>{item.label}</span>
				{/if}
			</a>
		{/each}
	</nav>

	<Separator />

	<div class="p-4">
		{#if collapsed}
			<Button variant="ghost" size="icon" class="w-full" onclick={() => (collapsed = !collapsed)}>
				<Menu class="h-4 w-4" />
			</Button>
		{:else}
			<div class="flex items-center gap-3">
				<div class="h-8 w-8 rounded-full bg-muted"></div>
				<div class="flex-1 overflow-hidden">
					<p class="truncate text-sm font-medium">Your Account</p>
					<p class="truncate text-xs text-muted-foreground">@username</p>
				</div>
			</div>
		{/if}
	</div>
</aside>
