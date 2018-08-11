# rust-webassembly

Just some trial & error project in order to test rust with WebAssembly.

## Dependencies

### Rust nightly toolchain

```
rustup toolchain install nightly
```

### Rust wasm32-unknown-unknown target

```
rustup target add wasm32-unknown-unknown --toolchain nightly
```

### wasm-gc

```
cargo install --git https://github.com/alexcrichton/wasm-gc
```

## To run:

```
git clone https://github.com/Galaf/rust-webassembly.git
cd rust-webassembly
./make-wasm.sh
cargo +nightly build
cargo +nightly run
```

Go to [localhost:8000](http://localhost:8000) and have a look!
