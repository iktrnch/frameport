<script lang="ts">
	import DeviceSetup from '$lib/DeviceSetup.svelte';
	import Modal from '$lib/Modal.svelte';
	import { Pencil, Upload } from '@lucide/svelte';

	let {
		name,
		mediaPath
	}: {
		name: string;
		mediaPath: string;
	} = $props();

	let editModalOpen = $state(false);

	async function startUpload() {
		// TODO: wire to backend upload command.
	}

	function closeEditModal() {
		editModalOpen = false;
	}
</script>

<article
	class="flex w-full items-center justify-between gap-6 rounded-3xl border border-white/10 bg-neutral-950/70 p-6 shadow-xl shadow-black/25"
>
	<div class="min-w-0">
		<h2 class="truncate text-2xl font-semibold text-slate-50">{name}</h2>
		<p class="mt-2 truncate text-sm text-slate-500">{mediaPath}</p>
	</div>

	<div class="flex shrink-0 items-center gap-3">
		<button
			type="button"
			onclick={startUpload}
			class="flex h-11 items-center gap-2 rounded-xl bg-sky-400 px-5 text-sm font-semibold text-neutral-950 shadow-lg shadow-black/20 transition hover:bg-sky-300 focus:outline-none focus:ring-2 focus:ring-sky-200/70"
		>
			<Upload class="h-4 w-4" aria-hidden="true" />
			Start Upload
		</button>
		<button
			type="button"
			aria-label={`Edit ${name}`}
			onclick={() => (editModalOpen = true)}
			class="flex h-11 w-11 items-center justify-center rounded-xl border border-white/10 bg-white/[0.04] text-slate-300 shadow-sm shadow-black/20 transition hover:bg-white/[0.08] hover:text-slate-50 focus:outline-none focus:ring-2 focus:ring-white/10"
		>
			<Pencil class="h-4 w-4" aria-hidden="true" />
		</button>
	</div>
</article>

<Modal bind:open={editModalOpen} showCloseButton>
	<DeviceSetup
		title="Edit device"
		sourceDeviceName={name}
		initialDisplayName={name}
		initialMediaPath={mediaPath}
		submitLabel="Save"
		onCancel={closeEditModal}
		onSubmit={closeEditModal}
	/>
</Modal>
