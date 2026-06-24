<script lang="ts">
	import Media from '$lib/Media.svelte';
	import NotFound from '$lib/NotFound.svelte';
	import { SquarePlay } from '@lucide/svelte';

	type LibraryMedia = {
		id: string;
		name: string;
		thumbnail: string;
		size: string;
		creationDate: string;
	};

	const mediaItems: LibraryMedia[] = [
		{
			id: 'summer-roll-001',
			name: 'Summer coast roll',
			thumbnail: '/svelte.svg',
			size: '2.4 GB',
			creationDate: '12 Jun 2026'
		},
		{
			id: 'studio-session-014',
			name: 'Studio portrait session',
			thumbnail: '/vite.svg',
			size: '842 MB',
			creationDate: '08 Jun 2026'
		},
		{
			id: 'product-b-roll-027',
			name: 'Product b-roll selects',
			thumbnail: '/tauri.svg',
			size: '1.8 GB',
			creationDate: '31 May 2026'
		},
		{
			id: 'product-b-roll-027',
			name: 'Product b-roll selects',
			thumbnail: '/tauri.svg',
			size: '1.8 GB',
			creationDate: '31 May 2026'
		},
		{
			id: 'product-b-roll-027',
			name: 'Product b-roll selects',
			thumbnail: '/tauri.svg',
			size: '1.8 GB',
			creationDate: '31 May 2026'
		},
		{
			id: 'product-b-roll-027',
			name: 'Product b-roll selects',
			thumbnail: '/tauri.svg',
			size: '1.8 GB',
			creationDate: '31 May 2026'
		},
		{
			id: 'city-night-009',
			name: 'City night timelapse',
			thumbnail: '',
			size: '3.1 GB',
			creationDate: '25 May 2026'
		}
	];
</script>

{#if mediaItems.length > 0}
	<div
		class="h-full w-full overflow-y-auto rounded-3xl border border-white/10 bg-neutral-950/30 p-5"
	>
		<div class="grid grid-cols-[repeat(auto-fill,minmax(160px,1fr))] gap-4">
			{#each mediaItems as media}
				<Media
					id={media.id}
					name={media.name}
					thumbnail={media.thumbnail}
					size={media.size}
					creationDate={media.creationDate}
				/>
			{/each}
		</div>
	</div>
{:else}
	<NotFound
		icon={SquarePlay}
		title="No media found"
		explanation="Upload your media or make sure the folder is accessible"
	/>
{/if}
