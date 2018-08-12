fetch("/wa/rwa.min.wasm")
  .then(reponse => reponse.arrayBuffer())
  .then(bytes => WebAssembly.instantiate(bytes))
  .then(results => {
    let alloc = results.instance.exports.alloc;
    let dealloc = results.instance.exports.dealloc;
    let fill = results.instance.exports.fill;

    let width = 600;
    let height = 600;

    let byteSize = width * height * 4;
    let pointer = alloc(byteSize);
    let buffer = new Uint8Array(
      results.instance.exports.memory.buffer,
      pointer,
      byteSize,
    );

    let canvas = document.getElementById("screen");

    if (canvas.getContext) {
      let ctx = canvas.getContext("2d");

      let pointer = alloc(byteSize);

      let usub = new Uint8ClampedArray(
        results.instance.exports.memory.buffer,
        pointer,
        byteSize,
      );
      let img = new ImageData(usub, width, height);

      fill(pointer, width, height);
      ctx.putImageData(img, 0, 0);
    }
  });
