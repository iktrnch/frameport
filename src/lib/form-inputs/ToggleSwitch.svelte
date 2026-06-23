<script lang="ts">
	type Props = {
		label: string;
		description: string;
		value: boolean;
		disabled?: boolean;
	};

	let { label, description, value = $bindable(), disabled = false }: Props = $props();
</script>

<div
	class="flex items-center justify-between gap-6 rounded-2xl border border-white/10 bg-white/[0.03] p-5"
>
	<div>
		<p class="text-sm font-medium text-slate-100">{label}</p>
		<p class="mt-1 text-sm text-slate-400">{description}</p>
	</div>
	<button
		type="button"
		role="switch"
		aria-checked={value}
		aria-label={label}
		{disabled}
		onclick={() => (value = !value)}
		class={`relative h-7 w-12 shrink-0 rounded-full border transition focus:outline-none focus:ring-2 focus:ring-white/10 disabled:opacity-60 ${
			value ? 'border-slate-100 bg-slate-100' : 'border-white/10 bg-neutral-950'
		}`}
	>
		<span
			class={`absolute top-1 h-5 w-5 rounded-full transition ${
				value ? 'left-6 bg-neutral-950' : 'left-1 bg-slate-500'
			}`}
		></span>
	</button>
</div>
