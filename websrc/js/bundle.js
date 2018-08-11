WebAssembly.instantiateStreaming(fetch("/wa/rwa.min.wasm")).then(results => {
  let fill = results.instance.exports.fill;
  console.log(fill());
});
