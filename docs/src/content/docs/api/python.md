---
title: Python API
description: Using ODict from Python via the theopendictionary package.
---

The Python bindings are distributed as the `theopendictionary` package on PyPI. They are native extensions built with [PyO3](https://pyo3.rs/).

## Installation

```bash
pip install theopendictionary
```

Requires Python 3.8.1+.

## Quick example

```python
from theopendictionary import OpenDictionary, compile

# Compile XML to bytes
xml = """
<dictionary name="My Dictionary">
  <entry term="hello">
    <ety>
      <sense pos="intj">
        <definition value="A greeting">
          <example value="Hello, world!" />
        </definition>
      </sense>
    </ety>
  </entry>
</dictionary>
"""

compiled_bytes = compile(xml)
dictionary = OpenDictionary(compiled_bytes)

results = dictionary.lookup("hello")
print(results[0].entry.term)        # "hello"
print(results[0].entry.etymologies)  # [Etymology(...)]
```

---

## Functions

### `compile(xml: str) -> bytes`

Compiles an ODXML string into binary `.odict` data (as a `bytes` object). This data can be passed to `OpenDictionary()` or saved to disk.

```python
from theopendictionary import compile

data = compile("<dictionary><entry term='hi'><ety><sense><definition value='greeting'/></sense></ety></entry></dictionary>")
```

---

## `OpenDictionary`

The main class for working with compiled dictionaries.

### Constructors

#### `OpenDictionary(data: bytes)`

Creates a dictionary from compiled binary data (as returned by `compile()`).

```python
from theopendictionary import OpenDictionary, compile

data = compile(xml_string)
dictionary = OpenDictionary(data)
```

#### `await OpenDictionary.load(dictionary: str, alias_path: str | None = None) -> OpenDictionary`

Loads a dictionary from a file path, alias, or remote identifier. This is an **async** method.

- If `dictionary` is a path to a `.odict` file, it loads from disk.
- If it matches the format `org/lang` (e.g. `wiktionary/eng`), it downloads from the remote registry.
- `alias_path` optionally specifies a custom alias file path.

```python
import asyncio
from theopendictionary import OpenDictionary

async def main():
    # Load from file
    dictionary = await OpenDictionary.load("./my-dictionary.odict")

    # Load from remote registry
    dictionary = await OpenDictionary.load("wiktionary/eng")

asyncio.run(main())
```

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `min_rank` | `int \| None` | The minimum rank value across all entries, or `None` if no entries have ranks |
| `max_rank` | `int \| None` | The maximum rank value across all entries, or `None` if no entries have ranks |

### Methods

#### `save(path: str, quality: int | None = None, window_size: int | None = None) -> None`

Saves the dictionary to disk as a `.odict` file. Optionally configure Brotli compression.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `path` | `str` | — | Output file path |
| `quality` | `int \| None` | `None` | Brotli compression level (0–11) |
| `window_size` | `int \| None` | `None` | Brotli window size (0–22) |

```python
dictionary.save("output.odict")
dictionary.save("output.odict", quality=11, window_size=22)
```

#### `lookup(query, split=None, follow=None, insensitive=None) -> list[LookupResult]`

Looks up one or more terms by exact match.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `query` | `str \| list[str]` | — | Term(s) to look up |
| `split` | `int \| None` | `None` | Minimum word length for compound splitting |
| `follow` | `bool \| int \| None` | `None` | Follow `see` cross-references. `True` = infinite, `False` = disabled, `int` = max depth |
| `insensitive` | `bool \| None` | `None` | Enable case-insensitive matching |

```python
# Simple lookup
results = dictionary.lookup("cat")

# Multiple terms
results = dictionary.lookup(["cat", "dog"])

# Follow cross-references, case-insensitive
results = dictionary.lookup("RaN", follow=True, insensitive=True)
# results[0].entry.term == "run"
# results[0].directed_from.term == "ran"

# Compound word splitting
results = dictionary.lookup("catdog", split=3)
```

#### `lexicon() -> list[str]`

Returns all terms defined in the dictionary, sorted alphabetically.

```python
words = dictionary.lexicon()
# ["cat", "dog", "run", ...]
```

#### `index(options=None) -> None`

Creates a full-text search index for the dictionary.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `options` | `IndexOptions \| None` | `None` | Indexing configuration |

```python
from theopendictionary import IndexOptions

dictionary.index()
dictionary.index(IndexOptions(overwrite=True, memory=50_000_000))
```

#### `search(query: str, options=None) -> list[Entry]`

Runs a full-text search across the dictionary. Requires an index (call `index()` first).

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `query` | `str` | — | Search query |
| `options` | `SearchOptions \| None` | `None` | Search configuration |

```python
from theopendictionary import SearchOptions

dictionary.index()
results = dictionary.search("domesticated mammal")
results = dictionary.search("greeting", SearchOptions(limit=5))
```

#### `tokenize(text: str, follow=None, insensitive=None) -> list[Token]`

Tokenizes text using NLP-based segmentation and matches each token against the dictionary. Supports Chinese, Japanese, Korean, Thai, Khmer, German, Swedish, and Latin-script languages.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `text` | `str` | — | Text to tokenize |
| `follow` | `bool \| int \| None` | `None` | Follow `see` cross-references |
| `insensitive` | `bool \| None` | `None` | Case-insensitive matching |

```python
tokens = dictionary.tokenize("the cat ran")
for token in tokens:
    print(token.lemma, token.entries)
```

---

## Types

### `LookupResult`

| Property | Type | Description |
|----------|------|-------------|
| `entry` | `Entry` | The matched entry |
| `directed_from` | `Entry \| None` | The original entry if a `see` redirect was followed |

### `Entry`

| Property | Type | Description |
|----------|------|-------------|
| `term` | `str` | The headword |
| `rank` | `int \| None` | Optional frequency rank |
| `see_also` | `str \| None` | Cross-reference target term |
| `etymologies` | `list[Etymology]` | List of etymologies |
| `media` | `list[MediaURL]` | Media URLs |

### `Token`

| Property | Type | Description |
|----------|------|-------------|
| `lemma` | `str` | The original token text |
| `language` | `str \| None` | Detected language code |
| `script` | `str` | Detected script name |
| `kind` | `str` | Token kind |
| `start` | `int` | Start offset in the original text |
| `end` | `int` | End offset in the original text |
| `entries` | `list[LookupResult]` | Matched dictionary entries |

### `IndexOptions`

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `directory` | `str \| None` | `None` | Custom directory for the index |
| `memory` | `int \| None` | `None` | Memory arena per thread in bytes (must be >15MB) |
| `overwrite` | `bool \| None` | `None` | Overwrite existing index |

### `SearchOptions`

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `directory` | `str \| None` | `None` | Custom index directory |
| `threshold` | `int \| None` | `None` | Relevance threshold |
| `autoindex` | `bool \| None` | `None` | Auto-create index if missing |
| `limit` | `int \| None` | `None` | Maximum results |

### `Pronunciation`

| Property | Type | Description |
|----------|------|-------------|
| `kind` | `EnumWrapper \| None` | The pronunciation system (e.g. IPA, Pinyin) |
| `value` | `str` | The pronunciation notation |
| `media` | `list[MediaURL]` | Audio URLs |

### `MediaURL`

| Property | Type | Description |
|----------|------|-------------|
| `src` | `str` | URL or path to the media file |
| `mime_type` | `str \| None` | MIME type (e.g. `audio/mpeg`) |
| `description` | `str \| None` | Description of the media |
