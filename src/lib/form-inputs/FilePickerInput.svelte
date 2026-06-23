<script lang="ts">
	import { FolderOpen } from '@lucide/svelte';

	type Props = {
		label: string;
		description: string;
		value: string;
		buttonLabel?: string;
		disabled?: boolean;
		onSelect: () => void | Promise<void>;
	};

	let {
		label,
		description,
		value = $bindable(),
		buttonLabel = 'Choose',
		disabled = false,
		onSelect
	}: Props = $props();
</script>

<div class="grid gap-3 rounded-2xl border border-white/10 bg-white/3 p-5">
	<div>
		<p class="text-sm font-medium text-slate-100">{label}</p>
		<p class="mt-1 text-sm text-slate-400">{description}</p>
	</div>
	<div class="flex gap-3">
		<input
			type="text"
			bind:value
			readonly
			class="h-11 min-w-0 flex-1 rounded-xl border border-white/10 bg-neutral-950 px-4 text-sm text-slate-100 outline-none"
		/>
		<button
			type="button"
			{disabled}
			onclick={onSelect}
			class="flex h-11 items-center gap-2 rounded-xl border border-white/10 bg-white/4 px-4 text-sm font-medium text-slate-100 transition hover:bg-white/8 focus:outline-none focus:ring-2 focus:ring-white/10 disabled:opacity-60"
		>
			<FolderOpen class="h-4 w-4" aria-hidden="true" />
			{buttonLabel}
		</button>
	</div>
</div>
