<script lang="ts">
	import Media from '$lib/Media.svelte';
	import { SkipForward, Trash2 } from '@lucide/svelte';
	import { fly } from 'svelte/transition';

	type MediaPropValue = string | number | Date;

	type DuplicateMedia = {
		id: string;
		name: string;
		thumbnail: string;
		size: MediaPropValue;
		creationDate: MediaPropValue;
	};

	let {
		left,
		right,
		onDeleteLeft,
		onSkip,
		onDeleteRight
	}: {
		left: DuplicateMedia;
		right: DuplicateMedia;
		onDeleteLeft: () => void | Promise<void>;
		onSkip: () => void | Promise<void>;
		onDeleteRight: () => void | Promise<void>;
	} = $props();

	type Resolution = 'delete-left' | 'skip' | 'delete-right';

	const transitionDuration = 220;

	let visible = $state(true);
	let resolving = $state(false);
	let resolution = $state<Resolution | null>(null);

	function exitY(side: 'left' | 'right') {
		if (resolution === 'delete-left') {
			return side === 'left' ? 96 : -96;
		}

		if (resolution === 'delete-right') {
			return side === 'right' ? 96 : -96;
		}

		return -72;
	}

	async function resolvePair(nextResolution: Resolution, callback: () => void | Promise<void>) {
		if (resolving) {
			return;
		}

		resolution = nextResolution;
		resolving = true;
		visible = false;

		await new Promise((resolve) => setTimeout(resolve, transitionDuration));
		await callback();
	}
</script>

<section class="mx-auto grid w-full max-w-3xl gap-4">
	<div class="relative grid grid-cols-2 gap-5">
		<div class="absolute top-3 bottom-3 left-1/2 w-px -translate-x-1/2 bg-white/10" aria-hidden="true"></div>

		{#if visible}
			<div out:fly={{ y: exitY('left'), duration: transitionDuration }}>
				<Media
					id={left.id}
					name={left.name}
					thumbnail={left.thumbnail}
					size={left.size}
					creationDate={left.creationDate}
					joinedSide="left"
					showActions={false}
				/>
			</div>

			<div out:fly={{ y: exitY('right'), duration: transitionDuration }}>
				<Media
					id={right.id}
					name={right.name}
					thumbnail={right.thumbnail}
					size={right.size}
					creationDate={right.creationDate}
					joinedSide="right"
					showActions={false}
				/>
			</div>
		{/if}
	</div>

	<div class="grid grid-cols-3 gap-3">
		<button
			type="button"
			disabled={resolving}
			onclick={() => resolvePair('delete-left', onDeleteLeft)}
			class="flex h-10 items-center justify-center gap-2 rounded-xl border border-red-400/20 bg-red-500/10 px-3 text-sm font-medium text-red-200 transition hover:bg-red-500/15 focus:outline-none focus:ring-2 focus:ring-red-300/30"
		>
			<Trash2 class="h-4 w-4" aria-hidden="true" />
			Delete left
		</button>
		<button
			type="button"
			disabled={resolving}
			onclick={() => resolvePair('skip', onSkip)}
			class="flex h-10 items-center justify-center gap-2 rounded-xl border border-white/10 bg-white/4 px-3 text-sm font-medium text-slate-200 transition hover:bg-white/8 focus:outline-none focus:ring-2 focus:ring-white/10"
		>
			<SkipForward class="h-4 w-4" aria-hidden="true" />
			Skip
		</button>
		<button
			type="button"
			disabled={resolving}
			onclick={() => resolvePair('delete-right', onDeleteRight)}
			class="flex h-10 items-center justify-center gap-2 rounded-xl border border-red-400/20 bg-red-500/10 px-3 text-sm font-medium text-red-200 transition hover:bg-red-500/15 focus:outline-none focus:ring-2 focus:ring-red-300/30"
		>
			<Trash2 class="h-4 w-4" aria-hidden="true" />
			Delete right
		</button>
	</div>
</section>
