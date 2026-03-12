<script lang="ts">
	import { appState } from '$lib';
	import Greet from '$lib/components/Greet.svelte';
	import InputCell from '$lib/components/InputCell.svelte';
	import ResultCell from '$lib/components/ResultCell.svelte';
	import { tick } from 'svelte';

	let scrollContainer: HTMLElement;

	$: if ($appState.data?.history && scrollContainer) {
		scrollToBottom();
	}

	async function scrollToBottom() {
		await tick();
		scrollContainer.scrollTo({
			top: scrollContainer.scrollHeight,
			behavior: 'smooth'
		});
	}
</script>

<div bind:this={scrollContainer} class="h-screen overflow-y-auto p-6">
	<Greet />
	<div class="flex flex-col gap-4">
		{#each $appState.data?.history as entry, index}
			<ResultCell {entry} index={index + 1} />
		{/each}
		<InputCell />
	</div>
</div>
