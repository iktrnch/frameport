<script lang="ts">
	let {
		name,
		currentAmountProcessed,
		totalToBeProcessed
	}: {
		name: string;
		currentAmountProcessed: number;
		totalToBeProcessed: number;
	} = $props();

	let progress = $derived(
		totalToBeProcessed > 0
			? Math.min(100, Math.max(0, Math.round((currentAmountProcessed / totalToBeProcessed) * 100)))
			: 0
	);
</script>

<article class="rounded-xl border border-white/10 bg-white/3 p-4">
	<div class="min-w-0">
		<h2 class="truncate text-sm font-medium text-slate-100">{name}</h2>
		<p class="mt-1 text-sm text-slate-400">
			{currentAmountProcessed.toLocaleString()} of {totalToBeProcessed.toLocaleString()} files processed
		</p>
	</div>

	<div class="mt-4 flex items-center gap-3">
		<div
			class="h-2 flex-1 overflow-hidden rounded-full bg-white/10"
			role="progressbar"
			aria-label={`${name} progress`}
			aria-valuenow={progress}
			aria-valuemin="0"
			aria-valuemax="100"
		>
			<div class="h-full rounded-full bg-slate-200" style={`width: ${progress}%`}></div>
		</div>
		<p class="w-10 text-right text-xs text-slate-400">{progress}%</p>
	</div>
</article>
