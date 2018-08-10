rust-webassembly
================

Just some trial & error project in order to test rust with WebAssembly.

## To run:

```
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
git clone https://github.com/Galaf/rust-webassembly.git
cd rust-webassembly
cargo +nightly --target wasm32-unknown-unknown build --release
cargo install --git https://github.com/alexcrichton/wasm-gc
cargo +nightly --target wasm32-unknown-unknown -O --crate-type=cdylib build --release
cargo run
```
