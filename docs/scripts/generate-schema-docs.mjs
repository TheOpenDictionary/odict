/**
 * Parses odict.xsd and lib/src/schema/pos.rs to generate
 * a Markdown reference page for the ODict XML schema.
 *
 * Run: node scripts/generate-schema-docs.mjs
 *
 * Outputs: src/content/docs/schema/reference.md
 */

import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const xsdPath = join(__dirname, "../../odict.xsd");
const posPath = join(__dirname, "../../lib/src/schema/pos.rs");
const outPath = join(__dirname, "../src/content/docs/schema/reference.md");

const xsd = readFileSync(xsdPath, "utf-8");
const posSource = readFileSync(posPath, "utf-8");

// ---------------------------------------------------------------------------
// Tokenize XSD into open / close / self-closing tags
// ---------------------------------------------------------------------------

function tokenize(xml) {
  const tokens = [];
  const re = /<(\/?)(\w[\w:.]*)((?:\s+[\w:]+\s*=\s*"[^"]*")*)\s*(\/?)>/g;
  let m;
  while ((m = re.exec(xml)) !== null) {
    const isClose = m[1] === "/";
    const tag = m[2];
    const attrStr = m[3];
    const isSelfClose = m[4] === "/";

    const attrs = {};
    const attrRe = /([\w:]+)\s*=\s*"([^"]*)"/g;
    let am;
    while ((am = attrRe.exec(attrStr)) !== null) {
      attrs[am[1]] = am[2];
    }

    if (isClose) {
      tokens.push({ type: "close", tag, attrs });
    } else if (isSelfClose) {
      tokens.push({ type: "selfclose", tag, attrs });
    } else {
      tokens.push({ type: "open", tag, attrs });
    }
  }
  return tokens;
}

const tokens = tokenize(xsd);

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/** Find the index of the matching close tag for an open tag at `openIndex`. */
function findClose(openIndex) {
  const openTag = tokens[openIndex].tag;
  let depth = 1;
  for (let i = openIndex + 1; i < tokens.length; i++) {
    if (tokens[i].tag === openTag) {
      if (tokens[i].type === "open") depth++;
      else if (tokens[i].type === "close") {
        depth--;
        if (depth === 0) return i;
      }
    }
  }
  return -1;
}

/**
 * Parse a complexType range (ctOpen..ctClose) for its direct attributes
 * and direct child elements. "Direct" means not inside a nested
 * xs:complexType — we track xs:complexType nesting depth and only
 * collect items at depth 0.
 */
function parseComplexTypeRange(ctOpen, ctClose) {
  const attributes = [];
  const children = [];
  let depth = 0;

  for (let i = ctOpen + 1; i < ctClose; i++) {
    const t = tokens[i];

    // Track nesting of inner xs:complexType blocks
    if (t.tag === "xs:complexType") {
      if (t.type === "open") depth++;
      else if (t.type === "close") depth--;
      continue;
    }

    if (depth > 0) continue; // inside a nested complexType — skip

    // Collect attributes at depth 0
    if (
      t.tag === "xs:attribute" &&
      t.attrs.name &&
      (t.type === "selfclose" || t.type === "open")
    ) {
      attributes.push({
        name: t.attrs.name,
        type: (t.attrs.type || "xs:string").replace("xs:", ""),
        required: t.attrs.use === "required",
      });
    }

    // Collect child elements at depth 0
    if (
      t.tag === "xs:element" &&
      t.attrs.name &&
      (t.type === "selfclose" || t.type === "open")
    ) {
      children.push({
        name: t.attrs.name,
        type: t.attrs.type || undefined,
        minOccurs: t.attrs.minOccurs ?? "1",
        maxOccurs: t.attrs.maxOccurs ?? "1",
      });

      // Skip past this element's entire subtree
      if (t.type === "open") {
        i = findClose(i);
      }
    }
  }

  return { attributes, children };
}

// ---------------------------------------------------------------------------
// Phase 1: Parse named complexTypes (defined at XSD top-level)
// ---------------------------------------------------------------------------

const namedTypes = new Map();

for (let i = 0; i < tokens.length; i++) {
  const t = tokens[i];
  if (t.tag === "xs:complexType" && t.type === "open" && t.attrs.name) {
    const ctClose = findClose(i);
    const { attributes, children } = parseComplexTypeRange(i, ctClose);
    namedTypes.set(t.attrs.name, { attributes, children });
    i = ctClose;
  }
}

