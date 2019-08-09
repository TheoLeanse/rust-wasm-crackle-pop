const plusOne = x => x + 1;

const init = async () => {
	const cp = await import('./rust_wasm_crackle_pop');
	const result = x => cp.get_result(x);
	Array.from(Array(100).keys())
		.map(plusOne)
		.map(result)
		.map(console.log);
};

init();
