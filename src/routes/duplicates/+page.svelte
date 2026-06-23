<script lang="ts">
	import NotFound from '$lib/NotFound.svelte';
	import { Copy } from '@lucide/svelte';
</script>

<NotFound
	icon={Copy}
	title="No duplicates found"
	explanation="When you upload duplicate files you can review them here"
/>
