const { run } = wasm_bindgen;

function start() {
  run();
}

wasm_bindgen("../wa/rust_webassembly_bg.wasm").then(start);
