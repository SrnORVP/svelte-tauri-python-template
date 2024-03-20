<script>
	import { invoke } from '@tauri-apps/api/tauri';

    let svg_content = ""

	async function serve_svg() {
		// to invoke server
		let temp = await invoke('serve_svg', );
		svg_content = temp;
	}
</script>

<button on:click={serve_svg}> Submit </button>

<div>
	{@html svg_content}
</div>
