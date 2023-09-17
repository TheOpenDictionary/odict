import { afterAll, beforeAll, describe, expect, it } from "@jest/globals";
import findProcess from "find-process";

import { existsSync } from "node:fs";
import { rm, stat } from "node:fs/promises";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

import { Dictionary } from "../src";
import { teardownServices } from "../src/service";

describe("Dictionary", () => {
  let dict1: Dictionary;

  afterAll(() => {
    teardownServices();
  });

  beforeAll(async () => {
    dict1 = await Dictionary.compile(
      join(
        fileURLToPath(new URL(import.meta.url)),
        "../../../examples/example1.xml",
      ),
    );

    const stats = await stat(dict1.path);

    expect(stats.isFile).toBeTruthy();
  });

  it("restarts if the process was killed", async () => {
    const result1 = await dict1.lookup({
      word: "dog",
      fallback: "dog",
    });

    expect(result1).toMatchSnapshot();

    const processes = await findProcess("name", "odict", true);

    processes.forEach((p) => process.kill(p.pid));

    await new Promise((r) => setTimeout(r, 0)); // Not sure why this is needed...

    const result2 = await dict1.lookup({
      word: "run",
      fallback: "run",
    });

    expect(result2).toMatchSnapshot();
  });

  it("can lookup terms properly", async () => {
    const result = await dict1.lookup({ word: "cat", fallback: "cat" });
    expect(result).toMatchSnapshot();
  });

  it("doesn't split unless specified", async () => {
    const result = await dict1.lookup("catdog");
    expect(result[0].length).toBe(0);
  });

  it("can return the lexicon", async () => {
    const result = await dict1.lexicon();
    expect(result).toStrictEqual(["cat", "dog", "poo", "ran", "run"]);
  });

  it("can write raw XML", async () => {
    await Dictionary.write(
      '<dictionary><entry term="hello"><ety><sense pos="v"><definition>hello world</definition></sense></ety></entry><entry term="world"><ety><sense pos="v"><definition>hello world</definition></sense></ety></entry></dictionary>"    )',
      "test.odict",
    );

    expect(existsSync("test.odict")).toBeTruthy();

    await rm("test.odict");
  });

  it("can split terms during lookup", async () => {
    const result = await dict1.lookup("catdog", { split: 3 });
    expect(result).toMatchSnapshot();
  });

  it("can split terms properly", async () => {
    const result = await dict1.split("catdog", 2);
    expect(result).toMatchSnapshot();
  });

  it("can index and search a dictionary", async () => {
    await dict1.index();

    const results = await dict1.search("run");

    expect(results).toMatchSnapshot();
  });
});
