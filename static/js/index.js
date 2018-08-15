const { draw } = wasm_bindgen;

function run() {
  const canvas = document.getElementById("screen");
  const ctx = canvas.getContext("2d");
  const configuration = {
    iterations: 10000,
    width: canvas.width,
    height: canvas.height,
  };
  draw(ctx, configuration);
}

wasm_bindgen("../wa/rust_webassembly_bg.wasm").then(run);
