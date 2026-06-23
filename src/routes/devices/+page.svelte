<script lang="ts">
	import Modal from '$lib/Modal.svelte';
	import NotFound from '$lib/NotFound.svelte';
	import { Camera, ScanSearch } from '@lucide/svelte';
	import { onMount } from 'svelte';

	let discoverModalOpen = $state(false);

	onMount(() => {
		discoverModalOpen = true;
	});
</script>

<div class="relative flex h-full w-full">
	<NotFound
		icon={Camera}
		title="No devices added"
		explanation="Add new device in the bottom right corner"
	/>

	<button
		type="button"
		aria-label="Discover device"
		onclick={() => (discoverModalOpen = true)}
		class="group absolute right-5 bottom-5 flex h-13 w-13 items-center justify-start overflow-hidden rounded-full border border-sky-300/35 bg-sky-500/90 px-3.5 text-white shadow-lg shadow-black/25 ring-1 ring-white/20 transition-[width,background-color,border-color,box-shadow,transform] duration-300 ease-out hover:w-36 hover:border-sky-200/50 hover:bg-sky-400 focus:w-36 focus:outline-none focus:ring-2 focus:ring-sky-200/70 active:translate-y-px"
	>
		<ScanSearch class="h-6 w-6 shrink-0" aria-hidden="true" />
		<span
			aria-hidden="true"
			class="ml-3 h-6 w-px shrink-0 bg-white/0 transition-colors duration-200 group-hover:bg-white/45 group-focus:bg-white/45"
		></span>
		<span
			class="ml-3 max-w-0 overflow-hidden whitespace-nowrap text-sm font-semibold opacity-0 transition-[max-width,opacity] duration-300 ease-out group-hover:max-w-20 group-hover:opacity-100 group-focus:max-w-20 group-focus:opacity-100"
		>
			Discover
		</span>
	</button>

	<Modal bind:open={discoverModalOpen} showCloseButton />
</div>
