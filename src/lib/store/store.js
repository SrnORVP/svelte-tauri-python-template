// This one direct mut value across components by the svelte store model

import { get, writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';


export const initString = 'Waiting for user input...';

async function validateString(userInputText, rustFuncName) {
	// to invoke server validation
	let temp = ''
	if (rustFuncName != null) {
		temp = await invoke(rustFuncName, { input_string: userInputText });
	} else {
		temp = userInputText;
	}
	console.log('after validate:', temp);
	return temp
}

export function createPathStore() {
	const { subscribe, set, update } = writable(initString);

	return {
		subscribe,
		validate: async (e) => {
			let p = e.target.value;
			set(`Validating "${p}"`)
			// set(await validateString(p, 'server_validate'))
			set(await validateString(p))
		},
		reset: () => set(initString),
	}
}

function createToggleStore() {
	const { subscribe, set, update } = writable('');

	return {
		subscribe,
		init: (v) => set(v),
		assign: (e) => set(e.target.value),
	}
}

export const validText = createPathStore();
export const radioToggle = createToggleStore();


