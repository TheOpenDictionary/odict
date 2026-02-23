---
title: Introduction
description: What is ODict and why does it exist?
---

**ODict** (The Open Dictionary) is a blazingly-fast, open-source dictionary file format designed for human languages. It provides a complete pipeline for defining, compiling, and querying dictionaries:

1. **Define** your dictionary entries in a simple XML format (ODXML)
2. **Compile** the XML into a compact binary `.odict` file
3. **Query** the compiled dictionary using exact lookups, full-text search, or multi-language tokenization

## Why ODict?

Most dictionary data is locked in proprietary formats, scattered across inconsistent APIs, or stored in slow, bloated files. ODict addresses these problems:

- **Universal schema** — A single, well-defined XML schema that can represent dictionaries for any human language, including etymologies, multiple senses, pronunciations, examples, and cross-references.
- **Fast binary format** — Compiled `.odict` files use [rkyv](https://rkyv.org/) for zero-copy deserialization and Brotli compression, making lookups extremely fast even on large dictionaries.
- **Full-text search** — Built-in indexing and search powered by [Tantivy](https://github.com/quickwit-oss/tantivy).
- **Multi-language tokenization** — Tokenize text in Chinese, Japanese, Korean, Thai, Khmer, German, Swedish, and Latin-script languages, and automatically match tokens to dictionary entries.
- **Cross-platform bindings** — Use ODict from Rust, Python, JavaScript (Node.js and browser), or through the CLI and HTTP server.

## Architecture

```
┌─────────────┐     ┌──────────┐     ┌─────────────┐
│  ODXML file  │────▶│ Compiler │────▶│  .odict file │
│  (XML)       │     │          │     │  (binary)    │
└─────────────┘     └──────────┘     └──────┬──────┘
                                            │
                    ┌───────────────────────┬┴──────────────────────┐
                    │                       │                       │
              ┌─────▼─────┐         ┌──────▼──────┐        ┌──────▼──────┐
              │   Lookup   │         │   Search    │        │  Tokenize   │
              │ (exact key)│         │ (full-text) │        │ (NLP-based) │
              └───────────┘         └─────────────┘        └─────────────┘
```

## What's next?

- [Install the CLI](/getting-started/installation/) to start working with dictionaries
- [Quick Start](/getting-started/quickstart/) walks you through creating and compiling your first dictionary
- Browse the [XML Schema Reference](/schema/reference/) to learn the full data model
