<script>
	import { validText, initString } from './store';
	let inputPath = initString;

	function hChange(e) {
		validText.validate(e);
	}

	function hClick(e) {
		if (e.target.value == initString) {
			e.target.value = '';
		}
	}

	function hOut(e) {
		if (e.target.value == '') {
			e.target.value = initString;
		}
	}

	function hReset() {
		validText.reset();
		inputPath = $validText;
	}
</script>

<div>
	<label for="text_input">
		<input
			name="text_input"
			type="text"
			bind:value={inputPath}
			on:change={hChange}
			on:click={hClick}
			on:focusout={hOut}
		/>
		<button on:click={hReset}> Reset </button>
	</label>
</div>

<!-- <div>
<br />
<hr />
	<p>Text within component: </p>
	<p>{$validText}</p>
</div> -->

<style>
	input[type='text'] {
		width: 80%;
		padding: 0.5rem 0.5rem;
		margin: 0.5rem 1rem 0.5rem 1rem;
		box-sizing: border-box;
	}

	button {
		height: 2.5rem;
	}
</style>
