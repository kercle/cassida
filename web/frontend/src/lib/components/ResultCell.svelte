<script lang="ts">
	import Math from './Math.svelte';
	import Plot from './Plot.svelte';

	export let index: number = 1;
	export let entry: ServerMessage | undefined = undefined;

	const getRawInput = (entry: ServerMessage | undefined) => {
		if (!entry) {
			return null;
		}

		if ('evalResult' in entry) {
			return entry.evalResult.input.raw;
		} else if ('plot' in entry) {
			return entry.plot.input.raw;
		} else {
			return null;
		}
	};
</script>

<div class="group w-full">
	<div class="bg-base-200 relative">
		{#if entry && 'parseError' in entry}
			<div class="overflow-x-auto bg-red-200 py-2 pl-6">
				<p>{entry.parseError.input}</p>
			</div>
		{:else}
			<div class="flex flex-row">
				<div class="bg-base-200 text-info-content flex w-20 items-center justify-center">
					(%i{index})
				</div>
				<div class="bg-base-200 overflow-x-auto pt-3 pb-2 pl-6">
					{getRawInput(entry)}
				</div>
			</div>
		{/if}
	</div>

	{#if entry && 'parseError' in entry}
		<div class=" overflow-x-auto border border-red-200 bg-white py-2 pl-6">
			<b class="mr-2">Error:</b>{entry.parseError.msg}
		</div>
	{:else if entry && 'evalResult' in entry}
		<div class="flex flex-row">
			<div class="bg-base-200 text-success-content flex w-20 items-center justify-center">
				(%o{index})
			</div>
			<div class="border-base-200 w-full overflow-x-auto border pl-6">
				<Math expr={entry.evalResult.output.latex} />
			</div>
		</div>
	{:else if entry && 'plot' in entry}
		<div class="flex flex-row">
			<div class="bg-base-200 text-success-content flex w-20 items-center justify-center">
				(%o{index})
			</div>
			<div class="border-base-200 w-full overflow-x-auto border pl-6">
				<Plot data={entry.plot.data} />
			</div>
		</div>
	{:else}
		<p>Unknown server message.</p>
	{/if}
</div>
