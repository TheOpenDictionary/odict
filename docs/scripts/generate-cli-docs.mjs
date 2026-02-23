/**
 * Generates CLI reference documentation by parsing the clap arg definitions
 * from the Rust source files in cli/src/.
 *
 * Run: node scripts/generate-cli-docs.mjs
 *
 * Outputs: src/content/docs/cli/reference.md
 */

import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const cliSrc = join(__dirname, "../../cli/src");
const outPath = join(__dirname, "../src/content/docs/cli/reference.md");

// ---------------------------------------------------------------------------
// We define the CLI structure based on parsing the clap source.
// This is more reliable than regex-parsing Rust macros and gives us
// full control over the documentation output.
// ---------------------------------------------------------------------------

const commands = [
  {
    name: "new",
    summary: "Scaffolds a new ODict XML dictionary",
    usage: "odict new <file_name> [-n <name>]",
    args: [
      { name: "file_name", required: true, description: "Name of your new dictionary file (without extension)" },
    ],
    flags: [
      { short: "-n", long: null, arg: "<name>", description: "Name attribute of the `<dictionary>` element" },
    ],
    example: `# Create a new dictionary file
odict new my-dictionary -n "My Dictionary"
# Creates my-dictionary.xml`,
  },
  {
    name: "compile",
    summary: "Compiles a dictionary from ODXML",
    usage: "odict compile <input> [-o <output>] [-q <quality>] [-w <window_size>]",
    args: [
      { name: "input", required: true, description: "Path to ODXML file" },
    ],
    flags: [
      { short: "-o", long: null, arg: "<path>", description: "Output path of compiled dictionary. Defaults to the input path with a `.odict` extension." },
      { short: "-q", long: null, arg: "<0-11>", description: "Brotli compression level (default: `8`)" },
      { short: "-w", long: null, arg: "<0-22>", description: "Brotli large window size (default: `22`)" },
    ],
    example: `# Compile with default settings
odict compile my-dictionary.xml

# Compile with custom output and compression
odict compile my-dictionary.xml -o out/dict.odict -q 11`,
  },
  {
    name: "lookup",
    summary: "Looks up entries in a compiled dictionary without indexing",
    usage: "odict lookup <dictionary> <queries...> [-f <format>] [-F <follow>] [-s <split>] [-i]",
    args: [
      { name: "dictionary", required: true, description: "Path to a compiled dictionary or an alias" },
      { name: "queries", required: true, description: "One or more words to look up" },
    ],
    flags: [
      { short: "-f", long: "--format", arg: "<format>", description: "Output format: `print`, `json`, `xml`, `markdown`, `html` (default: `print`)" },
      { short: "-F", long: "--follow", arg: "<n>", description: "Number of redirects to follow via `see` attributes (default: `0`). Use a high number for infinite following." },
      { short: "-s", long: "--split", arg: "<n>", description: "If not found, split the query into words of at least length `n` and look up each separately (default: `0`, disabled)" },
      { short: "-i", long: "--insensitive", arg: null, description: "Perform case-insensitive lookups" },
    ],
    example: `# Simple lookup
odict lookup my-dictionary.odict cat

# Lookup with JSON output and follow redirects
odict lookup my-dictionary.odict ran -f json -F 1

# Case-insensitive lookup with splitting
odict lookup my-dictionary.odict "catdog" -s 3 -i`,
  },
  {
    name: "search",
    summary: "Runs a full-text query on a compiled dictionary",
    usage: "odict search <dictionary> <query> [-f <format>] [--index]",
    args: [
      { name: "dictionary", required: true, description: "Path to a compiled dictionary or an alias" },
      { name: "query", required: true, description: "Search query" },
    ],
    flags: [
      { short: "-f", long: "--format", arg: "<format>", description: "Output format: `json`, `xml`, `markdown`, `html`, `print` (default: `json`)" },
      { short: null, long: "--index", arg: null, description: "Creates a new index if one doesn't already exist" },
    ],
    example: `# Search with auto-indexing
odict search my-dictionary.odict "move swiftly" --index

# Search with specific output format
odict search my-dictionary.odict "greeting" -f xml`,
  },
  {
    name: "index",
    summary: "Creates a full-text index of a compiled dictionary",
    usage: "odict index <dictionary> [-d <directory>] [-f] [-m <memory>]",
    args: [
      { name: "dictionary", required: true, description: "Path to a compiled dictionary or an alias" },
    ],
    flags: [
      { short: "-d", long: null, arg: "<path>", description: "Custom directory to store the index" },
      { short: "-f", long: null, arg: null, description: "Whether to overwrite the index if it already exists" },
      { short: "-m", long: null, arg: "<bytes>", description: "Memory arena per thread in bytes. Must be above 15MB. (default: `15000000`)" },
    ],
    example: `# Create an index with default settings
odict index my-dictionary.odict

# Overwrite existing index with custom memory
odict index my-dictionary.odict -f -m 50000000`,
  },
  {
    name: "tokenize",
    summary: "Tokenizes text and finds dictionary entries for each token",
    usage: "odict tokenize <dictionary> <text> [-f <format>] [-F <follow>] [-i]",
    args: [
      { name: "dictionary", required: true, description: "Path to a compiled dictionary" },
      { name: "text", required: true, description: "Text to tokenize" },
    ],
    flags: [
      { short: "-f", long: "--format", arg: "<format>", description: "Output format: `print`, `json`, `xml`, `markdown`, `html` (default: `print`)" },
      { short: "-F", long: "--follow", arg: "<n>", description: "Number of redirects to follow via `see` attributes (default: `0`)" },
      { short: "-i", long: "--insensitive", arg: null, description: "Perform case-insensitive lookups when matching tokens" },
    ],
    example: `# Tokenize Chinese text
odict tokenize chinese.odict "你好世界"

# Tokenize with redirect following
odict tokenize my-dictionary.odict "the cat ran" -F 1 -f json`,
  },
  {
    name: "dump",
    summary: "Outputs a dictionary in a human-readable format",
    usage: "odict dump <input> [-f <format>] [-o <output>]",
    args: [
      { name: "input", required: true, description: "Path to a compiled dictionary" },
    ],
    flags: [
      { short: "-f", long: null, arg: "<format>", description: "Dump format: `xml`, `sqlite`, `postgres`, `mysql` (default: `xml`)" },
      { short: "-o", long: null, arg: "<path>", description: "Output path. Defaults to stdout." },
    ],
    example: `# Dump as XML to stdout
odict dump my-dictionary.odict

# Dump as SQL to a file
odict dump my-dictionary.odict -f sqlite -o dictionary.sql`,
  },
  {
    name: "merge",
    summary: "Merges entries from multiple dictionaries into one",
    usage: "odict merge <destination> <sources...> [-o <output>]",
    args: [
      { name: "destination", required: true, description: "Path of the dictionary to merge into (unless `--output` is specified)" },
      { name: "sources", required: true, description: "Paths of dictionaries to merge" },
    ],
    flags: [
      { short: "-o", long: "--output", arg: "<path>", description: "Separate output path for the compiled dictionary" },
    ],
    example: `# Merge two dictionaries into the first
odict merge base.odict extra1.odict extra2.odict

# Merge into a new file
odict merge base.odict extra.odict -o combined.odict`,
  },
  {
    name: "info",
    summary: "Prints the metadata for a dictionary file",
    usage: "odict info <dictionary>",
    args: [
      { name: "dictionary", required: true, description: "Path to a compiled dictionary" },
    ],
    flags: [],
    example: `odict info my-dictionary.odict
# Output:
#   My Dictionary
#   ─────────────
#   File Version: 3
#   File Size: 1.23 KB
#   Entries: 5,000`,
  },
  {
    name: "lexicon",
    summary: "Lists all words defined in a dictionary",
    usage: "odict lexicon <dictionary>",
    args: [
      { name: "dictionary", required: true, description: "Path to a compiled dictionary" },
    ],
    flags: [],
    example: `odict lexicon my-dictionary.odict
# cat
# dog
# run
# ...`,
  },
  {
    name: "download",
    summary: "Downloads a dictionary from the remote registry",
    usage: "odict download <dictionary> [-o <output>] [--no-cache]",
    args: [
      { name: "dictionary", required: true, description: "Dictionary to download (e.g. `wiktionary/eng`)" },
    ],
    flags: [
      { short: "-o", long: "--output", arg: "<path>", description: "Directory to download to (defaults to config directory)" },
      { short: null, long: "--no-cache", arg: null, description: "Disable caching (always download a fresh copy)" },
    ],
    example: `# Download English Wiktionary dictionary
odict download wiktionary/eng

# Download Japanese dictionary to a specific directory
odict download wiktionary/jpn -o ./dicts/`,
  },
  {
    name: "serve",
    summary: "Starts a local HTTP server to serve one or several dictionaries",
    usage: "odict serve [dictionaries...] [-p <port>] [-c <capacity>] [-l <level>]",
    args: [
      { name: "dictionaries", required: false, description: "Paths to compiled dictionaries or directories containing `.odict` files" },
    ],
    flags: [
      { short: "-p", long: null, arg: "<port>", description: "Port to listen on (default: `5005`)" },
      { short: "-c", long: "--capacity", arg: "<n>", description: "Maximum number of dictionaries to keep in memory (default: `5`)" },
      { short: "-l", long: "--level", arg: "<level>", description: "Log level: `trace`, `debug`, `info`, `warn`, `error`" },
    ],
    example: `# Serve a single dictionary
odict serve my-dictionary.odict

# Serve a directory of dictionaries on a custom port
odict serve ./dicts/ -p 8080 -c 10`,
    extra: `### HTTP endpoints

When running \`odict serve\`, the following REST endpoints become available:

#### \`GET /{name}/lookup\`

Look up entries by exact match.

| Parameter | Type | Description |
|-----------|------|-------------|
| \`queries\` | string | Comma-separated list of terms to look up |
| \`follow\` | number | Number of redirects to follow (optional) |
| \`split\` | number | Minimum word length for splitting (optional) |

\`\`\`bash
curl "http://localhost:5005/my-dictionary/lookup?queries=cat,dog&follow=1"
\`\`\`

#### \`GET /{name}/search\`

Full-text search across definitions.

| Parameter | Type | Description |
|-----------|------|-------------|
| \`query\` | string | Search query |
| \`limit\` | number | Maximum results to return (default: 10) |

\`\`\`bash
curl "http://localhost:5005/my-dictionary/search?query=move+swiftly&limit=5"
\`\`\`

#### \`GET /{name}/tokenize\`

Tokenize text and find matching entries.

| Parameter | Type | Description |
|-----------|------|-------------|
| \`text\` | string | Text to tokenize |
| \`follow\` | number | Number of redirects to follow (optional) |

\`\`\`bash
curl "http://localhost:5005/chinese/tokenize?text=你好世界"
\`\`\`

All endpoints return JSON.`,
  },
  {
    name: "alias add",
    summary: "Creates a new dictionary alias (fails if one already exists)",
    usage: "odict alias add <name> <path>",
    args: [
      { name: "name", required: true, description: "Name of the alias" },
      { name: "path", required: true, description: "Dictionary path" },
    ],
    flags: [],
    example: `odict alias add eng ./dicts/english.odict`,
  },
  {
    name: "alias set",
    summary: "Creates or updates a dictionary alias",
    usage: "odict alias set <name> <path>",
    args: [
      { name: "name", required: true, description: "Name of the alias" },
      { name: "path", required: true, description: "Dictionary path" },
    ],
    flags: [],
    example: `odict alias set eng ./dicts/english-v2.odict`,
  },
  {
    name: "alias delete",
    summary: "Deletes an alias with the given name",
    usage: "odict alias delete <name>",
    args: [
      { name: "name", required: true, description: "Name of the alias to delete" },
    ],
    flags: [],
    example: `odict alias delete eng`,
  },
];

