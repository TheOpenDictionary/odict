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
    test: (t) => typeof t.value === "string" && !("variant" in t),
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

    it("is case-sensitive by default", async () => {
      const result = dict1.lookup("CAT");
      expect(result.length).toBe(0);
    });

    it("can perform case-insensitive lookup", async () => {
      const result = dict1.lookup("CAT", { insensitive: true });
      expect(result.length).toBe(1);
      expect(result[0].entry.term).toBe("cat");
    });

    it("works with mixed case", async () => {
      const result = dict1.lookup(["DoG", "cAT"], { insensitive: true });
      expect(result.length).toBe(2);
      expect(result[0].entry.term).toBe("dog");
      expect(result[1].entry.term).toBe("cat");
    });

    it("combines case-insensitivity with follow option", async () => {
      const result = dict1.lookup("RaN", { follow: true, insensitive: true });
      expect(result[0].entry.term).toBe("run");
      expect(result[0].directedFrom?.term).toBe("ran");
    });
  });

  it("can return the lexicon", async () => {
    const result = dict1.lexicon();
    expect(result).toStrictEqual(["cat", "dog", "poo", "ran", "run"]);
  });

  describe("rank", () => {
    it("returns correct min_rank for dictionary with ranks", () => {
      // example1 has one entry with rank=100 (the "run" entry)
      expect(dict1.minRank).toBe(100);
    });

    it("returns correct max_rank for dictionary with ranks", () => {
      // example1 has one entry with rank=100 (the "run" entry)
      expect(dict1.maxRank).toBe(100);
    });

    it("returns null min_rank for dictionary without ranks", () => {
      // example2 has no rank attributes
      expect(dict2.minRank).toBe(null);
    });

    it("returns null max_rank for dictionary without ranks", () => {
      // example2 has no rank attributes
      expect(dict2.maxRank).toBe(null);
    });

    it("handles mixed entries with and without ranks", async () => {
      // Create a test dictionary with mixed rank entries
      const mixedXml = `<?xml version="1.0" encoding="UTF-8"?>
<dictionary>
  <entry term="high">
    <ety>
      <pos>noun</pos>
      <def>High ranking</def>
    </ety>
  </entry>
  <entry term="medium" rank="50">
    <ety>
      <pos>noun</pos>
      <def>Medium ranking</def>
    </ety>
  </entry>
  <entry term="low" rank="10">
    <ety>
      <pos>noun</pos>
      <def>Low ranking</def>
    </ety>
  </entry>
  <entry term="highest" rank="100">
    <ety>
      <pos>noun</pos>
      <def>Highest ranking</def>
    </ety>
  </entry>
</dictionary>`;

      const mixedDict = new Dictionary(compile(mixedXml));
      expect(mixedDict.minRank).toBe(10);
      expect(mixedDict.maxRank).toBe(100);
    });
  });

  it.skipIf(process.env.NO_TOKENIZE)(
    "should tokenize text and find entries",
    () => {
      const tokens = dict3.tokenize("你好！你是谁？");

      expect(tokens).toMatchSnapshot();
      expect(tokens.length).toBeGreaterThan(0);
      expect(tokens[0].lemma).toBe("你好");
      expect(tokens[0].entries[0].entry.term).toBe("你");
      expect(tokens[0].entries[1].entry.term).toBe("好");
    },
  );

  it.skipIf(process.env.NO_TOKENIZE)(
    "should tokenize text case-sensitively by default",
    () => {
      const tokens = dict1.tokenize("DOG cat");

      expect(tokens.length).toBe(2);
      expect(tokens[0].lemma).toBe("DOG");
      expect(tokens[0].entries.length).toBe(0); // "DOG" shouldn't match "dog"
      expect(tokens[1].lemma).toBe("cat");
      expect(tokens[1].entries[0].entry.term).toBe("cat");
    },
  );

  it.skipIf(process.env.NO_TOKENIZE)(
    "should tokenize text case-insensitively when specified",
    () => {
      const tokens = dict1.tokenize("DOG cat", { insensitive: true });

      expect(tokens.length).toBe(2);
      expect(tokens[0].lemma).toBe("DOG");
      expect(tokens[0].entries.length).toBe(1); // "DOG" should match "dog" with insensitivity
      expect(tokens[0].entries[0].entry.term).toBe("dog");
      expect(tokens[1].lemma).toBe("cat");
      expect(tokens[1].entries[0].entry.term).toBe("cat");
    },
  );

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
