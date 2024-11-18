import { beforeAll, describe, expect, it } from "vitest";

import { existsSync } from "node:fs";
import { rm, stat } from "node:fs/promises";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

import { Dictionary } from "../dist/index.js";

describe("Dictionary", () => {
  expect.addSnapshotSerializer({
    test: (t) => typeof t.value === "string",
    serialize: (t) => `"${t.value}"`,
  });

  let dict1Path: string;
  let dict2Path: string;
  let dict1: Dictionary;
  let dict2: Dictionary;

  beforeAll(async () => {
    dict1Path = join(
      fileURLToPath(new URL(import.meta.url), { windows: false }),
      "../../../../examples/example1.xml"
    );

    dict1 = await Dictionary.compile(dict1Path);

    dict2Path = join(
      fileURLToPath(new URL(import.meta.url), { windows: false }),
      "../../../../examples/example2.xml"
    );

    dict2 = await Dictionary.compile(dict2Path);

    const stat1 = await stat(dict1.path);
    const stat2 = await stat(dict2.path);

    expect(stat1.isFile).toBeTruthy();
    expect(stat2.isFile).toBeTruthy();
  });

  it("returns the path correctly", () => {
    expect(dict1.path).toBe(dict1Path.replace(".xml", ".odict"));
    expect(dict2.path).toBe(dict2Path.replace(".xml", ".odict"));
  });

  describe("lookup", () => {
    it("looks up terms properly", async () => {
      const result = await dict1.lookup({ term: "cat", fallback: "cat" });
      expect(result).toMatchSnapshot();
    });

    it("doesn't split unless specified", async () => {
      const result = await dict1.lookup("catdog");
      expect(result[0].length).toBe(0);
    });

    it("can split terms", async () => {
      const result = await dict1.lookup("catdog", { split: 3 });
      expect(result).toMatchSnapshot();
    });
  });

  it("can return the lexicon", async () => {
    const result = await dict1.lexicon();
    expect(result).toStrictEqual(["cat", "dog", "poo", "ran", "run"]);
  });

  it("can write raw XML", async () => {
    await Dictionary.write(
      '<dictionary><entry term="hello"><ety><sense pos="v"><definition value="hello world" /></sense></ety></entry><entry term="world"><ety><sense pos="v"><definition value="hello world" /></sense></ety></entry></dictionary>"    )',
      "test.odict"
    );

    expect(existsSync("test.odict")).toBeTruthy();

    const dict = new Dictionary("test.odict");

    expect(dict.lookup("hello").length).toBe(1);

    await rm("test.odict");
  });

  it("can split terms properly", async () => {
    const result = await dict1.split("catdog", { threshold: 2 });
    expect(result).toMatchSnapshot();
  });

  it("can index and search a dictionary", async () => {
    await dict1.index();

    const results = await dict1.search("run");

    expect(results).toMatchSnapshot();
  });

  it("throws errors inside JavaScript", async () => {
    try {
      const dict = new Dictionary("fake-alias");
      await dict.lookup("dog");
    } catch (e) {
      expect((e as Error).message).toEqual(
        "No such file or directory (os error 2)"
      );
    }
  });
});
