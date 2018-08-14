const { draw } = wasm_bindgen;

function run() {
  const canvas = document.getElementById("screen");
  const ctx = canvas.getContext("2d");
  draw(ctx, 600, 600);
}

wasm_bindgen("../wa/rust_webassembly_bg.wasm").then(run);
