export type Query = { word: string; fallback: string } | string;

export interface DictionaryOptions {
  defaultSplitThreshold?: number;
}

export interface LookupOptions {
  split?: number;
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

export interface Usage {
  id?: string;
  pos: string;
  description?: string;
  definitions: string[];
  groups: Group[];
}

export interface Group {
  id?: string;
  description: string;
  definitions: string[];
}
