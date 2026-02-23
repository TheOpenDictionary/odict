/**
 * Generates CLI reference documentation by parsing the clap arg definitions
 * directly from the Rust source files in cli/src/.
 *
 * Run: node scripts/generate-cli-docs.mjs
 *
 * Outputs: src/content/docs/cli/reference.md
 */

import { readFileSync, writeFileSync, mkdirSync, readdirSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const cliSrc = join(__dirname, "../../cli/src");
const outPath = join(__dirname, "../src/content/docs/cli/reference.md");

// ---------------------------------------------------------------------------
// Read all Rust source files
// ---------------------------------------------------------------------------

function readRustFile(relPath) {
  return readFileSync(join(cliSrc, relPath), "utf-8");
}

// ---------------------------------------------------------------------------
// Parse the Commands enum from cli.rs to get command descriptions
// ---------------------------------------------------------------------------

function parseCommandsEnum(source) {
  const commands = {};
  // Match: /// doc comment followed by variant name
  const re = /\/\/\/\s*(.*)\n\s*(?:#\[.*\]\n\s*)*(\w+)\((\w+)\)/g;
  let m;
  while ((m = re.exec(source)) !== null) {
    const doc = m[1].trim();
    const variant = m[2];
    commands[variant] = doc;
  }
  return commands;
}

// ---------------------------------------------------------------------------
// Parse #[arg(...)] fields from an Args struct
// ---------------------------------------------------------------------------

function parseArgsStruct(source) {
  const fields = [];

  // Find the struct body (everything between the first { and last })
  const structMatch = source.match(
    /pub\s+struct\s+\w+Args\s*\{([\s\S]*?)\n\}/
  );
  if (!structMatch) return fields;

  const body = structMatch[1];

  // Split by field declarations - each field may have preceding attributes and doc comments
  // We look for patterns like:
  //   /// doc comment
  //   #[arg(...)]
  //   pub field_name: Type,
  //   -- or --
  //   #[arg(..., help = "...")]
  //   field_name: Type,

  const fieldRegex =
    /((?:\/\/\/[^\n]*\n\s*|#\[(?:arg|pyo3)[^\]]*\]\n\s*)*)\s*(?:pub(?:\((?:super|crate)\))?\s+)?(\w+)\s*:\s*([^,\n]+)/g;

  let fm;
  while ((fm = fieldRegex.exec(body)) !== null) {
    const attrs = fm[1];
    const name = fm[2];
    const type = fm[3].trim();

    // Skip command subcommand fields
    if (attrs.includes("#[command")) continue;

    // Parse #[arg(...)] attributes
    const argAttr = attrs.match(/#\[arg\(([\s\S]*?)\)\]/);
    const argContent = argAttr ? argAttr[1] : "";

    // Extract help text
    let help = extractQuoted(argContent, "help");

    // Fall back to /// doc comments
    if (!help) {
      const docMatch = attrs.match(/\/\/\/\s*(.*)/);
      if (docMatch) help = docMatch[1].trim();
    }

    // Extract short flag
    let short = null;
    const shortMatch = argContent.match(
      /short\s*=\s*'([^']+)'/
    );
    if (shortMatch) {
      short = `-${shortMatch[1]}`;
    } else if (/\bshort\b/.test(argContent) && !/short\s*=/.test(argContent)) {
      // bare `short` means use first char of field name
      short = `-${name[0]}`;
    }

    // Extract long flag
    let long = null;
    const longMatch = argContent.match(
      /long\s*=\s*"([^"]+)"/
    );
    if (longMatch) {
      long = `--${longMatch[1]}`;
    } else if (/\blong\b/.test(argContent) && !/long\s*=/.test(argContent)) {
      // bare `long` means use field name with _ -> -
      long = `--${name.replace(/_/g, "-")}`;
    }

    // Check if required
    const required =
      argContent.includes("required = true") ||
      (type !== "bool" &&
        !type.startsWith("Option<") &&
        !type.startsWith("Vec<") &&
        !short &&
        !long &&
        !argContent.includes("default_value"));

    // Check for default value
    let defaultVal = null;
    const defaultMatch = argContent.match(
      /default_value_t\s*=\s*([^,\)]+)/
    );
    if (defaultMatch) {
      defaultVal = defaultMatch[1].trim();
      // Clean up Rust-specific patterns
      defaultVal = defaultVal
        .replace(/crate::DEFAULT_RETRIES/, "3")
        .replace(/DEFAULT_INDEX_MEMORY/, "15000000")
        .replace(/DumpFormat::XML/, "xml")
        .replace(/PrintFormat::Print/, "print")
        .replace(/PrintFormat::JSON/, "json");
    }

    // Determine if this is a positional arg or a flag
    const isPositional = !short && !long && !argContent.includes("default_value_t") && type !== "bool";

    // Extract value_enum
    const isValueEnum = argContent.includes("value_enum");

    // Determine the arg type for display
    let argType = null;
    if (type === "bool" || type === "Option<bool>") {
      argType = null; // boolean flags don't take a value
    } else if (isValueEnum) {
      argType = `<${name}>`;
    } else if (type.includes("PathBuf") || type.includes("String")) {
      argType = `<${name}>`;
    } else if (type.includes("u32") || type.includes("usize") || type.includes("u16")) {
      argType = `<${name}>`;
    } else if (type.includes("Vec<String>")) {
      argType = `<${name}...>`;
    }

    // Extract value_parser range info for help
    const rangeMatch = argContent.match(/value_parser.*?range\((\d+)\.\.=(\d+)\)/);
    if (rangeMatch) {
      const rangeInfo = `(${rangeMatch[1]}–${rangeMatch[2]})`;
      if (help && !help.includes(rangeMatch[1])) {
        help = `${help} ${rangeInfo}`;
      }
    }

    fields.push({
      name,
      type,
      short,
      long,
      help: help || "",
      required,
      isPositional,
      defaultVal,
      argType,
    });
  }

  return fields;
}

