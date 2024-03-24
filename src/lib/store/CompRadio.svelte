<script>
	import { radioToggle } from './store';

	let allOps = ['Option1', 'Option2', 'Option3'];
	$: ops = allOps[0];
	$: radioToggle.init(ops);
</script>

{#each allOps as op}
	<label class="container">
		<input type="radio" name="ops" value={op} bind:group={ops} />
		{op}
		<span class="checkmark"></span>
	</label>
{/each}
<br />


<style>
	label {
		color: white;
		padding-left: 1rem;
	}

	p {
		color: white;
		padding-left: 1rem;
	}

	.container {
		display: block;
		position: relative;
		padding-left: 3rem;
		margin-bottom: 0.5rem;
		width: 5rem;
		cursor: pointer;
		-webkit-user-select: none;
		-moz-user-select: none;
		-ms-user-select: none;
		user-select: none;
	}

	/* Hide the browser's default checkbox */
	.container input {
		position: absolute;
		opacity: 1;
		cursor: pointer;
		height: 0;
		width: 0;
	}

	/* Create a custom checkbox */
	.checkmark {
		position: absolute;
		top: 0;
		left: 1rem;
		height: 1.1rem;
		width: 1.1rem;
		background-color: #eee;
	}

	/* On mouse-over, add a grey background color */
	.container:hover input ~ .checkmark {
		background-color: #ccc;
	}

	/* When the checkbox is checked, add a blue background */
	.container input:checked ~ .checkmark {
		background-color: #2196f3;
	}

	/* Create the checkmark/indicator (hidden when not checked) */
	.checkmark:after {
		content: '';
		position: absolute;
		display: none;
	}

	/* Show the checkmark when checked */
	.container input:checked ~ .checkmark:after {
		display: block;
	}

	/* Style the checkmark/indicator */
	.container .checkmark:after {
		left: 28%;
		top: 2%;
		width: 30%;
		height: 60%;
		border: solid white;
		border-width: 0 3px 3px 0;
		-webkit-transform: rotate(45deg);
		-ms-transform: rotate(45deg);
		transform: rotate(45deg);
	}
</style>