// ---------------------------------------------------------------------------
// Phase 2: Recursively walk the root element to build element map
// ---------------------------------------------------------------------------

const elements = new Map();

/** Resolve a named complexType into an element descriptor and register children. */
function resolveNamedType(typeName) {
  const type = namedTypes.get(typeName);
  if (!type) return { attributes: [], children: [] };

  const children = type.children.map((c) => ({
    name: c.name,
    minOccurs: c.minOccurs,
    maxOccurs: c.maxOccurs,
  }));

  // Recursively register child elements that reference named types
  for (const child of type.children) {
    if (!elements.has(child.name) && child.type && namedTypes.has(child.type)) {
      elements.set(child.name, resolveNamedType(child.type));
    } else if (!elements.has(child.name)) {
      elements.set(child.name, { attributes: [], children: [] });
    }
  }

  return { attributes: [...type.attributes], children };
}

/** Process an xs:element token at `index` and register it in the elements map. */
function processElement(index) {
  const t = tokens[index];
  const name = t.attrs.name;
  const type = t.attrs.type;

  if (elements.has(name)) return;

  // Self-closing element or element with a named type
  if (t.type === "selfclose") {
    if (type && namedTypes.has(type)) {
      elements.set(name, resolveNamedType(type));
    } else {
      elements.set(name, { attributes: [], children: [] });
    }
    return;
  }

  // Open element with a named type (no inline complexType)
  if (type && namedTypes.has(type)) {
    elements.set(name, resolveNamedType(type));
    return;
  }

  const elClose = findClose(index);

  // Find the inline xs:complexType within this element
  for (let i = index + 1; i < elClose; i++) {
    if (tokens[i].tag === "xs:complexType" && tokens[i].type === "open") {
      const ctClose = findClose(i);
      const { attributes, children } = parseComplexTypeRange(i, ctClose);

      elements.set(name, {
        attributes,
        children: children.map((c) => ({
          name: c.name,
          minOccurs: c.minOccurs,
          maxOccurs: c.maxOccurs,
        })),
      });

      // Recursively process child elements found at depth 0
      let depth = 0;
      for (let j = i + 1; j < ctClose; j++) {
        if (tokens[j].tag === "xs:complexType") {
          if (tokens[j].type === "open") depth++;
          else if (tokens[j].type === "close") depth--;
          continue;
        }
        if (depth > 0) continue;

        if (
          tokens[j].tag === "xs:element" &&
          tokens[j].attrs.name &&
          (tokens[j].type === "selfclose" || tokens[j].type === "open")
        ) {
          processElement(j);
          if (tokens[j].type === "open") {
            j = findClose(j);
          }
        }
      }

      break;
    }
  }
}

// Find the root <xs:element name="dictionary"> and process it
for (let i = 0; i < tokens.length; i++) {
  const t = tokens[i];
  if (
    t.tag === "xs:element" &&
    t.attrs.name === "dictionary" &&
    (t.type === "open" || t.type === "selfclose")
  ) {
    processElement(i);
    break;
  }
}

// ---------------------------------------------------------------------------
// Parse POS codes from lib/src/schema/pos.rs
// ---------------------------------------------------------------------------

function parsePosEnum(source) {
  const entries = [];
  const re =
    /#\[strum\(to_string\s*=\s*"([^"]+)"\)\]\s*(?:#\[.*\]\s*)*(\w+)/g;
  let m;
  while ((m = re.exec(source)) !== null) {
    const label = m[1];
    const variant = m[2];
    if (variant === "Other") continue;
    entries.push({ variant, label });
  }
  return entries;
}

const allPos = parsePosEnum(posSource);

const japaneseVariantPrefixes = [
  "AdjPn", "AdjKari", "AdjKu", "AdjNari", "AdjNa", "AdjShiku",
  "AdjT", "AdjIx", "NAdv", "AdvTo", "AdjNo", "NPref", "NSuf",
  "NT", "AdjF", "V5", "V1", "Vz", "Vk", "V2", "Vn", "Vr",
  "VsC", "Vs", "VUnspec", "V4",
];

