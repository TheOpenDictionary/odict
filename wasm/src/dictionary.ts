import { getService } from "./service";
import type { Entry, LookupOptions, Query } from "./types";

export class Dictionary {
  private constructor(
    private readonly name: string,
    private readonly service: any
  ) {}

  static async load(name: string, data: ArrayBuffer) {
    const service = await getService();
    await service.load(name, data);
    return new Dictionary(name, service);
  }

  static async compile(xml: string): Promise<Uint8Array> {
    const service = await getService();
    return service.compile(xml);
  }

  lookup(
    words: Query | Query[],
    { split, follow }: LookupOptions = {}
  ): Entry[][] {
    const queries = Array.isArray(words) ? words : [words];

    return JSON.parse(
      this.service.lookup(
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

  lexicon(): string[] {
    return JSON.parse(this.service.lexicon(this.name));
  }

  split(query: string, threshold: number): Entry[] {
    return JSON.parse(this.service.split(this.name, query, threshold));
  }
}
