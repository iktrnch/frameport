<script lang="ts">
	import Modal from '$lib/Modal.svelte';
	import { EllipsisVertical, Pencil, Trash2 } from '@lucide/svelte';
	import { untrack } from 'svelte';

	type MediaPropValue = string | number | Date;

	type Props = {
		id?: string | number;
		Id?: string | number;
		name?: string;
		Name?: string;
		thumbnail?: string;
		Thumbnail?: string;
		size?: MediaPropValue;
		Size?: MediaPropValue;
		creationDate?: MediaPropValue;
		CreationDate?: MediaPropValue;
	};

	let {
		id,
		Id,
		name,
		Name,
		thumbnail,
		Thumbnail,
		size,
		Size,
		creationDate,
		CreationDate
	}: Props = $props();

	let actionsOpen = $state(false);
	let renameModalOpen = $state(false);
	let draftName = $state('');

	let mediaId = $derived(String(id ?? Id ?? ''));
	let displayName = $state(untrack(() => name ?? Name ?? 'Untitled media'));
	let thumbnailSrc = $derived(thumbnail ?? Thumbnail ?? '');
	let mediaSize = $derived(formatDetail(size ?? Size));
	let createdAt = $derived(formatDetail(creationDate ?? CreationDate));
	let mediaHref = $derived(`/media/${encodeURIComponent(mediaId)}`);

	function formatDetail(value: MediaPropValue | undefined) {
		if (value instanceof Date) {
			return value.toLocaleDateString();
		}

		if (typeof value === 'number') {
			return value.toLocaleString();
		}

		return value?.trim() || 'Unknown';
	}

	function openRenameModal() {
		draftName = displayName;
		actionsOpen = false;
		renameModalOpen = true;
	}

	function closeRenameModal() {
		renameModalOpen = false;
	}

	async function deleteMedia() {
		actionsOpen = false;
		// TODO: wire to backend delete command.
	}

	async function renameMedia() {
		const nextName = draftName.trim();

		if (nextName) {
			displayName = nextName;
		}

		renameModalOpen = false;
		// TODO: wire to backend rename command.
	}
</script>

<article
	class="relative flex min-w-0 flex-col overflow-hidden rounded-xl border border-white/10 bg-neutral-950/70 shadow-lg shadow-black/20"
>
	<a
		href={mediaHref}
		aria-label={`Open ${displayName}`}
		class="block aspect-16/10 overflow-hidden bg-white/4 focus:outline-none focus:ring-2 focus:ring-sky-200/70"
	>
		{#if thumbnailSrc}
			<img src={thumbnailSrc} alt="" class="h-full w-full object-cover" />
		{:else}
			<div
				class="flex h-full w-full items-center justify-center bg-white/4 text-xs text-slate-500"
			>
				No thumbnail
			</div>
		{/if}
	</a>

	<div class="grid min-w-0 gap-1.5 p-3 pb-10">
		<a
			href={mediaHref}
			class="truncate text-sm font-semibold text-slate-50 transition hover:text-sky-200 focus:outline-none focus:ring-2 focus:ring-sky-200/70"
		>
			{displayName}
		</a>

		<p class="flex min-w-0 items-center gap-1.5 text-[11px] text-slate-500">
			<span class="truncate">{mediaSize}</span>
			<span aria-hidden="true">/</span>
			<span class="truncate">{createdAt}</span>
		</p>
	</div>

	<div class="absolute right-2 bottom-2">
		<button
			type="button"
			aria-label={`More actions for ${displayName}`}
			aria-expanded={actionsOpen}
			onclick={() => (actionsOpen = !actionsOpen)}
			class="flex h-7 w-7 items-center justify-center rounded-lg border border-white/10 bg-white/4 text-slate-300 shadow-sm shadow-black/20 transition hover:bg-white/8 hover:text-slate-50 focus:outline-none focus:ring-2 focus:ring-white/10"
		>
			<EllipsisVertical class="h-3.5 w-3.5" aria-hidden="true" />
		</button>

		{#if actionsOpen}
			<div
				class="absolute right-0 bottom-9 z-10 w-32 overflow-hidden rounded-lg border border-white/10 bg-neutral-950 py-1 shadow-xl shadow-black/30"
			>
				<button
					type="button"
					onclick={openRenameModal}
					class="flex w-full items-center gap-2 px-2.5 py-1.5 text-left text-xs text-slate-200 transition hover:bg-white/6"
				>
					<Pencil class="h-3.5 w-3.5" aria-hidden="true" />
					Rename
				</button>
				<button
					type="button"
					onclick={deleteMedia}
					class="flex w-full items-center gap-2 px-2.5 py-1.5 text-left text-xs text-red-300 transition hover:bg-red-500/10"
				>
					<Trash2 class="h-3.5 w-3.5" aria-hidden="true" />
					Delete
				</button>
			</div>
		{/if}
	</div>
</article>

<Modal bind:open={renameModalOpen} showCloseButton onClose={closeRenameModal}>
	<form class="mx-auto grid max-w-md gap-5" onsubmit={renameMedia}>
		<div>
			<h2 class="text-xl font-semibold text-slate-50">Rename media</h2>
			<p class="mt-2 text-sm text-slate-400">
				Update the display name for this library item.
			</p>
		</div>

		<label class="grid gap-2">
			<span class="text-sm font-medium text-slate-100">Name</span>
			<input
				type="text"
				bind:value={draftName}
				class="h-11 rounded-xl border border-white/10 bg-neutral-950 px-4 text-sm text-slate-100 outline-none transition placeholder:text-slate-600 focus:border-white/20 focus:ring-2 focus:ring-white/10"
			/>
		</label>

		<div class="flex justify-end gap-3">
			<button
				type="button"
				onclick={closeRenameModal}
				class="h-10 rounded-xl border border-white/10 px-4 text-sm font-medium text-slate-300 transition hover:bg-white/6 hover:text-slate-50 focus:outline-none focus:ring-2 focus:ring-white/10"
			>
				Cancel
			</button>
			<button
				type="submit"
				class="h-10 rounded-xl bg-sky-400 px-4 text-sm font-semibold text-neutral-950 transition hover:bg-sky-300 focus:outline-none focus:ring-2 focus:ring-sky-200/70"
			>
				Save
			</button>
		</div>
	</form>
</Modal>