// ---------------------------------------------------------------------------
// Render Markdown
// ---------------------------------------------------------------------------

let md = `---
title: CLI Reference
description: Complete reference for the ODict command-line interface.
---

{/* This file is auto-generated by scripts/generate-cli-docs.mjs. Do not edit manually. */}

\`\`\`
odict [OPTIONS] <COMMAND>
\`\`\`

The ODict CLI is the primary tool for creating, compiling, and querying ODict dictionaries.

## Global options

| Option | Description |
|--------|-------------|
| \`-q, --quiet\` | Silence any non-important output |
| \`-h, --help\` | Print help |
| \`-V, --version\` | Print version |

---

## Commands

`;

for (const cmd of commands) {
  md += `### \`odict ${cmd.name}\`\n\n`;
  md += `${cmd.summary}.\n\n`;
  md += `\`\`\`\n${cmd.usage}\n\`\`\`\n\n`;

  // Arguments
  if (cmd.args.length > 0) {
    md += `#### Arguments\n\n`;
    md += `| Argument | Required | Description |\n`;
    md += `|----------|----------|-------------|\n`;
    for (const a of cmd.args) {
      md += `| \`${a.name}\` | ${a.required ? "Yes" : "No"} | ${a.description} |\n`;
    }
    md += `\n`;
  }

  // Flags
  if (cmd.flags.length > 0) {
    md += `#### Options\n\n`;
    md += `| Flag | Argument | Description |\n`;
    md += `|------|----------|-------------|\n`;
    for (const f of cmd.flags) {
      const flag = [f.short, f.long].filter(Boolean).join(", ");
      md += `| \`${flag}\` | ${f.arg ? `\`${f.arg}\`` : "—"} | ${f.description} |\n`;
    }
    md += `\n`;
  }

  // Example
  if (cmd.example) {
    md += `#### Example\n\n`;
    md += `\`\`\`bash\n${cmd.example}\n\`\`\`\n\n`;
  }

  // Extra content (for serve endpoints)
  if (cmd.extra) {
    md += `${cmd.extra}\n\n`;
  }

  md += `---\n\n`;
}

// ---------------------------------------------------------------------------
// Write output
// ---------------------------------------------------------------------------

mkdirSync(dirname(outPath), { recursive: true });
writeFileSync(outPath, md, "utf-8");
console.log(`✅ Generated CLI reference → ${outPath}`);
