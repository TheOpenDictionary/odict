import { createRequire } from "node:module";
import type * as bindings from "../index.js";

const require = createRequire(import.meta.url);
const binding: typeof bindings = require(
  process.env.ODICT_TEST_WASI ? "../node.wasi.cjs" : "../index.js",
);

export const { compile, OpenDictionary } = binding;
export type OpenDictionary = bindings.OpenDictionary;
