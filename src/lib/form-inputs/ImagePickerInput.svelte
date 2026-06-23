<script lang="ts">
	import { ImagePlus } from '@lucide/svelte';

	type Props = {
		label: string;
		description: string;
		value: string;
		previewSrc?: string;
		fallbackSrc?: string;
		buttonLabel?: string;
		disabled?: boolean;
		onSelect: () => void | Promise<void>;
	};

	let {
		label,
		description,
		value = $bindable(),
		previewSrc,
		fallbackSrc = '/avatar-placeholder.svg',
		buttonLabel = 'Upload',
		disabled = false,
		onSelect
	}: Props = $props();
</script>

<div class="grid gap-3 rounded-2xl border border-white/10 bg-white/[0.03] p-5">
	<div>
		<p class="text-sm font-medium text-slate-100">{label}</p>
		<p class="mt-1 text-sm text-slate-400">{description}</p>
	</div>
	<div class="flex items-center gap-4">
		<div class="h-16 w-16 overflow-hidden rounded-2xl border border-white/10 bg-neutral-900">
			<img src={previewSrc || value || fallbackSrc} alt={label} class="h-full w-full object-cover" />
		</div>
		<button
			type="button"
			{disabled}
			onclick={onSelect}
			class="flex h-11 items-center gap-2 rounded-xl border border-white/10 bg-white/[0.04] px-4 text-sm font-medium text-slate-100 transition hover:bg-white/[0.08] focus:outline-none focus:ring-2 focus:ring-white/10 disabled:opacity-60"
		>
			<ImagePlus class="h-4 w-4" aria-hidden="true" />
			{buttonLabel}
		</button>
	</div>
</div>
