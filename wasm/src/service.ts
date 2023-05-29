import { polyfillCrypto } from "./polyfill.js";

let wasmURL: string;

async function loadWASM(importObject: WebAssembly.Imports) {
  if (typeof window !== "undefined") {
    if (!wasmURL) {
      throw new Error(
        "You need to call initializeWASM() before using ODict in a browser!"
      );
    }

    return WebAssembly.instantiateStreaming(
      fetch(new URL(wasmURL, import.meta.url)),
      importObject
    );
  }

  const fs = await import("node:fs/promises");

  const wasmArrayBuffer = await fs
    .readFile(new URL("odict.wasm", import.meta.url))
    .then((res) => res.buffer);

  return WebAssembly.instantiate(new Uint8Array(wasmArrayBuffer), importObject);
}

export const getService = (() => {
  let service: any = null;

  return async () => {
    if (service) {
      return service;
    }

    await polyfillCrypto();

    // @ts-ignore
    await import("./wasm_exec.js");

    // @ts-ignore
    const go = new Go();

    const result = await loadWASM(go.importObject);

    go.run(result.instance);

    service = (globalThis as any)["odict"];

    return service;
  };
})();

export async function initializeWASM(url?: string) {
  if (url) wasmURL = url;
  await getService();
}
