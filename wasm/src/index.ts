import "./__generated__/wasm_exec.js";
import type { Query } from "./types";

const go = new Go();

const getService = (() => {
  let service: any = null;

  return async () => {
    if (service) {
      return service;
    }

    const result = await WebAssembly.instantiateStreaming(
      fetch(new URL("./odict.wasm", import.meta.url)),
      go.importObject
    );

    go.run(result.instance);

    service = (globalThis as any)["odict"];

    return service;
  };
})();

export class Dictionary {
  private constructor(
    private readonly name: string,
    private readonly service: any
  ) {}

  static async load(name: string, data: ArrayBuffer) {
    const service = await getService();
    await service.loadDictionary(name, data);
    return new Dictionary(name, service);
  }

  lookup(word: Query[] | string[]) {
    console.log(word);
    return this.service.lookupWord(this.name, [word], 0, false);
  }
}
