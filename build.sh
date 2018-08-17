#!/bin/sh

set -ex

cargo +nightly build --target wasm32-unknown-unknown

wasm-bindgen target/wasm32-unknown-unknown/debug/rust_webassembly.wasm --no-modules --out-dir ./static/wa/

basic-http-server ./static/
