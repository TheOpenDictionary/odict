import { beforeAll, describe, expect, it } from "vitest";

import { readFile } from "node:fs/promises";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

import { compile, Dictionary } from "../index.js";

async function getDictionary(name: string) {
  return new Dictionary(
    compile(
      await readFile(
        join(
          fileURLToPath(new URL(import.meta.url)),
          `../../../examples/${name}.xml`,
        ),
        "utf-8",
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
  let dict3: Dictionary;

  beforeAll(async () => {
    dict1 = await getDictionary("example1");
    dict2 = await getDictionary("example2");
    dict3 = await getDictionary("example3");
  });

  describe("lookup", () => {
    it("looks up terms properly", async () => {
      const result = dict1.lookup("cat");
      expect(result).toMatchSnapshot();
    });

    it("doesn't split unless specified", async () => {
      const result = dict1.lookup("catdog");
      expect(result.length).toBe(0);
    });

    it("follows terms properly", async () => {
      const result = dict1.lookup("ran", { follow: true });
      expect(result[0].entry.term).toBe("run");
      expect(result[0].directedFrom?.term).toBe("ran");
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

  it.skipIf(process.env.NO_TOKENIZE)("should tokenize text and find entries", () => {
    const tokens = dict3.tokenize("你好！你是谁？");

    expect(tokens).toMatchSnapshot();
    expect(tokens.length).toBeGreaterThan(0);
    expect(tokens[0].lemma).toBe("你好");
    expect(tokens[0].entries[0].entry.term).toBe("你");
    expect(tokens[0].entries[1].entry.term).toBe("好");
  });

  it.skipIf(process.env.NAPI_RS_FORCE_WASI)(
    "can index and search a dictionary",
    async () => {
      dict1.index();

      const results = dict1.search("run");

      expect(results).toMatchSnapshot();
    },
  );

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
