<script>
	import InputComp from '$lib/store/CompInput.svelte';
	import { validText, initString } from '$lib/store/store';
</script>

<div>
	<h1>Text input component</h1>
	<!-- <br /> -->
	<hr />
</div>

<div>
	<InputComp />
	<hr />
</div>

<div>
	<!-- <p>Text outside component:</p> -->
	<p>Response: {$validText}</p>
	<hr />
</div>

<style>
</style>
