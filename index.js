const crackle_pop = import('./rust_wasm_crackle_pop');

crackle_pop.then(cp => {
	console.log(cp.get_result(100));
});
