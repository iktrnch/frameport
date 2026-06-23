<script lang="ts">
	import NotFound from '$lib/NotFound.svelte';
	import { ListChecks } from '@lucide/svelte';
</script>

<NotFound
	icon={ListChecks}
	title="No jobs currently running"
	explanation="Upload new media to start processing them"
/>
