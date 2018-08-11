WebAssembly.instantiateStreaming(fetch("/wa/rwa.min.wasm")).then(results => {
  alert(results.instance.exports.add_one(41));
});