function isJapanese(variant) {
  return japaneseVariantPrefixes.some(
    (p) => variant === p || variant.startsWith(p)
  );
}

const universalPos = allPos.filter((p) => !isJapanese(p.variant));
const japanesePos = allPos.filter((p) => isJapanese(p.variant));

function variantToCode(variant) {
  return variant
    .replace(/([a-z])([A-Z])/g, "$1_$2")
    .replace(/([A-Z]+)([A-Z][a-z])/g, "$1_$2")
    .toLowerCase();
}

// ---------------------------------------------------------------------------
// Build element hierarchy tree (with deduplication via seen set)
// ---------------------------------------------------------------------------

function buildTree(name, prefix = "", isLast = true, seen = new Set(), isRoot = true) {
  const el = elements.get(name);
  const connector = isRoot ? "" : isLast ? "└── " : "├── ";

  if (seen.has(name)) {
    return `${prefix}${connector}${name} …\n`;
  }

  let result = `${prefix}${connector}${name}\n`;
  seen.add(name);

  if (!el || el.children.length === 0) return result;

  const childPrefix = isRoot ? "" : prefix + (isLast ? "    " : "│   ");

  for (let i = 0; i < el.children.length; i++) {
    const child = el.children[i];
    const childIsLast = i === el.children.length - 1;
    result += buildTree(child.name, childPrefix, childIsLast, seen, false);
  }

  return result;
}

// ---------------------------------------------------------------------------
// Render Markdown
// ---------------------------------------------------------------------------

const elementOrder = [
  "dictionary", "entry", "ety", "sense", "group",
  "definition", "note", "example", "pronunciation", "url",
];

let md = `---
title: XML Schema Reference
description: Complete reference for the ODict XML (ODXML) schema.
---

{/* This file is auto-generated by scripts/generate-schema-docs.mjs — do not edit manually. */}

This page is automatically generated from [\`odict.xsd\`](https://github.com/TheOpenDictionary/odict/blob/main/odict.xsd) and [\`pos.rs\`](https://github.com/TheOpenDictionary/odict/blob/main/lib/src/schema/pos.rs).

## Element hierarchy

\`\`\`
${buildTree("dictionary").trimEnd()}
\`\`\`

---

## Elements

`;

for (const name of elementOrder) {
  const el = elements.get(name);
  if (!el) continue;

  md += `### \`<${name}>\`\n\n`;

  if (el.attributes.length > 0) {
    md += `#### Attributes\n\n`;
    md += `| Attribute | Type | Required |\n`;
    md += `|-----------|------|----------|\n`;
    for (const a of el.attributes) {
      md += `| \`${a.name}\` | \`${a.type}\` | ${a.required ? "Yes" : "No"} |\n`;
    }
    md += `\n`;
  }

  if (el.children.length > 0) {
    md += `#### Child elements\n\n`;
    md += `| Element | Min | Max |\n`;
    md += `|---------|-----|-----|\n`;
    for (const c of el.children) {
      md += `| [\`<${c.name}>\`](#${c.name}) | ${c.minOccurs} | ${c.maxOccurs} |\n`;
    }
    md += `\n`;
  }

  md += `---\n\n`;
}

// ---------------------------------------------------------------------------
// Parts of Speech
// ---------------------------------------------------------------------------

md += `## Parts of speech\n\n`;
md += `The \`pos\` attribute on \`<sense>\` accepts the following values. You can also pass any custom string, which will be treated as a custom part of speech.\n\n`;

md += `### Universal\n\n`;
md += `| Code | Label |\n`;
md += `|------|-------|\n`;
for (const p of universalPos) {
  md += `| \`${variantToCode(p.variant)}\` | ${p.label} |\n`;
}
md += `\n`;

md += `### Japanese-specific\n\n`;
md += `| Code | Label |\n`;
md += `|------|-------|\n`;
for (const p of japanesePos) {
  md += `| \`${variantToCode(p.variant)}\` | ${p.label} |\n`;
}
md += `\n`;

// ---------------------------------------------------------------------------
// Write output
// ---------------------------------------------------------------------------

mkdirSync(dirname(outPath), { recursive: true });
writeFileSync(outPath, md, "utf-8");

console.log(
  `Generated schema reference -> ${outPath} (${elements.size} elements, ${allPos.length} POS codes)`
);
