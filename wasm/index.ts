import './__generated__/wasm_exec.js';

const go = new Go();

WebAssembly.instantiateStreaming(fetch(new URL('./json.wasm', import.meta.url)), go.importObject).then((result) => {
    go.run(result.instance);
});