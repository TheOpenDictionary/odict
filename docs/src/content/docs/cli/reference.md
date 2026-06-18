---
title: CLI Reference
description: Complete reference for the ODict command-line interface.
---

```
odict [OPTIONS] <COMMAND>
```

The ODict CLI is the primary tool for creating, compiling, and querying ODict dictionaries.

## Global options

| Option | Description |
|--------|-------------|
| `-q, --quiet` | Silence any non-important output |
| `-h, --help` | Print help |
| `-V, --version` | Print version |

---

## Commands

### `compile`

Compiles a dictionary from ODXML.

```
odict compile <input> [-o <output>] [-q <quality>] [-w <window_size>]
```

#### Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `input` | Yes | Path to ODXML file |

#### Options

| Flag | Description |
|------|-------------|
| `-o` | Output path of compiled dictionary |
| `-q` | Brotli compression level (between 0 and 11) (default: `8`) |
| `-w` | Brotli large window size (between 0 and 22) (default: `22`) |

---

### `download`

Downloads a dictionary from the remote registry.

```
odict download <dictionary> [-o <output>] [--no-cache]
```

#### Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `dictionary` | Yes | Dictionary to download (e.g., 'wiktionary/eng') |

#### Options

| Flag | Description |
|------|-------------|
| `-o, --output` | Directory to download the dictionary to (defaults to config directory) |
| `--no-cache` | Disable caching (always download fresh copy) (default: `false`) |
| `-r, --retries` | Number of times to retry loading the dictionary (remote-only) (default: `3`) |

---

### `dump`

Outputs a dictionary in a human-readable format.

```
odict dump <input> [-f] [-o <output>]
```

#### Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `input` | Yes | Path to a compile dictionary |

#### Options

| Flag | Description |
|------|-------------|
| `-f` | Format in which to dump the dictionary. (default: `xml`) |
| `-o` | Output path of the dump. Defaults to stdout. |
| `-r, --retries` | Number of times to retry loading the dictionary (remote-only) (default: `3`) |

---

### `index`

Creates a full-text index of a compiled dictionary.

```
odict index <dictionary> [-d <directory>] [-f] [-m <memory>]
```

#### Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `dictionary` | Yes | Path to a compiled dictionary or an alias |

#### Options

| Flag | Description |
|------|-------------|
| `-d` | Custom directory to store the index |
| `-f` | Whether to overwrite the index if it already exists (default: `false`) |
| `-m` | Memory arena per thread in bytes. Must be above 15MB. (default: `15000000`) |
| `-r, --retries` | Number of times to retry loading the dictionary (remote-only) (default: `3`) |

---

### `info`

Prints the metadata info for a dictionary file.

```
odict info <dictionary_path>
```

#### Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `dictionary_path` | Yes | Path to a compiled dictionary |

#### Options

| Flag | Description |
|------|-------------|
| `-r, --retries` | Number of times to retry loading the dictionary (remote-only) (default: `3`) |

---

### `lexicon`

Lists all words defined in a dictionary.

```
odict lexicon <dictionary>
```

#### Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `dictionary` | Yes | Path to a compiled dictionary |

#### Options

| Flag | Description |
|------|-------------|
| `-r, --retries` | Number of times to retry loading the dictionary (remote-only) (default: `3`) |

---

### `lookup`

Looks up an entry in a compiled dictionary without indexing.

```
odict lookup <dictionary_path> <queries...> [-f <format>] [-F] [-s <split>] [-i]
```

#### Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `dictionary_path` | Yes | Path to a compiled dictionary |
| `queries` | Yes | Words to look up |

#### Options

| Flag | Description |
|------|-------------|
| `-f, --format` | Output format of the entries (default: `print`) |
| `-F, --follow` | Follow see_also redirects until finding an entry with etymologies |
| `-s, --split` | If a definition cannot be found, attempt to split the query into words of at least length S and look up each word separately. Can be relatively slow. (default: `0`) |
| `-i, --insensitive` | Perform case-insensitive lookups (default: `false`) |
| `-r, --retries` | Number of times to retry loading the dictionary (remote-only) (default: `3`) |

---

### `merge`

Merge entries from multiple dictionaries into a destination dictionary.

```
odict merge <destination> <sources...> [-o <output>]
```

#### Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `destination` | Yes | Path of the dictionary to merge into (unless --output is specified) |
| `sources` | Yes | Paths of dictionaries to merge |

#### Options

| Flag | Description |
|------|-------------|
| `-o, --output` | Separate output path for the compiled dictionary |
| `-r, --retries` | Number of times to retry loading the dictionary (remote-only) (default: `3`) |

---

### `new`

Scaffolds a new ODict XML dictionary.

