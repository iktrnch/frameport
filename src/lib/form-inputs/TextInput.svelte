<script lang="ts">
	type Props = {
		label: string;
		description: string;
		value: string;
		disabled?: boolean;
		placeholder?: string;
	};

	let {
		label,
		description,
		value = $bindable(),
		disabled = false,
		placeholder = ''
	}: Props = $props();
</script>

<label class="grid gap-3 rounded-2xl border border-white/10 bg-white/[0.03] p-5">
	<span>
		<span class="block text-sm font-medium text-slate-100">{label}</span>
		<span class="mt-1 block text-sm text-slate-400">{description}</span>
	</span>
	<input
		type="text"
		bind:value
		{disabled}
		{placeholder}
		class="h-11 rounded-xl border border-white/10 bg-neutral-950 px-4 text-sm text-slate-100 outline-none transition placeholder:text-slate-600 focus:border-white/20 focus:ring-2 focus:ring-white/10 disabled:opacity-60"
	/>
</label>
