import { beforeAll, describe, expect, it } from "vitest";

import { readFile } from "node:fs/promises";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

import { compile, Dictionary } from "../index.js";

async function getDictionary(name: string) {
  return new Dictionary(
    new Uint8Array(
      compile(
        await readFile(
          join(
            fileURLToPath(new URL(import.meta.url)),
            `../../../examples/${name}.xml`,
          ),
          "utf-8",
        ),
      ),
    ),
  );
}

describe("Dictionary", () => {
  expect.addSnapshotSerializer({
    test: (t) => typeof t.value === "string",
    serialize: (t) => `"${t.value}"`,
  });

  let dict1: Dictionary;
  let dict2: Dictionary;

  beforeAll(async () => {
    dict1 = await getDictionary("example1");
    dict2 = await getDictionary("example2");
  });

  describe("lookup", () => {
    it("looks up terms properly", async () => {
      const result = dict1.lookup({ term: "cat", fallback: "cat" });
      expect(result).toMatchSnapshot();
    });

    it("doesn't split unless specified", async () => {
      const result = dict1.lookup("catdog");
      expect(result[0].length).toBe(0);
    });

    it("can split terms", async () => {
      const result = dict1.lookup("catdog", { split: 3 });
      expect(result).toMatchSnapshot();
    });
  });

  it("can return the lexicon", async () => {
    const result = dict1.lexicon();
    expect(result).toStrictEqual(["cat", "dog", "poo", "ran", "run"]);
  });

  it("can split terms properly", async () => {
    const result = dict1.split("catdog", { threshold: 2 });
    expect(result).toMatchSnapshot();
  });

  it("can index and search a dictionary", async () => {
    dict1.index();

    const results = dict1.search("run");

    expect(results).toMatchSnapshot();
  });

  it("throws errors inside JavaScript", async () => {
    try {
      // @ts-expect-error
      const dict = new Dictionary("fake-alias");
      dict.lookup("dog");
    } catch (e) {
      expect((e as Error).message).toContain(
        "Failed to create reference from Buffer",
      );
    }
  });
});
