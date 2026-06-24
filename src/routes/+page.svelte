<script lang="ts">
	import Job from '$lib/Job.svelte';

	type StatWidget = {
		label: string;
		value: string;
	};

	type RunningJob = {
		id: string;
		name: string;
		currentAmountProcessed: number;
		totalToBeProcessed: number;
	};

	// Replace these assignments with Tauri invoke calls once the backend commands exist.
	let totalMedia = 1284;
	let totalDiskUsage = '482.6 GB';
	let duplicatesToReview = 37;
	let runningJobs: RunningJob[] = [
		{
			id: 'job-001',
			name: 'Importing camera roll',
			currentAmountProcessed: 928,
			totalToBeProcessed: 1284
		},
		{
			id: 'job-002',
			name: 'Generating previews',
			currentAmountProcessed: 690,
			totalToBeProcessed: 1284
		},
		{
			id: 'job-003',
			name: 'Duplicate analysis',
			currentAmountProcessed: 0,
			totalToBeProcessed: 1284
		},
		{
			id: 'job-004',
			name: 'Metadata indexing',
			currentAmountProcessed: 424,
			totalToBeProcessed: 1284
		}
	];

	let statWidgets: StatWidget[] = [
		{
			label: 'Total media',
			value: totalMedia.toLocaleString()
		},
		{
			label: 'Total disk usage',
			value: totalDiskUsage
		},
		{
			label: 'Duplicates',
			value: duplicatesToReview.toLocaleString()
		}
	];
</script>

<main class="h-full overflow-hidden p-8">
	<section class="grid h-full min-h-0 grid-rows-[auto_repeat(4,minmax(0,1fr))] gap-4">
		<div class="grid grid-cols-1 gap-4 lg:grid-cols-3">
			{#each statWidgets as widget}
				<article
					class="rounded-2xl border border-white/10 bg-neutral-950/70 p-5 shadow-xl shadow-black/20"
				>
					<p class="text-sm font-medium text-slate-400">{widget.label}</p>
					<p class="mt-4 text-3xl font-semibold text-slate-50">{widget.value}</p>
				</article>
			{/each}
		</div>

		<section
			class="row-span-4 flex min-h-0 flex-col overflow-hidden rounded-2xl border border-white/10 bg-neutral-950/70 p-5 shadow-xl shadow-black/20"
		>
			<div class="flex items-center justify-between gap-4">
				<div>
					<h1 class="text-xl font-semibold text-slate-50">Jobs</h1>
					<p class="mt-1 text-sm text-slate-400">Current background activity</p>
				</div>
				<p
					class="rounded-full border border-white/10 bg-white/5 px-3 py-1 text-sm text-slate-300"
				>
					{runningJobs.length} active
				</p>
			</div>

			<div class="mt-5 rounded-xl border border-white/10 bg-white/3 p-4 lg:hidden">
				<p class="text-sm font-medium text-slate-400">Running jobs</p>
				<p class="mt-2 text-3xl font-semibold text-slate-50">{runningJobs.length}</p>
			</div>

			<div
				class="mt-5 hidden min-h-0 flex-1 flex-col gap-3 overflow-hidden pr-1 lg:flex lg:overflow-y-auto"
			>
				{#each runningJobs as job}
					<Job
						name={job.name}
						currentAmountProcessed={job.currentAmountProcessed}
						totalToBeProcessed={job.totalToBeProcessed}
					/>
				{/each}
			</div>
		</section>
	</section>
</main>
