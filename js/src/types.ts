export type Query = { word: string; fallback: string } | string;

export interface DictionaryOptions {
  defaultSplitThreshold?: number;
}

export interface LookupOptions {
  split?: number;
  follow?: boolean;
}

export interface SearchOptions {
  force?: boolean;
}

export interface Entry {
  id?: string;
  term: string;
  etymologies: Etymology[];
}

export interface Etymology {
  id?: string;
  description?: string;
  usages: Record<string, Usage>;
}

export interface Definition {
  value: string;
  examples: string[];
}

export interface Usage {
  id?: string;
  pos: string;
  description?: string;
  definitions: Definition[];
  groups: Group[];
}

export interface Group {
  id?: string;
  description: string;
  definitions: Definition[];
}
