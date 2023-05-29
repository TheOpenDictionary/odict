import { describe, it } from "@jest/globals";

import { readFile, writeFile } from "node:fs/promises";

import { Dictionary } from "../src";

describe("Dictionary", () => {
  let dict1: Dictionary;

  beforeAll(async () => {
    const xmlContents = await readFile(
      new URL("../../examples/example1.xml", import.meta.url),
      "utf-8"
    );

    const compiled = await Dictionary.compile(xmlContents);

    await writeFile(
      new URL("../../examples/example1.odict", import.meta.url),
      compiled
    );

    dict1 = await Dictionary.load("dict", compiled);
  });

  it("can lookup words properly", async () => {
    const result = await dict1.lookup("run");
    console.log(typeof result);
    expect(result[0][0].term).toBe("run");
  });
});
