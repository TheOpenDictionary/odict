# Python API

*Auto-generated from Rust doc comments.*

---

## Functions

### `compile()`

Compiles an ODXML string into binary `.odict` data.

Takes an XML string conforming to the ODict XML schema and returns
the compiled binary representation as a byte vector. The resulting
bytes can be passed to [`OpenDictionary::new`] or saved to disk.

# Errors

Returns an error if the XML is malformed or does not conform to the
ODict schema.

## `OpenDictionary`

The main class for working with compiled ODict dictionaries.

An `OpenDictionary` wraps a compiled binary dictionary and provides
methods for looking up terms, full-text search, tokenization, and more.

# Construction

Create from compiled bytes or an XML string using [`OpenDictionary::new`],
or load from a file path or remote registry using [`OpenDictionary::load`].

### Methods

#### `load()`

Loads a dictionary from a file path, alias, or remote identifier.

This is an async method. If `dictionary` is a path to a `.odict` file,
it loads from disk. If it matches the format `org/lang` (e.g. `wiktionary/eng`),
it downloads from the remote registry.

#### `new()`

Creates a dictionary from compiled binary data or directly from an XML string.

Accepts either `bytes` (as returned by [`compile`]) or a `str` containing
ODXML markup.

#### `save()`

Saves the dictionary to disk as a `.odict` file.

Optionally configure Brotli compression via `quality` (0–11) and
`window_size` (0–22).

#### `min_rank()`

The minimum rank value across all entries, or `None` if no entries have ranks.

#### `max_rank()`

The maximum rank value across all entries, or `None` if no entries have ranks.

#### `lookup()`

Looks up one or more terms by exact match.

- `query` — a single term or list of terms to look up.
- `split` — minimum word length for compound splitting.
- `follow` — follow `see_also` cross-references until an entry with etymologies is found.
- `insensitive` — enable case-insensitive matching.

#### `lexicon()`

Returns all terms defined in the dictionary, sorted alphabetically.

#### `index()`

Creates a full-text search index for the dictionary.

Must be called before [`OpenDictionary::search`].

#### `search()`

Runs a full-text search across the dictionary.

Requires an index — call [`OpenDictionary::index`] first.

#### `tokenize()`

Tokenizes text using NLP-based segmentation and matches each token against the dictionary.

Supports Chinese, Japanese, Korean, Thai, Khmer, German, Swedish,
and Latin-script languages.

- `text` — the text to tokenize.
- `follow` — follow `see_also` cross-references. Accepts `True`/`False` or a number (nonzero = follow).
- `insensitive` — enable case-insensitive matching.

---

## `Definition`

A single definition of a word sense.

Contains the definition text along with optional examples and notes.

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `id` | `str | None` | Optional identifier for this definition. |
| `value` | `str` | The definition text. |
| `examples` | `list[Example]` | Usage examples illustrating this definition. |
| `notes` | `list[Note]` | Additional notes about this definition. |

---

## `Entry`

A dictionary entry representing a single headword and its associated data.

Each entry contains the term itself, optional ranking metadata,
cross-reference information, etymologies, and media attachments.

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `term` | `str` | The headword for this entry. |
| `rank` | `int | None` | Optional frequency rank for ordering entries. |
| `see_also` | `str | None` | Cross-reference target term, if this entry redirects to another. |
| `etymologies` | `list[Etymology]` | The etymologies associated with this entry. |
| `media` | `list[MediaURL]` | Media URLs (audio, images, etc.) associated with this entry. |

---

## `EnumWrapper`

A wrapper for ODict enumeration values (e.g. part of speech, pronunciation kind).

ODict enums are represented as string triples: the enum name,
the variant name, and the variant's string value.

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `name` | `str` | The enum type name (e.g. `"PartOfSpeech"`). |
| `variant` | `str` | The variant name (e.g. `"Noun"`). |
| `value` | `str` | The string value of the variant (e.g. `"n"`). |

---

## `Etymology`

An etymology grouping for a dictionary entry.

Etymologies group together senses that share a common word origin.
Each etymology can have its own pronunciations and description.

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `id` | `str | None` | Optional identifier for this etymology. |
| `pronunciations` | `list[Pronunciation]` | Pronunciations associated with this etymology. |
| `description` | `str | None` | Optional description of the word origin. |
| `senses` | `list[Sense]` | The senses (meanings) under this etymology. |

---

## `Example`

A usage example illustrating a definition.

Examples can optionally include translations and pronunciations.

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `value` | `str` | The example text. |
| `translations` | `list[Translation]` | Translations of this example into other languages. |
| `pronunciations` | `list[Pronunciation]` | Pronunciations for this example. |

---

## `Form`

An inflected or alternate form of a word.

Forms represent morphological variants such as plurals, conjugations,
or other inflections.

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `term` | `str` | The inflected form text. |
| `kind` | `EnumWrapper | None` | The kind of form (e.g. plural, past tense), or `None`. |
| `tags` | `list[str]` | Tags for categorizing this form. |

---

## `Group`

A named group of related definitions.

Groups allow organizing multiple definitions under a shared description,
such as grouping definitions by semantic domain.

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `id` | `str | None` | Optional identifier for this group. |
| `description` | `str` | A description of what this group of definitions has in common. |
| `definitions` | `list[Definition]` | The definitions within this group. |

