rust-webassembly
================

Just some trial & error project in order to test rust with WebAssembly.

## To run:

```
rustup target add wasm32-unknown-emscripten
sudo apt-get install emscripten

git clone https://github.com/Galaf/rust-webassembly.git
cd rust-webassembly
cargo build
cargo run
```

## I don't use Linux

`apt-get` should be the only difference with Windows and macOS.
Please refer to [Emscripten's official website](http://kripken.github.io/emscripten-site/) to learn how to install Emscripten on your computer.
