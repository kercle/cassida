<script lang="ts">
export let title: string;
export let patterns: Array<[string, string]>;
</script>

<div class="bg-base-200 font-sans mt-1">
<h1 class="p-2 bg-neutral-content text-2xl font-semibold">{title}</h1>
<ul class="ml-8 p-4 list-disc">
    {#each patterns as pat }
        <li>
            <div class="badge badge-accent font-mono">{pat[0]}</div>
            <div class="mt-2">{pat[1]}</div>
        </li>
    {/each}
</ul>
</div>
