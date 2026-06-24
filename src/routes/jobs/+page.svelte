<script lang="ts">
	import Job from '$lib/Job.svelte';
	import NotFound from '$lib/NotFound.svelte';
	import { ListChecks } from '@lucide/svelte';
	import { onMount } from 'svelte';

	type Job = {
		id: string;
		name: string;
		currentAmountProcessed: number;
		totalToBeProcessed: number;
	};

	// Replace this assignment with a Tauri invoke call once the backend command exists.
	let jobs: Job[] = [
		{ id: 'Job', name: 'Job', currentAmountProcessed: 53, totalToBeProcessed: 100 }
	];
</script>

{#if jobs.length > 0}
	<div
		class="h-full w-full overflow-y-auto rounded-3xl border border-white/10 bg-neutral-950/30 p-6"
	>
		<div class="grid gap-4">
			{#each jobs as job}
				<Job
					name={job.name}
					currentAmountProcessed={job.currentAmountProcessed}
					totalToBeProcessed={job.totalToBeProcessed}
				/>
			{/each}
		</div>
	</div>
{:else}
	<NotFound
		icon={ListChecks}
		title="No jobs currently running"
		explanation="Upload new media to start processing them"
	/>
{/if}
