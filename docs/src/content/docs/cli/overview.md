---
title: CLI Overview
description: What the ODict CLI can do and when to use each command.
---

The ODict CLI is the fastest way to create, inspect, combine, and query dictionaries from a terminal. Most workflows start with XML, compile it to a
`.odict` file, and then use the compiled file for lookup, search, tokenization, or serving over HTTP.

## What you can do

| Task                      | Command          | Use it when                                                                               |
| ------------------------- | ---------------- | ----------------------------------------------------------------------------------------- |
| Create a starter XML file | `odict new`      | You want a blank ODXML file with the dictionary name filled in.                           |
| Compile XML               | `odict compile`  | You are ready to turn ODXML into a compact `.odict` file.                                 |
| Inspect a dictionary      | `odict info`     | You want basic metadata such as file version, size, and entry count.                      |
| List headwords            | `odict lexicon`  | You need every term defined in a dictionary.                                              |
| Look up entries           | `odict lookup`   | You know the word and want an exact dictionary entry.                                     |
| Split compound text       | `odict split`    | You want to break a term into known dictionary words without trying the whole term first. |
| Build a search index      | `odict index`    | You want full-text search across definitions.                                             |
| Search definitions        | `odict search`   | You know the meaning or phrase, but not necessarily the headword.                         |
| Tokenize text             | `odict tokenize` | You want language-aware tokens matched against dictionary entries.                        |
| Convert back to text      | `odict dump`     | You want XML or SQL output from a compiled dictionary.                                    |
| Merge dictionaries        | `odict merge`    | You want to combine entries from multiple dictionaries.                                   |
| Download dictionaries     | `odict download` | You want a dictionary from the remote registry.                                           |
| Manage aliases            | `odict alias`    | You want short names for local or downloaded dictionaries.                                |
| Serve over HTTP           | `odict serve`    | You want JSON lookup, search, split, or tokenization endpoints.                           |

## Common workflow

```bash
odict new animals -n "Animal Dictionary"
odict compile animals.xml
odict lookup animals.odict cat
```

Add search when you need discovery by definition text:

```bash
odict index animals.odict
odict search animals.odict "domesticated mammal"
```

Use `dump` when you want to inspect or transform a compiled dictionary:

```bash
odict dump animals.odict -f xml
odict dump animals.odict -f sqlite -o animals.sql
```

Use `merge` when you want to combine multiple compiled dictionaries:

```bash
odict merge base.odict extra.odict -o combined.odict
```

## Paths, aliases, and remote names

Most commands accept a local `.odict` path. Commands that load dictionaries can also use aliases or remote names, depending on the feature:

```bash
odict download wiktionary/eng
odict alias set eng ./eng.odict
odict lookup eng hello
```

Remote dictionary names use the form `dictionary/language`, such as `wiktionary/eng`.

## Reference

For every flag and argument, see the [Command Reference](/cli/reference/). For guided workflows, see [Downloading Dictionaries](/guides/downloading/),
[Aliases and Loading](/guides/aliases/), and [Serving Dictionaries](/guides/serving/).
