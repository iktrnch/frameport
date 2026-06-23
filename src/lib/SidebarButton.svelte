<script lang="ts">
	import { page } from '$app/state';
	import type { LucideIcon } from '@lucide/svelte';

	let { label, href, icon: Icon }: { label: string; href: string; icon?: LucideIcon } = $props();

	let isActive = $derived(
		href === '/'
			? page.url.pathname === href
			: page.url.pathname === href || page.url.pathname.startsWith(`${href}/`)
	);
</script>

<a
	{href}
	aria-current={isActive ? 'page' : undefined}
	class={`flex items-center gap-3 rounded-r-2xl rounded-l-md border px-4 py-3 text-left text-sm font-medium shadow-sm shadow-black/20 transition focus:outline-none focus:ring-2 focus:ring-neutral-500 ${
		isActive
			? 'border-white/15 bg-white/12 text-slate-50 ring-1 ring-white/10'
			: 'border-white/5 bg-white/3 text-slate-200 hover:border-white/10 hover:bg-white/8 hover:text-slate-50'
	}`}
>
	{#if Icon}
		<Icon class="h-4 w-4 shrink-0" aria-hidden="true" />
	{/if}
	<span class="truncate">{label}</span>
</a>
