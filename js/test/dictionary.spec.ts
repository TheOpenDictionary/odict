import { stat } from "node:fs/promises";
import { basename, dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import { Dictionary } from "../src";

describe("Dictionary", () => {
  let dict1: Dictionary;

  beforeAll(async () => {
    dict1 = await Dictionary.compile(
      join(
        fileURLToPath(new URL(import.meta.url)),
        "../../../examples/example1.xml"
      )
    );

    const stats = await stat(dict1.path);

    expect(stats.isFile).toBeTruthy();
  });

  it("can compile dictionaries properly", async () => {
    const result = await dict1.lookup("run");
    expect(result[0][0].term).toBe("run");
  });

  it("can index and search a dictionary", async () => {
    await dict1.index();
    const results = await dict1.search("run");
    expect(results[0][0].term).toBe("run");
  });
});
