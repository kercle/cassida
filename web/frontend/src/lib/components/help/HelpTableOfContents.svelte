<script lang="ts">
	export let builtins: Array<[string, string]>;
</script>

<div class="bg-base-200 mt-1 font-sans">
	<h1 class="bg-neutral-content p-2 text-2xl font-semibold">Table of contents</h1>
	<div class="p-4">
		<table>
			<tbody>
				{#each builtins as b}
					<tr>
						<td><span class="badge badge-accent font-mono">{b[0]}</span></td>
						<td><span class="ml-4">{b[1]}</span></td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
</div>
