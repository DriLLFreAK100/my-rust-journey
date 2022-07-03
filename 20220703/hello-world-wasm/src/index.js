import("../pkg")
    .then(rust => rust.hello_world())
    .catch(e => console.error("Failed loading Wasm module:", e));