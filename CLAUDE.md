# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ODict is a fast, portable dictionary file format. XML source files (.odxml) compile to compact binary dictionaries (.odict) using CBOR serialization and Brotli compression. The project is a Rust workspace monorepo with language bindings for Node.js and Python.

## Build & Development Commands

This project uses `mise` as the monorepo task runner. All tasks are defined in `mise.toml` and per-crate `mise.toml` files.

### Core Commands

| Command | Purpose |
|---------|---------|
| `mise build` | Build the CLI binary |
| `mise build:all` | Build all packages (CLI, lib, node, python) |
| `mise test` | Run core Rust tests (lib + cli + internal) |
| `mise test:all` | Run all tests including language bindings |
| `mise lint` | Lint all Rust code (clippy + per-crate lints) |
| `mise fix` | Auto-fix clippy warnings |
| `mise format` | Format all Rust code (`cargo fmt --all`) |
| `mise snapshot` | Accept insta snapshot updates (`cargo insta accept`) |
| `mise bench` | Run criterion benchmarks |
| `mise setup` | Install all dependencies |

### Running a Single Test

```bash
# By test name
cargo nextest run -p odict --all-features <test_name>

# By crate
cargo nextest run -p odict-cli --all-features
```

### Language Binding Development

**Node** (`crates/node/`): Uses napi-rs, yarn 4, AVA for tests, oxlint + prettier for linting.
```bash
cd crates/node && mise test    # runs yarn test (AVA)
cd crates/node && mise build   # builds native extension
```

**Python** (`crates/python/`): Uses PyO3 + maturin, pytest + syrupy for snapshot tests, ruff for linting.
```bash
cd crates/python && mise test    # runs pytest
cd crates/python && mise build   # builds wheel via maturin
```

## Architecture

### Workspace Crates

```
crates/
├── lib/       → `odict` - Core library (cdylib + staticlib + rlib)
│                 All dictionary logic: compile, lookup, search, merge, serialize
├── cli/       → `odict-cli` - CLI binary (clap-based)
│                 Subcommands: alias, compile, download, dump, index, info,
│                 lexicon, lookup, merge, new, search, serve, tokenize
├── internal/  → `internal` - Shared internal utilities (not published)
├── node/      → Node.js NAPI bindings
└── python/    → Python PyO3 bindings
```

### Feature Flags (lib crate)

The lib has extensive feature flags controlling optional functionality:

- **default**: `sql`, `config`
- **search**: Full-text search via tantivy
- **http**: Remote dictionary fetching via reqwest
- **html/markdown**: Markup rendering in definitions
- **alias**: Named dictionary shortcuts
- **tokenize**: Text tokenization (with per-language sub-features: `tokenize-chinese`, `tokenize-japanese`, `tokenize-thai`, etc.)

The CLI enables: `sql`, `search`, `http`, `alias`, `html`, `tokenize`.

### Key Patterns

- **Error handling**: `thiserror` derive macros with a central `Error` enum per crate; `Result<T> = Result<T, Error>` alias
- **Serialization**: `rkyv` for zero-copy deserialization of dictionary data, `serde` for config/interchange formats
- **Testing**: `cargo-nextest` runner, `insta` for snapshot assertions, test dictionaries loaded via `LazyLock` from `examples/` directory
- **XML Schema**: Dictionary format defined in `odict.xsd`

### Data Flow

XML source (.odxml) → `quick-xml` parser → internal AST → `rkyv` serialization → Brotli compression → .odict binary

Lookup: .odict binary → Brotli decompression → `rkyv` zero-copy access → results

## CI/CD

- **ci.yml**: Builds and tests on Linux/macOS/Windows
- **node.yml**: Builds native extensions for 14+ platforms, publishes to npm
- **python.yml**: Builds wheels for multiple architectures, publishes to PyPI
- **release-please.yml**: Automated version bumping and release PRs (lib+cli linked, node/python separate)

## Release Configuration

Release profiles use `lto = true`, `codegen-units = 1`, `strip = true`, `opt-level = 3` for maximum optimization. Distribution builds (`profile.dist`) use `lto = "thin"` for faster CI builds.

Cross-compilation targets are configured in `.cargo/config.toml` (Windows static CRT, ARM linker settings).
