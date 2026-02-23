# ODict Python API (from rustdoc JSON)

*Generated from rustdoc JSON format v57*

## `CompressOptions`

Brotli compression options for saving dictionaries.

| Field | Type | Description |
|-------|------|-------------|
| `quality` | `?<u32>` | Compression quality level (0–11). |
| `window_size` | `?<u32>` | Compression window size (0–22). |

## `Definition`

A single definition of a word sense.

Contains the definition text along with optional examples and notes.

| Field | Type | Description |
|-------|------|-------------|
| `id` | `?<?>` | Optional identifier for this definition. |
| `value` | `?` | The definition text. |
| `examples` | `?<?>` | Usage examples illustrating this definition. |
| `notes` | `?<?>` | Additional notes about this definition. |

## `Entry`

A dictionary entry representing a single headword and its associated data.

Each entry contains the term itself, optional ranking metadata,
cross-reference information, etymologies, and media attachments.

| Field | Type | Description |
|-------|------|-------------|
| `term` | `?` | The headword for this entry. |
| `rank` | `?<u32>` | Optional frequency rank for ordering entries. |
| `see_also` | `?<?>` | Cross-reference target term, if this entry redirects to another. |
| `etymologies` | `?<?>` | The etymologies associated with this entry. |
| `media` | `?<?>` | Media URLs (audio, images, etc.) associated with this entry. |

## `EnumWrapper`

A wrapper for ODict enumeration values (e.g. part of speech, pronunciation kind).

ODict enums are represented as string triples: the enum name,
the variant name, and the variant's string value.

| Field | Type | Description |
|-------|------|-------------|
| `name` | `?` | The enum type name (e.g. `"PartOfSpeech"`). |
| `variant` | `?` | The variant name (e.g. `"Noun"`). |
| `value` | `?` | The string value of the variant (e.g. `"n"`). |

## `Etymology`

An etymology grouping for a dictionary entry.

Etymologies group together senses that share a common word origin.
Each etymology can have its own pronunciations and description.

| Field | Type | Description |
|-------|------|-------------|
| `id` | `?<?>` | Optional identifier for this etymology. |
| `pronunciations` | `?<?>` | Pronunciations associated with this etymology. |
| `description` | `?<?>` | Optional description of the word origin. |
| `senses` | `?<?>` | The senses (meanings) under this etymology. |

## `Example`

A usage example illustrating a definition.

Examples can optionally include translations and pronunciations.

| Field | Type | Description |
|-------|------|-------------|
| `value` | `?` | The example text. |
| `translations` | `?<?>` | Translations of this example into other languages. |
| `pronunciations` | `?<?>` | Pronunciations for this example. |

## `Form`

An inflected or alternate form of a word.

Forms represent morphological variants such as plurals, conjugations,
or other inflections.

| Field | Type | Description |
|-------|------|-------------|
| `term` | `?` | The inflected form text. |
| `kind` | `?<?>` | The kind of form (e.g. plural, past tense), or `None`. |
| `tags` | `?<?>` | Tags for categorizing this form. |

## `Group`

A named group of related definitions.

Groups allow organizing multiple definitions under a shared description,
such as grouping definitions by semantic domain.

| Field | Type | Description |
|-------|------|-------------|
| `id` | `?<?>` | Optional identifier for this group. |
| `description` | `?` | A description of what this group of definitions has in common. |
| `definitions` | `?<?>` | The definitions within this group. |

## `IndexOptions`

Options for configuring full-text index creation.

| Field | Type | Description |
|-------|------|-------------|
| `directory` | `?<?>` | Custom directory for storing the index. |
| `memory` | `?<usize>` | Memory arena size per thread in bytes (must be >15 MB). |
| `overwrite` | `?<bool>` | Whether to overwrite an existing index. |

## `LoadOptions`

Options for loading a dictionary from a file path, alias, or remote registry.

| Field | Type | Description |
|-------|------|-------------|
| `config_dir` | `?<?>` | Custom configuration directory. |
| `remote` | `?<?>` | Options for remote dictionary loading. |

## `LookupOptions`

Options for configuring term lookups.

| Field | Type | Description |
|-------|------|-------------|
| `split` | `?<u32>` | Minimum word length for compound splitting. |
| `follow` | `?<bool>` | Whether to follow `see_also` cross-references. |
| `insensitive` | `?<bool>` | Whether to enable case-insensitive matching. |

## `LookupResult`

The result of a dictionary lookup.

Contains the matched entry and, if a `see_also` redirect was followed,
the original entry that initiated the redirect.

| Field | Type | Description |
|-------|------|-------------|
| `entry` | `?` | The matched dictionary entry. |
| `directed_from` | `?<?>` | The original entry if a `see_also` redirect was followed, or `None`. |

