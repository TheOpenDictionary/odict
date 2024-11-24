import {
  instantiateNapiModuleSync as __emnapiInstantiateNapiModuleSync,
  getDefaultContext as __emnapiGetDefaultContext,
  WASI as __WASI,
  createOnMessage as __wasmCreateOnMessageForFsProxy,
} from "@napi-rs/wasm-runtime";

import __wasmUrl from "./node.wasm32-wasi.wasm?url";

const __wasi = new __WASI({
  version: "preview1",
});

const __emnapiContext = __emnapiGetDefaultContext();

const __sharedMemory = new WebAssembly.Memory({
  initial: 4000,
  maximum: 65536,
  shared: true,
});

const __wasmFile = await fetch(__wasmUrl).then((res) => res.arrayBuffer());

const {
  instance: __napiInstance,
  module: __wasiModule,
  napiModule: __napiModule,
} = __emnapiInstantiateNapiModuleSync(__wasmFile, {
  context: __emnapiContext,
  asyncWorkPoolSize: 4,
  wasi: __wasi,
  onCreateWorker() {
    const worker = new Worker(
      new URL("./wasi-worker-browser.mjs", import.meta.url),
      {
        type: "module",
      },
    );

    return worker;
  },
  overwriteImports(importObject) {
    importObject.env = {
      ...importObject.env,
      ...importObject.napi,
      ...importObject.emnapi,
      memory: __sharedMemory,
    };
    return importObject;
  },
  beforeInit({ instance }) {
    __napi_rs_initialize_modules(instance);
  },
});

function __napi_rs_initialize_modules(__napiInstance) {
  __napiInstance.exports["__napi_register__Dictionary_struct_0"]?.();
  __napiInstance.exports["__napi_register__Dictionary_impl_10"]?.();
  __napiInstance.exports["__napi_register__Definition_struct_11"]?.();
  __napiInstance.exports["__napi_register__DictionaryOptions_struct_12"]?.();
  __napiInstance.exports["__napi_register__Entry_struct_13"]?.();
  __napiInstance.exports["__napi_register__Etymology_struct_14"]?.();
  __napiInstance.exports["__napi_register__Example_struct_15"]?.();
  __napiInstance.exports["__napi_register__Group_struct_16"]?.();
  __napiInstance.exports["__napi_register__IndexOptions_struct_17"]?.();
  __napiInstance.exports["__napi_register__LookupOptions_struct_18"]?.();
  __napiInstance.exports["__napi_register__LookupQuery_struct_19"]?.();
  __napiInstance.exports["__napi_register__MarkdownStrategy_20"]?.();
  __napiInstance.exports["__napi_register__MDString_struct_21"]?.();
  __napiInstance.exports["__napi_register__MDString_impl_25"]?.();
  __napiInstance.exports["__napi_register__Note_struct_26"]?.();
  __napiInstance.exports["__napi_register__SearchOptions_struct_27"]?.();
  __napiInstance.exports["__napi_register__Sense_struct_28"]?.();
  __napiInstance.exports["__napi_register__SplitOptions_struct_29"]?.();
}
export const Dictionary = __napiModule.exports.Dictionary;
export const MdString = __napiModule.exports.MdString;
export const MDString = __napiModule.exports.MDString;
export const MarkdownStrategy = __napiModule.exports.MarkdownStrategy;
