import { POS } from "./__generated__";

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
  exact?: boolean;
}

export interface Entry {
  id?: string;
  term: string;
  pronunciation?: string;
  etymologies: Etymology[];
}

export interface Etymology {
  id?: string;
  description?: string;
  senses: Record<string, Sense>;
}

export interface DefinitionNote {
  id?: string;
  value?: string;
  examples?: string[];
}

export interface Definition {
  value?: string;
  examples?: string[];
  notes?: DefinitionNote[];
}

export interface Sense {
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

export type PartOfSpeech = keyof typeof POS;

export const PartOfSpeech = Object.entries(POS).reduce((acc, [key, value]) => {
  const t = POS[POS.det];

  if (typeof value === "number") {
    acc[key as PartOfSpeech] = key as PartOfSpeech;
  }
  return acc;
}, {} as Record<PartOfSpeech, PartOfSpeech>);
