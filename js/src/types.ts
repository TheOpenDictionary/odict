export type Query = { word: string; fallback: string } | string;

export interface DictionaryOptions {
  defaultSplitThreshold: number;
}

export interface LookupOptions {
  queries: Query[];
  split?: number;
}
