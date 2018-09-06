const { draw } = wasm_bindgen;

function getSubmitData() {
  return {
    iterations: parseInt(document.getElementById("iterations").value, 10),
    xmin: parseFloat(document.getElementById("xmin").value),
    xmax: parseFloat(document.getElementById("xmax").value),
    ymin: parseFloat(document.getElementById("ymin").value),
    ymax: parseFloat(document.getElementById("ymax").value),
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
    xmin: formConf.xmin,
    xmax: formConf.xmax,
    ymin: formConf.ymin,
    ymax: formConf.ymax,
  };

  console.log(configuration);

  draw(ctx, configuration);
}

(function() {
  wasm_bindgen("../wa/rust_webassembly_bg.wasm").then(run);
})();
