WebAssembly.instantiateStreaming(fetch("../wa/rwa.wasm"))
.then(results => {
    alert(results.instance.exports.add_one(41));
});
