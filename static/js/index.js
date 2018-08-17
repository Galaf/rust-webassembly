const { draw } = wasm_bindgen;

function getSubmitData() {
    return {
        iterations: parseInt(document.getElementById("iterations").value, 10)
    };
}

function run() {
  const canvas = document.getElementById("screen");
  const ctx = canvas.getContext("2d");

  const formConf = getSubmitData();

  const configuration = {
    iterations: formConf.iterations,
    width: canvas.width,
    height: canvas.height,
  };
  draw(ctx, configuration);
}

(function () {
    wasm_bindgen("../wa/rust_webassembly_bg.wasm").then(run);
})();
