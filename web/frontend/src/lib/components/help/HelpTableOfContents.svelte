<script lang="ts">
	import HelpLink from './HelpLink.svelte';

	export let builtins: Array<[string, string, string]>;

	const groupCategories = (builtins: Array<[string, string, string]>) => {
		let result: Record<string, Array<[string, string]>> = {};

		for (let [head, summary, cat] of builtins) {
			console.log(head);
			if (!(cat in result)) {
				result[cat] = [];
			}
			result[cat].push([head, summary]);
		}

		return result;
	};
</script>

<div class="bg-base-200 mt-1 font-sans">
	<h1 class="bg-neutral-content p-2 text-2xl font-semibold">Table of contents</h1>
	<div class="p-4">
		{#each Object.entries(groupCategories(builtins)) as [category, items]}
			<details class="collapse collapse-arrow">
				<summary class="collapse-title font-semibold after:inset-s-5 after:end-auto pe-4 ps-12">{category}</summary>
				<div class="collapse-content text-sm">
					<table>
						<tbody>
							{#each items as [head, summary]}
								<tr>
									<td class="p-0.5"><HelpLink symbol={head} /></td>
									<td><span class="ml-4">{summary}</span></td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			</details>
		{/each}
	</div>
</div>
