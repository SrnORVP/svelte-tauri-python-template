<script>
	import RadioComp from '$lib/store/CompRadio.svelte';
	import { radioToggle } from '$lib/store/store';
</script>

<div>
	<h1>Radio component</h1>
	<!-- <br /> -->
	<hr />
</div>

<div>
	<RadioComp />
	<hr />
</div>

<div>
	<p>Response: {$radioToggle}</p>
	<hr />
</div>

<style>
</style>
