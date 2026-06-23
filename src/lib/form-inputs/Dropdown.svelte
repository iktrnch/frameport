<script lang="ts">
	import { ChevronDown } from '@lucide/svelte';

	type Props = {
		label: string;
		description: string;
		options: string[];
		defaultOptionIndex: number;
		value: string;
		disabled?: boolean;
	};

	let {
		label,
		description,
		options,
		defaultOptionIndex,
		value = $bindable(),
		disabled = false
	}: Props = $props();

	let isOpen = $state(false);
	let fallbackValue = $derived(options[defaultOptionIndex] ?? options[0] ?? '');
	let selectedValue = $derived(value || fallbackValue);

	function toggleDropdown() {
		if (disabled) {
			return;
		}

		isOpen = !isOpen;
	}

	function selectOption(option: string) {
		value = option;
		isOpen = false;
	}
</script>

<div class="relative grid gap-3 rounded-2xl border border-white/10 bg-white/[0.03] p-5">
	<div>
		<p class="text-sm font-medium text-slate-100">{label}</p>
		<p class="mt-1 text-sm text-slate-400">{description}</p>
	</div>

	<button
		type="button"
		aria-haspopup="listbox"
		aria-expanded={isOpen}
		{disabled}
		onclick={toggleDropdown}
		class="flex h-11 w-full items-center justify-between gap-3 rounded-xl border border-white/10 bg-neutral-950 px-4 text-left text-sm text-slate-100 outline-none transition focus:border-white/20 focus:ring-2 focus:ring-white/10 disabled:opacity-60"
	>
		<span class="truncate">{selectedValue}</span>
		<ChevronDown
			class={`h-4 w-4 shrink-0 text-slate-400 transition ${isOpen ? 'rotate-180' : ''}`}
			aria-hidden="true"
		/>
	</button>

	{#if isOpen}
		<div
			class="absolute right-5 left-5 top-[calc(100%-1.25rem)] z-10 overflow-hidden rounded-xl border border-white/10 bg-neutral-950 shadow-2xl shadow-black/40"
			role="listbox"
			aria-label={label}
		>
			{#each options as option}
				<button
					type="button"
					role="option"
					aria-selected={option === selectedValue}
					onclick={() => selectOption(option)}
					class={`block w-full px-4 py-3 text-left text-sm transition ${
						option === selectedValue
							? 'bg-white/10 text-slate-50'
							: 'text-slate-300 hover:bg-white/[0.06] hover:text-slate-50'
					}`}
				>
					{option}
				</button>
			{/each}
		</div>
	{/if}
</div>
