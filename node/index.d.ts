/* auto-generated by NAPI-RS */
/* eslint-disable */
export declare class Dictionary {
  constructor(data: Buffer, options?: DictionaryOptions | undefined | null);
  lookup(
    query: string | Array<string>,
    options?: LookupOptions | undefined | null,
  ): Array<LookupResult>;
  lexicon(): Array<string>;
  index(options?: IndexOptions | undefined | null): void;
  search(
    query: string,
    options?: SearchOptions | undefined | null,
  ): Array<Entry>;
  tokenize(
    text: string,
    options?: TokenizeOptions | undefined | null,
  ): Array<Token>;
}

export declare function compile(xml: string): Buffer;

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
  rank?: number;
  seeAlso?: string;
  etymologies: Array<Etymology>;
  media: Array<MediaURL>;
}

export interface EnumWrapper {
  name: string;
  variant: string;
  value: string;
}

export interface Etymology {
  id?: string;
  pronunciations: Array<Pronunciation>;
  description?: string;
  senses: Record<string, Sense>;
}

export interface Example {
  value: string;
  translations: Array<Translation>;
  pronunciations: Array<Pronunciation>;
}

export interface Form {
  term: string;
  kind?: EnumWrapper;
  tags: Array<string>;
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
  insensitive?: boolean;
}

export interface LookupResult {
  entry: Entry;
  directedFrom?: Entry;
}

export interface MediaUrl {
  src: string;
  mimeType?: string;
  description?: string;
}

export interface Note {
  id?: string;
  value: string;
  examples: Array<Example>;
}

export interface Pronunciation {
  kind?: EnumWrapper;
  value: string;
  media: Array<MediaUrl>;
}

export interface SearchOptions {
  directory?: string;
  threshold?: number;
  autoindex?: boolean;
  limit?: number;
}

export interface Sense {
  pos: EnumWrapper;
  lemma?: string;
  definitions: Array<Definition | Group>;
  tags: Array<string>;
  translations: Array<Translation>;
  forms: Array<Form>;
}

export interface SplitOptions {
  minLength?: number;
}

export interface Token {
  lemma: string;
  language?: string;
  entries: Array<LookupResult>;
  kind: string;
  script: string;
  start: number;
  end: number;
}

export interface TokenizeOptions {
  follow?: boolean;
  allowList?: Array<string>;
  insensitive?: boolean;
}

export interface Translation {
  lang: string;
  value: string;
}
