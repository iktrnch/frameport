<script lang="ts">
	import DeviceSetup from '$lib/DeviceSetup.svelte';
	import { ArrowLeft, Camera, ChevronRight, LoaderCircle } from '@lucide/svelte';
	import { onMount } from 'svelte';
	import { fly } from 'svelte/transition';

	type DiscoveredDevice = {
		id: string;
		name: string;
	};

	type Screen = 'discovering' | 'device-list' | 'device-form';

	let { onClose }: { onClose?: () => void } = $props();

	let screen = $state<Screen>('discovering');
	let discoveredDevice = $state<DiscoveredDevice | null>(null);
	let displayName = $state('');
	let mediaPath = $state('');
	let transitionDirection = $state<1 | -1>(1);

	onMount(() => {
		const timeout = window.setTimeout(() => {
			discoveredDevice = {
				id: 'canon-eos-r6',
				name: 'Canon EOS R6'
			};
			displayName = discoveredDevice.name;
			screen = 'device-list';
		}, 3000);

		return () => window.clearTimeout(timeout);
	});

	function selectDevice(device: DiscoveredDevice) {
		discoveredDevice = device;
		displayName = device.name;
		transitionDirection = 1;
		screen = 'device-form';
	}

	function selectDiscoveredDevice() {
		if (!discoveredDevice) {
			return;
		}

		selectDevice(discoveredDevice);
	}

	function returnToDeviceList() {
		transitionDirection = -1;
		screen = 'device-list';
	}

	function closeModal() {
		onClose?.();
	}
</script>

<div class="flex min-h-full flex-col">
	<div class="border-b border-white/10 pb-6">
		<p class="text-sm font-medium text-slate-400">Devices</p>
		<h1 class="mt-2 text-3xl font-semibold text-slate-50">Discover device</h1>
	</div>

	{#if screen === 'discovering'}
		<div class="flex flex-1 items-center justify-center py-16 text-center">
			<div class="flex max-w-sm flex-col items-center">
				<div
					class="flex h-16 w-16 items-center justify-center rounded-2xl border border-white/10 bg-white/[0.03]"
				>
					<LoaderCircle class="h-7 w-7 animate-spin text-slate-200" aria-hidden="true" />
				</div>
				<h2 class="mt-6 text-xl font-semibold text-slate-50">Searching for connected devices</h2>
				<p class="mt-3 text-sm leading-6 text-slate-400">
					Frameport is checking available media sources.
				</p>
			</div>
		</div>
	{:else}
		<div class="relative min-h-0 flex-1 overflow-hidden">
			{#if screen === 'device-list' && discoveredDevice}
				<div
					class="absolute inset-0 max-w-3xl py-8"
					in:fly={{ x: transitionDirection * 24, duration: 180 }}
					out:fly={{ x: transitionDirection * -24, duration: 140 }}
				>
					<p class="text-sm text-slate-400">Select a device to finish adding it to Frameport.</p>

					<button
						type="button"
						onclick={selectDiscoveredDevice}
						class="mt-6 flex w-full items-center gap-4 rounded-2xl border border-white/10 bg-white/[0.03] p-5 text-left shadow-lg shadow-black/20 transition hover:border-white/20 hover:bg-white/[0.06] focus:outline-none focus:ring-2 focus:ring-white/10"
					>
						<div
							class="flex h-12 w-12 shrink-0 items-center justify-center rounded-xl border border-white/10 bg-neutral-950 text-slate-300"
						>
							<Camera class="h-6 w-6" aria-hidden="true" />
						</div>
						<div class="min-w-0 flex-1">
							<p class="truncate text-sm font-semibold text-slate-50">{discoveredDevice.name}</p>
							<p class="mt-1 truncate text-sm text-slate-400">Ready to configure</p>
						</div>
						<ChevronRight class="h-5 w-5 shrink-0 text-slate-500" aria-hidden="true" />
					</button>
				</div>
			{:else if screen === 'device-form' && discoveredDevice}
				<div
					class="absolute inset-0 max-w-3xl overflow-y-auto py-8"
					in:fly={{ x: transitionDirection * 24, duration: 180 }}
					out:fly={{ x: transitionDirection * -24, duration: 140 }}
				>
					<button
						type="button"
						onclick={returnToDeviceList}
						class="inline-flex h-10 items-center gap-2 rounded-xl border border-white/10 bg-white/[0.04] px-4 text-sm font-medium text-slate-200 transition hover:bg-white/[0.08] hover:text-slate-50 focus:outline-none focus:ring-2 focus:ring-white/10"
					>
						<ArrowLeft class="h-4 w-4" aria-hidden="true" />
						Back
					</button>

					<DeviceSetup
						title="Configure device"
						sourceDeviceName={discoveredDevice.name}
						initialDisplayName={displayName}
						initialMediaPath={mediaPath}
						submitLabel="Add"
						onCancel={closeModal}
						onSubmit={closeModal}
					/>
				</div>
			{/if}
		</div>
	{/if}
</div>
