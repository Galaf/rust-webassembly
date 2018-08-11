rustc +nightly --target wasm32-unknown-unknown -O --crate-type=cdylib src/web.rs -o websrc/wa/rwa.big.wasm
wasm-gc websrc/wa/rwa.big.wasm websrc/wa/rwa.min.wasm
rm websrc/wa/rwa.big.wasm