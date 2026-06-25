<script lang="ts">
	import SettingsView from '$lib/SettingsView.svelte';
	import { goto } from '$app/navigation';

	async function finishOnboarding() {
		await goto('/');
	}
</script>

<SettingsView variant="onboarding" onSaved={finishOnboarding} />
