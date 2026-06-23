<script lang="ts">
	import SidebarButton from '$lib/SidebarButton.svelte';
	import {
		SquarePlay,
		LayoutDashboard,
		ListChecks,
		Settings,
		Copy,
		Camera
	} from '@lucide/svelte';
</script>

<aside
	class="my-5 flex h-[calc(100vh-2.5rem)] w-64 shrink-0 flex-col rounded-r-3xl border-y border-r border-white/10 bg-neutral-950/95 px-4 py-5 text-slate-100 shadow-2xl shadow-black/40 backdrop-blur"
>
	<nav class="flex flex-col gap-2" aria-label="Primary">
		<SidebarButton label="Dashboard" href="/" icon={LayoutDashboard} />
		<SidebarButton label="Media" href="/media" icon={SquarePlay} />
		<SidebarButton label="Duplicates" href="/duplicates" icon={Copy} />
		<SidebarButton label="Devices" href="/devices" icon={Camera} />
		<SidebarButton label="Jobs" href="/jobs" icon={ListChecks} />
	</nav>

	<div
		class="mt-auto flex items-center gap-3 rounded-r-2xl rounded-l-md border border-white/10 bg-white/3 p-3 shadow-lg shadow-black/20"
	>
		<div class="h-10 w-10 overflow-hidden rounded-full bg-neutral-700 ring-1 ring-white/10">
			<img
				src="/avatar-placeholder.svg"
				alt="User avatar"
				class="h-full w-full object-cover"
			/>
		</div>
		<div class="min-w-0 flex-1">
			<p class="truncate text-sm font-medium text-slate-100">Username</p>
			<p class="truncate text-xs text-slate-400">Local user</p>
		</div>
		<a
			href="/settings"
			type="button"
			class="flex h-10 w-10 items-center justify-center rounded-r-xl rounded-l-md border border-white/10 bg-white/4 text-slate-300 shadow-sm shadow-black/20 transition hover:bg-white/8 hover:text-slate-100 focus:outline-none focus:ring-2 focus:ring-neutral-500"
			aria-label="Settings"
		>
			<Settings />
		</a>
	</div>
</aside>
