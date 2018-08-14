const { draw } = wasm_bindgen;

function run() {
  const canvas = document.getElementById("screen");
  const ctx = canvas.getContext("2d");
  draw(ctx, canvas.width, canvas.height);
}

wasm_bindgen("../wa/rust_webassembly_bg.wasm").then(run);
