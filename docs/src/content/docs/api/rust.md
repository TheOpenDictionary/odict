---
title: Rust API
description: Using the ODict Rust crate.
---

The `odict` crate is the core library that powers the CLI and all language bindings. It is published on [crates.io](https://crates.io/crates/odict).

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
odict = "2"
```

## Documentation

Full API documentation is available on **docs.rs**:

**[docs.rs/odict](https://docs.rs/odict)**

## Feature flags

The `odict` crate uses feature flags to control which capabilities are compiled in. The `default` feature includes `sql` and `config`.

| Feature | Description |
|---------|-------------|
| `default` | Enables `sql` and `config` |
| `sql` | SQL dump support (SQLite, PostgreSQL, MySQL) via sea-query |
| `config` | Access to platform-specific config directories |
| `alias` | Dictionary alias management (implies `config`) |
| `search` | Full-text search via Tantivy (implies `config`) |
| `markdown` | Markdown rendering support via pulldown-cmark |
| `html` | HTML output support (implies `markdown`) |
| `http` | Remote dictionary downloading (implies `config`) |
| `tokenize` | Full multi-language tokenization (enables all language tokenizers) |
| `tokenize-latin` | Latin-script tokenization |
| `tokenize-chinese` | Chinese segmentation |
| `tokenize-japanese` | Japanese segmentation (UniDic) |
| `tokenize-korean` | Korean segmentation |
| `tokenize-thai` | Thai segmentation |
| `tokenize-khmer` | Khmer segmentation |
| `tokenize-swedish` | Swedish recomposition |
| `tokenize-german` | German segmentation |

## Quick example

```rust
use odict::{OpenDictionary, ToDictionary};

fn main() -> odict::Result<()> {
    // Compile from XML
    let xml = r#"
      <dictionary name="Example">
        <entry term="hello">
          <ety>
            <sense pos="intj">
              <definition value="A greeting" />
            </sense>
          </ety>
        </entry>
      </dictionary>
    "#;

    // Compile and write to disk
    let dict = xml.to_dictionary()?.build()?;
    dict.to_disk("example.odict")?;

    // Read from disk
    let file = OpenDictionary::from_path("example.odict")?;
    let contents = file.contents()?;

    // Lookup
    let results = contents.lookup(
        &["hello"],
        &odict::lookup::LookupOptions::default(),
    )?;

    println!("{:?}", results);
    Ok(())
}
```

## Key traits and types

| Type | Description |
|------|-------------|
| `OpenDictionary` | A compiled dictionary loaded from disk or bytes |
| `ToDictionary` | Trait for converting XML strings to `Dictionary` |
| `Dictionary` | The deserialized dictionary schema type |
| `CompilerOptions` | Options for compiling (compression settings) |
| `lookup::LookupOptions` | Options for exact-match lookups |
| `search::SearchOptions` | Options for full-text search |
| `index::IndexOptions` | Options for creating a search index |
| `tokenize::TokenizeOptions` | Options for text tokenization |

Refer to the [docs.rs documentation](https://docs.rs/odict) for complete details on all types, traits, and methods.
