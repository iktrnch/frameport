<script lang="ts">
	import type { LucideIcon } from '@lucide/svelte';

	let {
		icon: Icon,
		title,
		explanation
	}: {
		icon: LucideIcon;
		title: string;
		explanation: string;
	} = $props();
</script>

<section
	class="flex h-full w-full items-center justify-center rounded-3xl border-4 border-neutral-800 bg-transparent p-8 text-center text-neutral-600"
>
	<div class="flex max-w-md flex-col items-center">
		<Icon class="h-20 w-20" aria-hidden="true" />
		<h1 class="mt-6 text-2xl font-semibold text-current">{title}</h1>
		<p class="mt-3 text-sm leading-6 text-current">{explanation}</p>
	</div>
</section>
