<script lang="ts">
	import { appState } from '$lib';
	import InputCell from '$lib/components/InputCell.svelte';
	import ResultCell from '$lib/components/ResultCell.svelte';
	import { tick } from 'svelte';
	import { fade } from 'svelte/transition';
	import tortoise_beatle from '$lib/assets/tortoise_beatle.svg';

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

<div class="content-area absolute inset-0 -z-10 flex items-center justify-center">
	{#if $appState.data?.history.length == 0}
		<div out:fade={{ duration: 100 }} class="scale-200 opacity-80">
			<img src={tortoise_beatle} alt="App Logo" />
		</div>
	{/if}
</div>

<div class="flex h-screen flex-col overflow-hidden">
	<div bind:this={scrollContainer} class="grow overflow-y-auto p-8">
		<div class="flex flex-col gap-4">
			{#each $appState.data?.history as entry}
				<ResultCell {entry} />
			{/each}
		</div>
	</div>

	<div class="sticky bottom-0 p-8 pt-4">
		<InputCell />
	</div>
</div>
