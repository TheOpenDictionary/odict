---
title: Serving Dictionaries
description: How to serve ODict dictionaries over HTTP.
---

`odict serve` starts a local JSON API for lookup, search, split, and tokenization. It is useful when an application in another language needs dictionary access without embedding a native binding.

## Start the server

```bash
odict serve animals.odict -p 8080
```

The server exposes each dictionary by file stem. For `animals.odict`, the route prefix is `/animals`.

You can serve multiple dictionaries or every `.odict` file in a directory:

```bash
odict serve animals.odict plants.odict -p 8080
odict serve ./dictionaries -p 8080
```

## Endpoints

All successful responses are JSON.

| Endpoint               | Parameters                  | Use it for                                                   |
| ---------------------- | --------------------------- | ------------------------------------------------------------ |
| `GET /{name}/lookup`   | `q`, `follow`, `split`      | Exact headword lookup. Use commas in `q` for multiple terms. |
| `GET /{name}/search`   | `q`, `limit`                | Full-text search across definitions.                         |
| `GET /{name}/split`    | `q`, `follow`, `min_length` | Compound splitting without first trying the whole query.     |
| `GET /{name}/tokenize` | `text`, `follow`            | Language-aware tokenization and dictionary matching.         |

## Examples

```bash
curl "http://localhost:8080/animals/lookup?q=cat,dog"
curl "http://localhost:8080/animals/lookup?q=kitty&follow=true"
curl "http://localhost:8080/animals/search?q=domesticated&limit=5"
curl "http://localhost:8080/animals/split?q=catdog&min_length=3"
curl "http://localhost:8080/animals/tokenize?text=the+cat+and+the+dog"
```

## Cache size and logging

The server keeps recently used dictionaries in memory. Use `--capacity` to control how many dictionaries stay cached:

```bash
odict serve ./dictionaries --capacity 10
```

Use `--level` to set the log level:

```bash
odict serve animals.odict --level debug
```

:::note
Search endpoints require an index. Build it ahead of time with `odict index`, or use CLI search with `--index` when you want the index created automatically.
:::
