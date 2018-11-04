const init = async () => {
	const cp = await import('./rust_wasm_crackle_pop');
	const result = Array.from(Array(100).keys())
		.map(x => x + 1)
		.map(x => cp.get_result(x));
	console.log(result);
};

init();