---

## `IndexOptions`

Options for configuring full-text index creation.

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `directory` | `str | None` | Custom directory for storing the index. |
| `memory` | `int | None` | Memory arena size per thread in bytes (must be >15 MB). |
| `overwrite` | `bool | None` | Whether to overwrite an existing index. |

---

## `RemoteLoadOptions`

Options for loading dictionaries from remote registries.

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `out_dir` | `str | None` | Custom output directory for downloaded files. |
| `caching` | `bool | None` | Whether to cache downloaded dictionaries locally. |
| `retries` | `int | None` | Number of download retries on failure. |

---

## `LoadOptions`

Options for loading a dictionary from a file path, alias, or remote registry.

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `config_dir` | `str | None` | Custom configuration directory. |
| `remote` | `RemoteLoadOptions | None` | Options for remote dictionary loading. |

---

## `LookupOptions`

Options for configuring term lookups.

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `split` | `int | None` | Minimum word length for compound splitting. |
| `follow` | `bool | None` | Whether to follow `see_also` cross-references. |
| `insensitive` | `bool | None` | Whether to enable case-insensitive matching. |

---

## `LookupResult`

The result of a dictionary lookup.

Contains the matched entry and, if a `see_also` redirect was followed,
the original entry that initiated the redirect.

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `entry` | `Entry` | The matched dictionary entry. |
| `directed_from` | `Entry | None` | The original entry if a `see_also` redirect was followed, or `None`. |

---

## `MediaURL`

A reference to an external media resource (audio, image, etc.).

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `src` | `str` | URL or path to the media file. |
| `mime_type` | `str | None` | MIME type (e.g. `audio/mpeg`), or `None`. |
| `description` | `str | None` | Human-readable description of the media. |

---

## `Note`

An additional note attached to a definition.

Notes provide supplementary information such as usage guidance,
historical context, or grammatical remarks.

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `id` | `str | None` | Optional identifier for this note. |
| `value` | `str` | The note text. |
| `examples` | `list[Example]` | Examples associated with this note. |

---

## `Pronunciation`

A pronunciation entry for a word or etymology.

Represents how a word is pronounced in a given notation system
(e.g. IPA, Pinyin), with optional audio media.

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `kind` | `EnumWrapper | None` | The pronunciation system (e.g. IPA, Pinyin), or `None`. |
| `value` | `str` | The pronunciation notation string. |
| `media` | `list[MediaURL]` | Audio media URLs for this pronunciation. |

---

## `CompressOptions`

Brotli compression options for saving dictionaries.

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `quality` | `int | None` | Compression quality level (0–11). |
| `window_size` | `int | None` | Compression window size (0–22). |

---

## `SaveOptions`

Options for saving a dictionary to disk.

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `compress` | `CompressOptions | None` | Optional Brotli compression settings. |

---

## `SearchOptions`

Options for configuring full-text search.

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `directory` | `str | None` | Custom directory for the search index. |
| `threshold` | `int | None` | Relevance score threshold for filtering results. |
| `autoindex` | `bool | None` | Whether to automatically create an index if one does not exist. |
| `limit` | `int | None` | Maximum number of results to return. |

---

## `Sense`

A word sense — a specific meaning grouped by part of speech.

Senses represent distinct meanings of a word under a given etymology.
Each sense has a part of speech and contains definitions (or definition groups),
along with optional tags, translations, and inflected forms.

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `pos` | `EnumWrapper` | The part of speech for this sense (e.g. noun, verb, adjective). |
| `lemma` | `str | None` | Optional lemma reference linking to another entry. |
| `definitions` | `list[Definition | Group]` | Definitions or definition groups under this sense. |
| `tags` | `list[str]` | Tags for categorizing or filtering this sense. |
| `translations` | `list[Translation]` | Translations of this sense into other languages. |
| `forms` | `list[Form]` | Inflected forms of the word under this sense. |

---

## `Token`

A token produced by NLP-based text segmentation.

Each token represents a segment of the input text, with metadata about
its position, detected language and script, and any matching dictionary entries.

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `lemma` | `str` | The original token text (lemma form). |
| `language` | `str | None` | Detected language code (e.g. `"eng"`), or `None` if unknown. |
| `entries` | `list[LookupResult]` | Matched dictionary entries for this token. |
| `kind` | `str` | The token kind (e.g. `"Word"`, `"Punctuation"`). |
| `script` | `str` | Detected script name (e.g. `"Latin"`, `"Han"`). |
| `start` | `int` | Start byte offset in the original text. |
| `end` | `int` | End byte offset in the original text. |

---

## `TokenizeOptions`

Options for configuring text tokenization.

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `follow` | `bool | int | None` | Whether to follow `see_also` cross-references. Accepts `True`/`False` or a number (nonzero = follow). |
| `insensitive` | `bool | None` | Whether to enable case-insensitive matching. |

---

## `Translation`

A translation of a word, definition, or example into another language.

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `lang` | `str` | The BCP-47 language code (e.g. `"fra"`, `"deu"`). |
| `value` | `str` | The translated text. |

---
