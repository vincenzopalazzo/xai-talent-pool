<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { Loader2, Download, X } from 'lucide-svelte';

	let {
		applicationId,
		filename = 'Resume',
		open = $bindable(false)
	}: {
		applicationId: string;
		filename?: string;
		open: boolean;
	} = $props();

	let isLoading = $state(true);
	let error = $state<string | null>(null);

	const pdfUrl = $derived(`http://localhost:8080/api/v1/applications/${applicationId}/resume`);

	function handleLoad() {
		isLoading = false;
	}

	function handleError() {
		isLoading = false;
		error = 'Failed to load PDF preview';
	}

	function downloadPdf() {
		const link = document.createElement('a');
		link.href = pdfUrl;
		link.download = filename;
		link.click();
	}

	// Reset state when dialog opens
	$effect(() => {
		if (open) {
			isLoading = true;
			error = null;
		}
	});
</script>

<Dialog.Root bind:open>
	<Dialog.Content class="max-h-[95vh] max-w-5xl p-0">
		<Dialog.Header class="flex flex-row items-center justify-between border-b px-4 py-3">
			<div>
				<Dialog.Title class="text-lg">{filename}</Dialog.Title>
				<Dialog.Description class="text-sm">Resume Preview</Dialog.Description>
			</div>
			<div class="flex items-center gap-2">
				<Button variant="outline" size="sm" onclick={downloadPdf}>
					<Download class="mr-2 h-4 w-4" />
					Download
				</Button>
				<Button variant="ghost" size="icon" class="h-8 w-8" onclick={() => (open = false)}>
					<X class="h-4 w-4" />
				</Button>
			</div>
		</Dialog.Header>

		<div class="relative h-[80vh] w-full bg-muted">
			{#if isLoading}
				<div class="absolute inset-0 flex items-center justify-center">
					<Loader2 class="h-8 w-8 animate-spin text-muted-foreground" />
				</div>
			{/if}

			{#if error}
				<div class="absolute inset-0 flex items-center justify-center">
					<div class="text-center">
						<p class="text-destructive">{error}</p>
						<Button variant="outline" class="mt-4" onclick={downloadPdf}>
							<Download class="mr-2 h-4 w-4" />
							Download Instead
						</Button>
					</div>
				</div>
			{:else}
				<iframe
					src={pdfUrl}
					title={filename}
					class="h-full w-full border-0"
					onload={handleLoad}
					onerror={handleError}
				></iframe>
			{/if}
		</div>
	</Dialog.Content>
</Dialog.Root>
