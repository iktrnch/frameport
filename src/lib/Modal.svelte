<script lang="ts">
	import { fly } from 'svelte/transition';
	import type { Snippet } from 'svelte';
	import { X } from '@lucide/svelte';

	let {
		open = $bindable(false),
		showCloseButton = true,
		children,
		onClose
	}: {
		open?: boolean;
		showCloseButton?: boolean;
		children?: Snippet;
		onClose?: () => void;
	} = $props();

	function close() {
		open = false;
		onClose?.();
	}
</script>

{#if open}
	<section
		role="dialog"
		aria-modal="true"
		class="absolute inset-0 z-20 flex h-full w-full overflow-visible rounded-3xl border border-white/10 bg-neutral-950 text-slate-100 shadow-2xl shadow-black/40"
		transition:fly={{ x: 64, duration: 240 }}
	>
		{#if showCloseButton}
			<button
				type="button"
				aria-label="Close modal"
				onclick={close}
				class="absolute top-1 right-1 z-10 flex h-5 w-5 translate-x-1/2 -translate-y-1/2 items-center justify-center rounded-full border border-white/10 bg-neutral-950 text-slate-300 shadow-md shadow-black/30 transition hover:bg-neutral-900 hover:text-slate-50 focus:outline-none focus:ring-2 focus:ring-neutral-500"
			>
				<X class="h-4 w-4" aria-hidden="true" />
			</button>
		{/if}

		<div class="min-h-0 flex-1 overflow-y-auto p-8">
			{#if children}
				{@render children()}
			{/if}
		</div>
	</section>
{/if}
