<script lang="ts">
	import Dupes from '$lib/Dupes.svelte';
	import NotFound from '$lib/NotFound.svelte';
	import { Copy } from '@lucide/svelte';
	import { fly } from 'svelte/transition';

	type DuplicateMedia = {
		id: string;
		name: string;
		thumbnail: string;
		size: string;
		creationDate: string;
	};

	type DuplicatePair = {
		id: string;
		left: DuplicateMedia;
		right: DuplicateMedia;
	};

	const duplicatePairs: DuplicatePair[] = [
		{
			id: 'coast-roll-copy',
			left: {
				id: 'summer-roll-001',
				name: 'Summer coast roll',
				thumbnail: '/svelte.svg',
				size: '2.4 GB',
				creationDate: '12 Jun 2026'
			},
			right: {
				id: 'summer-roll-001-copy',
				name: 'Summer coast roll copy',
				thumbnail: '/svelte.svg',
				size: '2.4 GB',
				creationDate: '12 Jun 2026'
			}
		},
		{
			id: 'studio-session-copy',
			left: {
				id: 'studio-session-014',
				name: 'Studio portrait session',
				thumbnail: '/vite.svg',
				size: '842 MB',
				creationDate: '08 Jun 2026'
			},
			right: {
				id: 'studio-session-014-backup',
				name: 'Studio portrait session backup',
				thumbnail: '/vite.svg',
				size: '842 MB',
				creationDate: '08 Jun 2026'
			}
		},
		{
			id: 'city-night-copy',
			left: {
				id: 'city-night-009',
				name: 'City night timelapse',
				thumbnail: '',
				size: '3.1 GB',
				creationDate: '25 May 2026'
			},
			right: {
				id: 'city-night-009-import-2',
				name: 'City night timelapse import 2',
				thumbnail: '',
				size: '3.1 GB',
				creationDate: '25 May 2026'
			}
		}
	];

	let currentPairIndex = $state(0);
	let currentPair = $derived(duplicatePairs[currentPairIndex]);

	function showNextPair() {
		currentPairIndex += 1;
	}

	async function deleteLeft() {
		// TODO: wire to backend delete command.
		showNextPair();
	}

	async function skipPair() {
		// TODO: wire to backend skip duplicate command.
		showNextPair();
	}

	async function deleteRight() {
		// TODO: wire to backend delete command.
		showNextPair();
	}
</script>

{#if currentPair}
	<div class="flex h-full w-full items-center justify-center rounded-3xl border border-white/10 bg-neutral-950/30 p-6">
		{#key currentPair.id}
			<div class="w-full" in:fly={{ x: 96, duration: 240 }}>
				<Dupes
					left={currentPair.left}
					right={currentPair.right}
					onDeleteLeft={deleteLeft}
					onSkip={skipPair}
					onDeleteRight={deleteRight}
				/>
			</div>
		{/key}
	</div>
{:else}
	<NotFound
		icon={Copy}
		title="No duplicates found"
		explanation="When you upload duplicate files you can review them here"
	/>
{/if}
