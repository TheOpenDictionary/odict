---
title: JavaScript API
description: Using ODict from JavaScript/TypeScript via the @odict/node package.
---

The JavaScript bindings are distributed as `@odict/node` on npm. They are native extensions built with [NAPI-RS](https://napi.rs/) and also support the browser via WASI.

## Installation

```bash
npm install @odict/node
```

Requires Node.js 12+. Native binaries are included for all major platforms (macOS, Linux, Windows, ARM64, WASI).

## Quick example

```typescript
import { readFile } from "node:fs/promises";
import { compile, OpenDictionary } from "@odict/node";

// Compile XML to a buffer
const xml = await readFile("my-dictionary.xml", "utf-8");
const data = compile(xml);
const dictionary = new OpenDictionary(data);

const results = dictionary.lookup("hello");
console.log(results[0].entry.term); // "hello"
```

---

## Functions

### `compile(xml: string): Buffer`

Compiles an ODXML string into binary `.odict` data. Returns a `Buffer` that can be passed to `new OpenDictionary()`.

```typescript
import { compile } from "@odict/node";

const data = compile(`
  <dictionary>
    <entry term="hi">
      <ety><sense><definition value="greeting"/></sense></ety>
    </entry>
  </dictionary>
`);
```

---

## `OpenDictionary`

The main class for working with compiled dictionaries.

### Constructors

#### `new OpenDictionary(data: Buffer)`

Creates a dictionary from compiled binary data (as returned by `compile()`).

```typescript
import { compile, OpenDictionary } from "@odict/node";

const data = compile(xmlString);
const dictionary = new OpenDictionary(data);
```

#### `OpenDictionary.load(dictionary: string, options?: LoadOptions): Promise<OpenDictionary>`

Loads a dictionary from a file path or remote identifier. Returns a `Promise`.

- If `dictionary` is a path to a `.odict` file, it loads from disk.
- If it matches the format `org/lang` (e.g. `wiktionary/eng`), it downloads from the remote registry.

```typescript
import { OpenDictionary } from "@odict/node";

// Load from file
const dictionary = await OpenDictionary.load("./my-dictionary.odict");

// Load from remote registry
const dictionary = await OpenDictionary.load("wiktionary/eng");

// Load with alias options
const dictionary = await OpenDictionary.load("./dict.odict", {
  alias: { path: "./aliases.json" },
});
```

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `minRank` | `number \| null` | The minimum rank value across all entries, or `null` if no entries have ranks |
| `maxRank` | `number \| null` | The maximum rank value across all entries, or `null` if no entries have ranks |

### Methods

#### `save(path: string, options?: SaveOptions): void`

Saves the dictionary to disk as a `.odict` file.

```typescript
dictionary.save("output.odict");
dictionary.save("output.odict", {
  compress: { quality: 11, windowSize: 22 },
});
```

#### `lookup(query: string | string[], options?: LookupOptions): LookupResult[]`

Looks up one or more terms by exact match.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `query` | `string \| string[]` | — | Term(s) to look up |
| `options.split` | `number` | — | Minimum word length for compound splitting |
| `options.follow` | `boolean \| number` | — | Follow `see` cross-references. `true` = infinite, `false` = disabled, number = max depth |
| `options.insensitive` | `boolean` | — | Enable case-insensitive matching |

```typescript
// Simple lookup
const results = dictionary.lookup("cat");

// Multiple terms
const results = dictionary.lookup(["cat", "dog"]);

// Follow cross-references, case-insensitive
const results = dictionary.lookup("RaN", {
  follow: true,
  insensitive: true,
});
// results[0].entry.term === "run"
// results[0].directedFrom?.term === "ran"

// Compound word splitting
const results = dictionary.lookup("catdog", { split: 3 });
```

#### `lexicon(): string[]`

Returns all terms defined in the dictionary, sorted alphabetically.

```typescript
const words = dictionary.lexicon();
// ["cat", "dog", "run", ...]
```

#### `index(options?: IndexOptions): void`

Creates a full-text search index for the dictionary.

```typescript
dictionary.index();
dictionary.index({ overwrite: true, memory: 50_000_000 });
```

#### `search(query: string, options?: SearchOptions): Entry[]`

Runs a full-text search. Requires an index (call `index()` first).

```typescript
dictionary.index();

const results = dictionary.search("domesticated mammal");
const results = dictionary.search("greeting", { limit: 5 });
```

#### `tokenize(text: string, options?: TokenizeOptions): Token[]`

Tokenizes text and matches each token against the dictionary. Supports Chinese, Japanese, Korean, Thai, Khmer, German, Swedish, and Latin-script languages.

```typescript
const tokens = dictionary.tokenize("the cat ran");
for (const token of tokens) {
  console.log(token.lemma, token.entries);
}

// With options
const tokens = dictionary.tokenize("DOG cat", {
  insensitive: true,
  follow: true,
});
```

---

## Types

### `LookupResult`

```typescript
interface LookupResult {
  entry: Entry;
  directedFrom?: Entry;
}
```

### `Entry`

```typescript
interface Entry {
  term: string;
  rank?: number;
  seeAlso?: string;
  etymologies: Etymology[];
  media: MediaURL[];
}
```

### `Etymology`

```typescript
interface Etymology {
  id?: string;
  pronunciations: Pronunciation[];
  description?: string;
  senses: Record<string, Sense>;
}
```

### `Sense`

```typescript
interface Sense {
  pos: EnumWrapper;
  lemma?: string;
  definitions: Array<Definition | Group>;
  tags: string[];
  translations: Translation[];
  forms: Form[];
}
```

### `Definition`

```typescript
interface Definition {
  id?: string;
  value: string;
  examples: Example[];
  notes: Note[];
}
```

### `Group`

```typescript
interface Group {
  id?: string;
  description: string;
  definitions: Definition[];
}
```

### `Example`

```typescript
interface Example {
  value: string;
  translations: Translation[];
  pronunciations: Pronunciation[];
}
```

### `Note`

```typescript
interface Note {
  id?: string;
  value: string;
  examples: Example[];
}
```

### `Pronunciation`

```typescript
interface Pronunciation {
  kind?: EnumWrapper;
  value: string;
  media: MediaUrl[];
}
```

### `MediaUrl`

```typescript
interface MediaUrl {
  src: string;
  mimeType?: string;
  description?: string;
}
```

### `Token`

```typescript
interface Token {
  lemma: string;
  language?: string;
  entries: LookupResult[];
  kind: string;
  script: string;
  start: number;
  end: number;
}
```

### `EnumWrapper`

```typescript
interface EnumWrapper {
  name: string;
  variant: string;
  value: string;
}
```

### Options

```typescript
interface LoadOptions {
  alias?: AliasLoadOptions;
}

interface AliasLoadOptions {
  path?: string;
}

interface SaveOptions {
  compress?: CompressOptions;
}

interface CompressOptions {
  quality?: number;
  windowSize?: number;
}

interface LookupOptions {
  split?: number;
  follow?: boolean | number;
  insensitive?: boolean;
}

interface IndexOptions {
  directory?: string;
  memory?: number;
  overwrite?: boolean;
}

interface SearchOptions {
  directory?: string;
  threshold?: number;
  autoindex?: boolean;
  limit?: number;
}

interface TokenizeOptions {
  follow?: boolean | number;
  allowList?: string[];
  insensitive?: boolean;
}
```

## Browser support

The `@odict/node` package also supports browser environments via WASI. Import from the browser entry point:

```typescript
import { compile, OpenDictionary } from "@odict/node/browser";
```

:::note
Browser support runs ODict compiled to WebAssembly via WASI. The `load()` method (which accesses the filesystem and network) is not available in the browser — use `new OpenDictionary(data)` with pre-compiled data instead.
:::