function extractQuoted(text, key) {
  // Match: key = "value" where value may span multiple lines due to formatting
  const re = new RegExp(`${key}\\s*=\\s*"([^"]*)"`, "s");
  const m = re.exec(text);
  return m ? m[1].trim() : null;
}

// ---------------------------------------------------------------------------
// Parse the AliasCommands enum
// ---------------------------------------------------------------------------

function parseAliasCommands(source) {
  const commands = {};
  const re = /\/\/\/\s*(.*)\n\s*(?:#\[.*\]\n\s*)*(\w+)\((\w+)\)/g;
  let m;
  while ((m = re.exec(source)) !== null) {
    commands[m[2]] = m[1].trim();
  }
  return commands;
}

// ---------------------------------------------------------------------------
// Parse HTTP serve endpoint structs from serve/ directory
// ---------------------------------------------------------------------------

function parseServeEndpoints() {
  const endpoints = [];

  for (const file of ["lookup.rs", "search.rs", "tokenize.rs"]) {
    const source = readRustFile(`serve/${file}`);

    // Extract route path: #[get("/{name}/...")]
    const routeMatch = source.match(/#\[get\("([^"]+)"\)\]/);
    if (!routeMatch) continue;
    const route = routeMatch[1];

    // Extract request struct fields
    const structMatch = source.match(
      /pub\s+struct\s+(\w+Request)\s*\{([\s\S]*?)\}/
    );
    if (!structMatch) continue;

    const structName = structMatch[1];
    const body = structMatch[2];

    const params = [];
    const fieldRe = /(\w+)\s*:\s*([^,\n]+)/g;
    let fm;
    while ((fm = fieldRe.exec(body)) !== null) {
      const name = fm[1];
      const type = fm[2].trim().replace(/,$/, "");
      const isOptional = type.startsWith("Option<");
      const innerType = isOptional
        ? type.match(/Option<(\w+)>/)?.[1] || type
        : type;
      params.push({
        name,
        type: innerType === "String" ? "string" : innerType === "bool" ? "boolean" : "number",
        optional: isOptional,
      });
    }

    endpoints.push({ route, params });
  }

  return endpoints;
}

// ---------------------------------------------------------------------------
// Build CLI documentation from parsed source
// ---------------------------------------------------------------------------

const cliSource = readRustFile("cli.rs");
const commandDescs = parseCommandsEnum(cliSource);
const aliasSource = readRustFile("alias/alias.rs");
const aliasDescs = parseAliasCommands(aliasSource);

// Map command variant names to their source files
const commandFiles = {
  Compile: "compile.rs",
  Download: "download.rs",
  Dump: "dump.rs",
  Index: "index.rs",
  Info: "info.rs",
  Lexicon: "lexicon.rs",
  Lookup: "lookup.rs",
  Merge: "merge.rs",
  New: "new.rs",
  Search: "search.rs",
  Serve: "serve/mod.rs",
  Tokenize: "tokenize.rs",
};

const aliasFiles = {
  Add: "alias/set.rs",
  Set: "alias/set.rs",
  Delete: "alias/delete.rs",
};

// Parse serve HTTP endpoints
const serveEndpoints = parseServeEndpoints();

// ---------------------------------------------------------------------------
// Render Markdown
// ---------------------------------------------------------------------------

let md = `---
title: CLI Reference
description: Complete reference for the ODict command-line interface.
---

{/* This file is auto-generated by scripts/generate-cli-docs.mjs — do not edit manually. */}

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

// Render each main command
for (const [variant, file] of Object.entries(commandFiles)) {
  const source = readRustFile(file);
  const fields = parseArgsStruct(source);
  const desc = commandDescs[variant] || variant;
  const cmdName = variant.toLowerCase();

  md += `### \`odict ${cmdName}\`\n\n`;
  md += `${desc}.\n\n`;

  // Build usage string
  const positionals = fields.filter((f) => f.isPositional);
  const options = fields.filter((f) => !f.isPositional);
  let usage = `odict ${cmdName}`;
  for (const p of positionals) {
    if (p.type.includes("Vec<")) {
      usage += p.required ? ` <${p.name}...>` : ` [${p.name}...]`;
    } else {
      usage += p.required ? ` <${p.name}>` : ` [${p.name}]`;
    }
  }
  for (const o of options) {
    if (o.name === "retries") continue; // skip common retries flag in usage
    const flag = o.short || o.long;
    if (flag) {
      if (o.argType) {
        usage += ` [${flag} ${o.argType}]`;
      } else {
        usage += ` [${flag}]`;
      }
    }
  }
  md += `\`\`\`\n${usage}\n\`\`\`\n\n`;

  // Positional arguments table
  if (positionals.length > 0) {
    md += `#### Arguments\n\n`;
    md += `| Argument | Required | Description |\n`;
    md += `|----------|----------|-------------|\n`;
    for (const p of positionals) {
      md += `| \`${p.name}\` | ${p.required ? "Yes" : "No"} | ${p.help} |\n`;
    }
    md += `\n`;
  }

  // Options table
  if (options.length > 0) {
    md += `#### Options\n\n`;
    md += `| Flag | Description |\n`;
    md += `|------|-------------|\n`;
    for (const o of options) {
      const flags = [o.short, o.long].filter(Boolean).join(", ");
      let desc = o.help;
      if (o.defaultVal && !desc.includes("default")) {
        desc += ` (default: \`${o.defaultVal}\`)`;
      }
      md += `| \`${flags}\` | ${desc} |\n`;
    }
    md += `\n`;
  }

  // HTTP endpoints for serve command
  if (cmdName === "serve" && serveEndpoints.length > 0) {
    md += `#### HTTP endpoints\n\n`;
    md += `When running \`odict serve\`, the following REST endpoints become available. All return JSON.\n\n`;

    for (const ep of serveEndpoints) {
      md += `##### \`GET ${ep.route}\`\n\n`;
      md += `| Parameter | Type | Required | Description |\n`;
      md += `|-----------|------|----------|-------------|\n`;
      for (const p of ep.params) {
        md += `| \`${p.name}\` | ${p.type} | ${p.optional ? "No" : "Yes"} | |\n`;
      }
      md += `\n`;
    }
  }

  md += `---\n\n`;
}

// Render alias subcommands
md += `### \`odict alias\`\n\n`;
md += `Manage dictionary aliases.\n\n`;

for (const [variant, file] of Object.entries(aliasFiles)) {
  const source = readRustFile(file);
  const fields = parseArgsStruct(source);
  const desc = aliasDescs[variant] || variant;
  const cmdName = variant.toLowerCase();

  md += `#### \`odict alias ${cmdName}\`\n\n`;
  md += `${desc}.\n\n`;

  // Build usage
  const positionals = fields.filter((f) => f.isPositional);
  const options = fields.filter((f) => !f.isPositional);
  let usage = `odict alias ${cmdName}`;
  for (const p of positionals) {
    usage += p.required ? ` <${p.name}>` : ` [${p.name}]`;
  }
  md += `\`\`\`\n${usage}\n\`\`\`\n\n`;

  if (positionals.length > 0) {
    md += `| Argument | Required | Description |\n`;
    md += `|----------|----------|-------------|\n`;
    for (const p of positionals) {
      md += `| \`${p.name}\` | ${p.required ? "Yes" : "No"} | ${p.help} |\n`;
    }
    md += `\n`;
  }

  if (options.length > 0) {
    md += `| Flag | Description |\n`;
    md += `|------|-------------|\n`;
    for (const o of options) {
      const flags = [o.short, o.long].filter(Boolean).join(", ");
      let desc = o.help;
      if (o.defaultVal && !desc.includes("default")) {
        desc += ` (default: \`${o.defaultVal}\`)`;
      }
      md += `| \`${flags}\` | ${desc} |\n`;
    }
    md += `\n`;
  }
}

md += `---\n`;

// ---------------------------------------------------------------------------
// Write output
// ---------------------------------------------------------------------------

mkdirSync(dirname(outPath), { recursive: true });
writeFileSync(outPath, md, "utf-8");
console.log(`✅ Generated CLI reference → ${outPath}`);