```
odict new <file_name> [-n <name>]
```

#### Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `file_name` | Yes | Name of your new dictionary file |

#### Options

| Flag | Description |
|------|-------------|
| `-n` | Name attribute of the dictionary element |

---

### `search`

Run a full-text query on a compiled dictionary.

```
odict search <dictionary> <query> [-f] [--index]
```

#### Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `dictionary` | Yes | Path to a compiled dictionary or an alias |
| `query` | Yes | Search query |

#### Options

| Flag | Description |
|------|-------------|
| `-f, --format` | Format in which to print the results (default: `json`) |
| `--index` | Creates a new index if one doesn't already exist (default: `false`) |
| `-r, --retries` | Number of times to retry loading the dictionary (remote-only) (default: `3`) |

---

### `serve`

Start a local web server to serve one or several dictionaries.

```
odict serve [dictionaries...] [-p <port>] [-c <capacity>] [-l]
```

#### Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `dictionaries` | No | Dictionary files or directories containing `.odict` files |

#### Options

| Flag | Description |
|------|-------------|
| `-p` | Port to listen on (default: `5005`) |
| `-c, --capacity` | Maximum number of dictionaries to keep in memory (default: `5`) |
| `-l, --level` | Log level (`trace`, `debug`, `info`, `warn`, or `error`) |

#### HTTP endpoints

When running `serve`, the following REST endpoints become available. All return JSON.

##### `GET /{name}/lookup`

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `q` | string | Yes | Term or comma-separated terms to look up |
| `follow` | boolean | No | Follow `see` cross-references |
| `split` | number | No | Minimum character length for fallback splitting |

##### `GET /{name}/search`

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `q` | string | Yes | Full-text search query |
| `limit` | number | No | Maximum number of results |

##### `GET /{name}/split`

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `q` | string | Yes | Term or comma-separated terms to split |
| `follow` | boolean | No | Follow `see` cross-references |
| `min_length` | number | No | Minimum character length for each segment |

##### `GET /{name}/tokenize`

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `text` | string | Yes | Text to tokenize |
| `follow` | boolean | No | Follow `see` cross-references for matched tokens |

---

### `split`

Splits text into component dictionary words without attempting a whole-word lookup first.

```
odict split <dictionary_path> <queries...> [-f <format>] [-F] [-m <min_length>] [-i]
```

#### Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `dictionary_path` | Yes | Path to a compiled dictionary |
| `queries` | Yes | Text to split into dictionary words |

#### Options

| Flag | Description |
|------|-------------|
| `-f, --format` | Output format of the entries (default: `print`) |
| `-F, --follow` | Follow see_also redirects until finding an entry with etymologies |
| `-m, --min-length` | Minimum character length of each split segment (default: `1`) |
| `-i, --insensitive` | Perform case-insensitive lookups (default: `false`) |
| `-r, --retries` | Number of times to retry loading the dictionary (remote-only) (default: `3`) |

---

### `tokenize`

Tokenize text and find dictionary entries for each token.

```
odict tokenize <dictionary_path> <text> [-f <format>] [-F] [-i]
```

#### Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `dictionary_path` | Yes | Path to a compiled dictionary |
| `text` | Yes | Text to tokenize |

#### Options

| Flag | Description |
|------|-------------|
| `-f, --format` | Output format of the entries (default: `print`) |
| `-F, --follow` | Follow see_also redirects until finding an entry with etymologies |
| `-i, --insensitive` | Perform case-insensitive lookups when matching tokens (default: `false`) |
| `-r, --retries` | Number of times to retry loading the dictionary (remote-only) (default: `3`) |

---

### `alias`

Manage dictionary aliases.

#### `alias add`

Attempts to create a new dictionary alias, failing if one already exists with the given name.

```
odict alias add <name> <path>
```

| Argument | Required | Description |
|----------|----------|-------------|
| `name` | Yes | Name of the alias |
| `path` | Yes | Dictionary path |

| Flag | Description |
|------|-------------|
| `-r, --retries` | Number of times to retry loading the dictionary (remote-only) (default: `3`) |

#### `alias set`

Creates or updates an existing dictionary alias.

```
odict alias set <name> <path>
```

| Argument | Required | Description |
|----------|----------|-------------|
| `name` | Yes | Name of the alias |
| `path` | Yes | Dictionary path |

| Flag | Description |
|------|-------------|
| `-r, --retries` | Number of times to retry loading the dictionary (remote-only) (default: `3`) |

#### `alias delete`

Deletes an alias with the given name if it exists.

```
odict alias delete <name>
```

| Argument | Required | Description |
|----------|----------|-------------|
| `name` | Yes | Name of the alias |

---
