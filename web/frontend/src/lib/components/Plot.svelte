<script lang="ts">
	import * as echarts from 'echarts';
	import { onMount } from 'svelte';

	export let data: [];

	let elem: HTMLDivElement;

	onMount(() => {
		var myChart = echarts.init(elem);
		myChart.setOption({
			animation: false,
			grid: {
				top: 30,
				left: 30,
				right: 30,
				bottom: 30,
				containLabel: true
			},
			xAxis: {},
			yAxis: {},
			series: [
				{
					type: 'line',
					showSymbol: false,
					clip: true,
					data: data
				}
			]
		});
	});
</script>

<div bind:this={elem} class="h-100 w-130"></div>
