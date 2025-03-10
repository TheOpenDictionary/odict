/* auto-generated by NAPI-RS */
/* eslint-disable */
export declare class Dictionary {
  constructor(
    pathOrAlias: string,
    options?: DictionaryOptions | undefined | null,
  );
  static write(
    xmlStr: string,
    outPath: string,
    options?: DictionaryOptions | undefined | null,
  ): Dictionary;
  static compile(
    xmlPath: string,
    outPath?: string | undefined | null,
    options?: DictionaryOptions | undefined | null,
  ): Dictionary;
  get path(): string;
  lookup(
    query: LookupQuery | string | Array<LookupQuery | string>,
    options?: LookupOptions | undefined | null,
  ): Array<Array<Entry>>;
  lexicon(): Array<string>;
  split(query: string, options?: SplitOptions | undefined | null): Array<Entry>;
  index(options?: IndexOptions | undefined | null): void;
  search(
    query: string,
    options?: SearchOptions | undefined | null,
  ): Array<Entry>;
}

export interface Definition {
  id?: string;
  value: string;
  examples: Array<Example>;
  notes: Array<Note>;
}

export interface DictionaryOptions {
  split?: SplitOptions;
  index?: IndexOptions;
  search?: SearchOptions;
}

export interface Entry {
  term: string;
  seeAlso?: string;
  etymologies: Array<Etymology>;
}

export interface Etymology {
  id?: string;
  pronunciation?: string;
  description?: string;
  senses: Record<string, Sense>;
}

export interface Example {
  value: string;
}

export interface Group {
  id?: string;
  description: string;
  definitions: Array<Definition>;
}

export interface IndexOptions {
  directory?: string;
  memory?: number;
  overwrite?: boolean;
}

export interface LookupOptions {
  split?: number;
  follow?: boolean;
}

export interface LookupQuery {
  term: string;
  fallback: string;
}

export interface Note {
  id?: string;
  value: string;
  examples: Array<Example>;
}

export interface SearchOptions {
  directory?: string;
  threshold?: number;
  autoindex?: boolean;
  limit?: number;
}

export interface Sense {
  pos: string;
  definitions: Array<Definition | Group>;
}

export interface SplitOptions {
  threshold?: number;
}
