<script lang="ts">
	import FilePickerInput from '$lib/form-inputs/FilePickerInput.svelte';
	import TextInput from '$lib/form-inputs/TextInput.svelte';
	import { open } from '@tauri-apps/plugin-dialog';

	let {
		title,
		sourceDeviceName,
		initialDisplayName = '',
		initialMediaPath = '',
		submitLabel = 'Add',
		onCancel,
		onSubmit
	}: {
		title: string;
		sourceDeviceName?: string;
		initialDisplayName?: string;
		initialMediaPath?: string;
		submitLabel?: string;
		onCancel?: () => void;
		onSubmit?: (device: { displayName: string; mediaPath: string }) => void | Promise<void>;
	} = $props();

	let displayName = $state(getInitialDisplayName());
	let mediaPath = $state(getInitialMediaPath());

	function getInitialDisplayName() {
		return initialDisplayName;
	}

	function getInitialMediaPath() {
		return initialMediaPath;
	}

	async function selectMediaPath() {
		const selectedDirectory = await open({
			multiple: false,
			directory: true
		});

		if (typeof selectedDirectory === 'string') {
			mediaPath = selectedDirectory;
		}
	}

	async function submit() {
		await onSubmit?.({
			displayName,
			mediaPath
		});
	}
</script>

<div class="max-w-3xl py-8">
	<div>
		{#if sourceDeviceName}
			<p class="text-sm font-medium text-slate-400">{sourceDeviceName}</p>
		{/if}
		<h2 class="mt-2 text-2xl font-semibold text-slate-50">{title}</h2>
	</div>

	<div class="mt-8 grid gap-5">
		<TextInput
			label="Display name"
			description="The device name shown in Frameport."
			bind:value={displayName}
			placeholder={sourceDeviceName || 'Device name'}
		/>

		<FilePickerInput
			label="Media path"
			description="Folder where this device exposes photos and videos."
			bind:value={mediaPath}
			buttonLabel="Browse"
			onSelect={selectMediaPath}
		/>
	</div>

	<div class="mt-8 flex justify-end gap-3">
		<button
			type="button"
			onclick={onCancel}
			class="h-11 rounded-xl border border-white/10 bg-white/[0.04] px-5 text-sm font-medium text-slate-100 transition hover:bg-white/[0.08] focus:outline-none focus:ring-2 focus:ring-white/10"
		>
			Cancel
		</button>
		<button
			type="button"
			onclick={submit}
			class="h-11 rounded-xl bg-slate-100 px-5 text-sm font-semibold text-neutral-950 transition hover:bg-white focus:outline-none focus:ring-2 focus:ring-white/30"
		>
			{submitLabel}
		</button>
	</div>
</div>
