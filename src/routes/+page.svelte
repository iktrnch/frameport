<script lang="ts">
	type StatWidget = {
		label: string;
		value: string;
	};

	type JobStatus = 'Running' | 'Queued' | 'Paused';

	type Job = {
		id: string;
		name: string;
		status: JobStatus;
		progress: number;
		detail: string;
	};

	// Replace these assignments with Tauri invoke calls once the backend commands exist.
	let totalMedia = 1284;
	let totalDiskUsage = '482.6 GB';
	let duplicatesToReview = 37;
	let runningJobs: Job[] = [
		{
			id: 'job-001',
			name: 'Importing camera roll',
			status: 'Running',
			progress: 72,
			detail: '928 of 1,284 files scanned'
		},
		{
			id: 'job-002',
			name: 'Generating previews',
			status: 'Running',
			progress: 46,
			detail: '594 thumbnails remaining'
		},
		{
			id: 'job-003',
			name: 'Duplicate analysis',
			status: 'Queued',
			progress: 0,
			detail: 'Waiting for import to finish'
		},
		{
			id: 'job-004',
			name: 'Metadata indexing',
			status: 'Paused',
			progress: 33,
			detail: 'Paused by user'
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

	function statusClasses(status: JobStatus) {
		if (status === 'Running') {
			return 'border-emerald-400/20 bg-emerald-400/10 text-emerald-200';
		}

		if (status === 'Queued') {
			return 'border-sky-400/20 bg-sky-400/10 text-sky-200';
		}

		return 'border-amber-400/20 bg-amber-400/10 text-amber-200';
	}
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

			<div class="mt-5 rounded-xl border border-white/10 bg-white/[0.03] p-4 lg:hidden">
				<p class="text-sm font-medium text-slate-400">Running jobs</p>
				<p class="mt-2 text-3xl font-semibold text-slate-50">{runningJobs.length}</p>
			</div>

			<div class="mt-5 hidden min-h-0 flex-1 flex-col gap-3 overflow-hidden pr-1 lg:flex lg:overflow-y-auto">
				{#each runningJobs as job}
					<article class="rounded-xl border border-white/10 bg-white/[0.03] p-4">
						<div class="flex items-start justify-between gap-4">
							<div class="min-w-0">
								<h2 class="truncate text-sm font-medium text-slate-100">
									{job.name}
								</h2>
								<p class="mt-1 text-sm text-slate-400">{job.detail}</p>
							</div>
							<span
								class={`rounded-full border px-2.5 py-1 text-xs ${statusClasses(job.status)}`}
							>
								{job.status}
							</span>
						</div>

						<div class="mt-4 flex items-center gap-3">
							<div class="h-2 flex-1 overflow-hidden rounded-full bg-white/10">
								<div
									class="h-full rounded-full bg-slate-200"
									style={`width: ${job.progress}%`}
								></div>
							</div>
							<p class="w-10 text-right text-xs text-slate-400">{job.progress}%</p>
						</div>
					</article>
				{/each}
			</div>
		</section>
	</section>
</main>
