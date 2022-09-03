import { writeFile } from "node:fs/promises";

import { exec } from "./exec";
import { withTemporaryFile } from "./tmp";
import type { DictionaryOptions, LookupOptions } from "./types";

class Dictionary {
  private readonly options: DictionaryOptions;

  constructor(
    private readonly path: string,
    options: Partial<DictionaryOptions>
  ) {
    this.options = {
      defaultSplitThreshold: 2,
      ...options,
    };
  }

  static async compile(xmlPath: string, outPath?: string) {
    const commands = ["compile"];

    if (outPath) {
      commands.concat(["-o", outPath]);
    }

    commands.push(xmlPath);

    await exec(...commands);
  }

  static async write(xml: string, outPath: string) {
    return withTemporaryFile(async (tmp) => {
      await writeFile(tmp, xml, "utf-8");
      await exec("compile", "-o", outPath, tmp);
    });
  }

  async lookup({
    queries,
    split = this.options.defaultSplitThreshold,
  }: LookupOptions) {
    const results = await Promise.all(
      queries.map(async (query) => {
        const value =
          typeof query === "string"
            ? query
            : `${query.word} (${query.fallback})`;

        const raw = await exec(
          "lookup",
          "-s",
          split.toString(),
          this.path,
          value
        );

        return JSON.parse(Buffer.from(raw).toString("utf-8"));
      })
    );
  }
}

export { Dictionary };