## `MediaURL`

A reference to an external media resource (audio, image, etc.).

| Field | Type | Description |
|-------|------|-------------|
| `src` | `?` | URL or path to the media file. |
| `mime_type` | `?<?>` | MIME type (e.g. `audio/mpeg`), or `None`. |
| `description` | `?<?>` | Human-readable description of the media. |

## `Note`

An additional note attached to a definition.

Notes provide supplementary information such as usage guidance,
historical context, or grammatical remarks.

| Field | Type | Description |
|-------|------|-------------|
| `id` | `?<?>` | Optional identifier for this note. |
| `value` | `?` | The note text. |
| `examples` | `?<?>` | Examples associated with this note. |

## `OpenDictionary`

The main class for working with compiled ODict dictionaries.

An `OpenDictionary` wraps a compiled binary dictionary and provides
methods for looking up terms, full-text search, tokenization, and more.

# Construction

Create from compiled bytes or an XML string using [`OpenDictionary::new`],
or load from a file path or remote registry using [`OpenDictionary::load`].

## `Pronunciation`

A pronunciation entry for a word or etymology.

Represents how a word is pronounced in a given notation system
(e.g. IPA, Pinyin), with optional audio media.

| Field | Type | Description |
|-------|------|-------------|
| `kind` | `?<?>` | The pronunciation system (e.g. IPA, Pinyin), or `None`. |
| `value` | `?` | The pronunciation notation string. |
| `media` | `?<?>` | Audio media URLs for this pronunciation. |

## `RemoteLoadOptions`

Options for loading dictionaries from remote registries.

| Field | Type | Description |
|-------|------|-------------|
| `out_dir` | `?<?>` | Custom output directory for downloaded files. |
| `caching` | `?<bool>` | Whether to cache downloaded dictionaries locally. |
| `retries` | `?<u32>` | Number of download retries on failure. |

## `SaveOptions`

Options for saving a dictionary to disk.

| Field | Type | Description |
|-------|------|-------------|
| `compress` | `?<?>` | Optional Brotli compression settings. |

## `SearchOptions`

Options for configuring full-text search.

| Field | Type | Description |
|-------|------|-------------|
| `directory` | `?<?>` | Custom directory for the search index. |
| `threshold` | `?<u32>` | Relevance score threshold for filtering results. |
| `autoindex` | `?<bool>` | Whether to automatically create an index if one does not exist. |
| `limit` | `?<usize>` | Maximum number of results to return. |

## `Sense`

A word sense — a specific meaning grouped by part of speech.

Senses represent distinct meanings of a word under a given etymology.
Each sense has a part of speech and contains definitions (or definition groups),
along with optional tags, translations, and inflected forms.

| Field | Type | Description |
|-------|------|-------------|
| `pos` | `?` | The part of speech for this sense (e.g. noun, verb, adjective). |
| `lemma` | `?<?>` | Optional lemma reference linking to another entry. |
| `definitions` | `?<?<?, ?>>` | Definitions or definition groups under this sense. |
| `tags` | `?<?>` | Tags for categorizing or filtering this sense. |
| `translations` | `?<?>` | Translations of this sense into other languages. |
| `forms` | `?<?>` | Inflected forms of the word under this sense. |

## `Token`

A token produced by NLP-based text segmentation.

Each token represents a segment of the input text, with metadata about
its position, detected language and script, and any matching dictionary entries.

| Field | Type | Description |
|-------|------|-------------|
| `lemma` | `?` | The original token text (lemma form). |
| `language` | `?<?>` | Detected language code (e.g. `"eng"`), or `None` if unknown. |
| `entries` | `?<?>` | Matched dictionary entries for this token. |
| `kind` | `?` | The token kind (e.g. `"Word"`, `"Punctuation"`). |
| `script` | `?` | Detected script name (e.g. `"Latin"`, `"Han"`). |
| `start` | `usize` | Start byte offset in the original text. |
| `end` | `usize` | End byte offset in the original text. |

## `TokenizeOptions`

Options for configuring text tokenization.

| Field | Type | Description |
|-------|------|-------------|
| `follow` | `?<?<bool, u32>>` | Whether to follow `see_also` cross-references. Accepts `True`/`False` or a number (nonzero = follow). |
| `insensitive` | `?<bool>` | Whether to enable case-insensitive matching. |

## `Translation`

A translation of a word, definition, or example into another language.

| Field | Type | Description |
|-------|------|-------------|
| `lang` | `?` | The BCP-47 language code (e.g. `"fra"`, `"deu"`). |
| `value` | `?` | The translated text. |
