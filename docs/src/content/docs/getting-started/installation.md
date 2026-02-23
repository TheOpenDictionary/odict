---
title: Installation
description: How to install the ODict CLI and language bindings.
---

## CLI

### Homebrew (macOS)

```bash
brew install TheOpenDictionary/odict/odict
```

### Shell installer (macOS / Linux)

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/TheOpenDictionary/odict/releases/latest/download/odict-installer.sh | sh
```

### PowerShell installer (Windows)

```powershell
powershell -ExecutionPolicy ByPass -c "irm https://github.com/TheOpenDictionary/odict/releases/latest/download/odict-installer.ps1 | iex"
```

### From source

Requires [Rust](https://rustup.rs/) 1.75+.

```bash
git clone https://github.com/TheOpenDictionary/odict.git
cd odict
cargo install --path cli
```

### Verify installation

```bash
odict --version
```

---

## Language bindings

### Python

```bash
pip install theopendictionary
```

Requires Python 3.8.1+. See the [Python API docs](/api/python/) for usage.

### JavaScript (Node.js)

```bash
npm install @odict/node
```

Requires Node.js 12+. The package includes native binaries for all major platforms. See the [JavaScript API docs](/api/javascript/) for usage.

### Rust

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
odict = "2"
```

See the [Rust API docs](/api/rust/) for usage and feature flags.
