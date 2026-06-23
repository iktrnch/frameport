<script lang="ts">
	import '../app.css';
	import Sidebar from './Sidebar.svelte';

	let { children } = $props();
</script>

<div class="flex h-screen max-h-screen overflow-hidden bg-neutral-900 text-slate-100">
	<Sidebar />
	<div class="min-h-0 min-w-0 flex-1 overflow-hidden p-5">
		{@render children()}
	</div>
</div>
