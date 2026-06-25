<script lang="ts">
	import '../app.css';
	import Sidebar from './Sidebar.svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { onMount } from 'svelte';

	let { children } = $props();

	let isOnboarding = $derived(page.url.pathname === '/onboarding');

	onMount(async () => {
		if (page.url.pathname === '/') {
			await goto('/onboarding', { replaceState: true });
		}
	});
</script>

<div class="flex h-screen max-h-screen overflow-hidden bg-neutral-900 text-slate-100">
	{#if !isOnboarding}
		<Sidebar />
	{/if}
	<div class={`min-h-0 min-w-0 flex-1 overflow-hidden ${isOnboarding ? 'p-8' : 'p-5'}`}>
		{@render children()}
	</div>
</div>
