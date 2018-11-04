build:
	cargo +nightly build --target wasm32-unknown-unknown

invoke:
	wasm-bindgen target/wasm32-unknown-unknown/debug/rust_wasm_crackle_pop.wasm --out-dir .
