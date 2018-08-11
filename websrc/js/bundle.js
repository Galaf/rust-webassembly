WebAssembly.instantiateStreaming(fetch("/wa/rwa.min.wasm")).then(results => {
  let alloc = results.instance.exports.alloc;
  let buffer = results.instance.exports.memory.buffer;
  let dealloc = results.instance.exports.dealloc;
  let fill = results.instance.exports.fill;

  let width = 600;
  let height = 600;

  let canvas = document.getElementById("screen");

  if (canvas.getContext) {
    let ctx = canvas.getContext("2d");

    let byteSize = width * height * 4;
    console.log(byteSize);
    let pointer = alloc(byteSize);
    console.log(pointer);

    let usub = new Uint8ClampedArray(buffer, pointer, byteSize);
    let img = new ImageData(usub, width, height);

    fill(pointer, width, height);
    ctx.putImageData(img, 0, 0);
  }
});
