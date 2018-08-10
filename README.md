rust-webassembly
================

Just some trial & error project in order to test rust with WebAssembly.

## To run:

```
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown --toolchain nightly

git clone https://github.com/Galaf/rust-webassembly.git
cd rust-webassembly
cargo  +nightly build --target wasm32-unknown-unknown --release
cargo run
```
