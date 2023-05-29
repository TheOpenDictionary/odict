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

    expect(compiled.length).toBeGreaterThan(0);

    await writeFile(
      new URL("../../examples/example1.odict", import.meta.url),
      compiled
    );

    dict1 = await Dictionary.load("dict", compiled);
  });

  it("can lookup words properly", async () => {
    const result = await dict1.lookup("run");
    expect(result[0][0].term).toBe("run");
  });

  it("returns a lexicon properly", async () => {
    const result = await dict1.lexicon();
    expect(result).toHaveLength(4);
  });

  it("can split terms during lookup", async () => {
    const result = await dict1.lookup("catdog", { split: 3 });
    expect(result[0][0].term).toBe("cat");
    expect(result[0][1].term).toBe("dog");
  });

  it("can split terms properly", async () => {
    const result = await dict1.split("catdog", 2);
    expect(result[0].term).toBe("cat");
    expect(result[1].term).toBe("dog");
  });
});
