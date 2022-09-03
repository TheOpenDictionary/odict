import {join}from 'node:path';
import {fileURLToPath}from 'node:url';
import { Dictionary } from "../src";

describe("Dictionary", () => {
  it("can compile dictionaries properly", () => {
    Dictionary.compile(join(fileURLToPath( new URL(import.meta.url)), "../../../examples/example1.xml"));
  });
});
