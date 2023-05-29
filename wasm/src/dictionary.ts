import { getService } from "./service";
import type { Entry, LookupOptions, Query, WordWithFallback } from "./types";

export class Dictionary {
  private constructor(
    private readonly name: string,
    private readonly service: any
  ) {}

  static async load(name: string, data: ArrayBuffer) {
    const service = await getService();
    await service.loadDictionary(name, data);
    return new Dictionary(name, service);
  }

  static async compile(xml: string) {
    const service = await getService();
    return service.compileXML(xml);
  }

  lookup(
    words: Query | Query[],
    { split, follow }: LookupOptions = {}
  ): Entry[][] {
    const queries = Array.isArray(words) ? words : [words];

    return JSON.parse(
      this.service.lookupWord(
        this.name,
        queries.map((query) => {
          return typeof query === "string"
            ? query
            : `${query.word} (${query.fallback})`;
        }),
        split ?? 0,
        follow ?? false
      )
    );
  }

  lexicon() {
    return JSON.parse(this.service.getLexicon(this.name));
  }
}
