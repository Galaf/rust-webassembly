# rust-webassembly

Just some trial & error project in order to test rust with WebAssembly.

## Dependencies

### NodeJS

[https://nodejs.org/en/](Download from the official website)

### Rust nightly toolchain

```
rustup toolchain install nightly
```

### Rust wasm32-unknown-unknown target

```
rustup target add wasm32-unknown-unknown --toolchain nightly
```

### wasm-bindgen

```
cargo +nightly install wasm-bindgen-cli
```

## To run:

```
git clone https://github.com/Galaf/rust-webassembly.git
cd rust-webassembly
./build.sh
```

Go to [localhost:8080](http://localhost:8080) and have a look!
