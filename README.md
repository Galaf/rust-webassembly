# rust-webassembly

Just some trial & error project in order to test rust with WebAssembly.

## Dependencies

### basic-http-server

```
cargo install basic-http-server
```

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

Go to [localhost:4000](http://localhost:4000) and have a look!
