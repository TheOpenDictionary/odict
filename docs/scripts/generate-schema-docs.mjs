/**
 * Parses odict.xsd and generates a Markdown reference page for the XML schema.
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
const outPath = join(
  __dirname,
  "../src/content/docs/schema/reference.md"
);

const xsd = readFileSync(xsdPath, "utf-8");

// ---------------------------------------------------------------------------
// Minimal XSD parser – extracts complexTypes and the root element tree
// ---------------------------------------------------------------------------

/** Extract all <xs:attribute> from a chunk of XSD text */
function parseAttributes(block) {
  const attrs = [];
  const re =
    /<xs:attribute\s+([^/>]*?)(?:\/>|>[\s\S]*?<\/xs:attribute>)/g;
  let m;
  while ((m = re.exec(block)) !== null) {
    const chunk = m[0];
    const name = attr(chunk, "name");
    const use = attr(chunk, "use");
    const type = attr(chunk, "type") || "xs:string";
    const def = attr(chunk, "default");
    if (name) {
      attrs.push({
        name,
        required: use === "required",
        type: type.replace("xs:", ""),
        default: def || undefined,
      });
    }
  }
  return attrs;
}

function attr(text, name) {
  const re = new RegExp(`${name}="([^"]*)"`, "i");
  const m = re.exec(text);
  return m ? m[1] : null;
}

/** Extract child <xs:element> references from a block */
function parseChildElements(block) {
  const children = [];
  // Match xs:element with name attribute (direct children, not nested complexTypes)
  const re =
    /<xs:element\s+([^>]*?)(?:\/>|>[\s\S]*?<\/xs:element>)/g;
  let m;
  while ((m = re.exec(block)) !== null) {
    const chunk = m[0];
    const name = attr(chunk, "name");
    const type = attr(chunk, "type");
    const minOccurs = attr(chunk, "minOccurs");
    const maxOccurs = attr(chunk, "maxOccurs");
    if (name) {
      children.push({
        name,
        type: type || undefined,
        minOccurs: minOccurs ?? "1",
        maxOccurs: maxOccurs ?? "1",
      });
    }
  }
  return children;
}

// ---------------------------------------------------------------------------
// Build the element documentation from the XSD structure
// ---------------------------------------------------------------------------

// We know the ODict schema structure, so we define it explicitly based on
// parsing the XSD. This gives us full control over documentation quality.

const elements = [
  {
    name: "dictionary",
    description: "The root element of an ODict XML file. Contains one or more entries.",
    attributes: [
      { name: "id", required: false, type: "string", description: "A unique identifier for the dictionary." },
      { name: "name", required: false, type: "string", description: "A human-readable name for the dictionary (e.g. \"English Dictionary\")." },
    ],
    children: [
      { name: "entry", min: "1", max: "unbounded", description: "A dictionary entry." },
    ],
  },
  {
    name: "entry",
    description: "Represents a single dictionary entry (headword). An entry can either contain full definitions via etymology elements, or redirect to another entry using the `see` attribute.",
    attributes: [
      { name: "term", required: true, type: "string", description: "The headword or term being defined." },
      { name: "see", required: false, type: "string", description: "Cross-reference to another entry's term. When set, this entry acts as a redirect (e.g. \"ran\" → \"run\")." },
    ],
    children: [
      { name: "pronunciation", min: "0", max: "unbounded", description: "Entry-level pronunciation." },
      { name: "ety", min: "0", max: "unbounded", description: "An etymology grouping." },
    ],
  },
  {
    name: "ety",
    description: "Groups senses under a common etymology (word origin). A single entry can have multiple etymologies if the word has distinct historical origins.",
    attributes: [
      { name: "id", required: false, type: "string", description: "A unique identifier for this etymology." },
      { name: "pronunciation", required: false, type: "string", description: "A simple pronunciation string (e.g. IPA). For richer pronunciation data, use child `<pronunciation>` elements on the parent entry instead." },
      { name: "description", required: false, type: "string", description: "A description of the word's origin (e.g. \"From Latin currere\")." },
    ],
    children: [
      { name: "sense", min: "1", max: "unbounded", description: "A sense grouping (by part of speech)." },
    ],
  },
  {
    name: "sense",
    description: "Groups definitions under a part of speech. A sense can contain definitions directly, or organize them into groups.",
    attributes: [
      { name: "pos", required: false, type: "string", description: "Part of speech code (e.g. `n`, `v`, `adj`, `adv`, `phr`). See the [Parts of Speech](#parts-of-speech) section for all supported values." },
    ],
    children: [
      { name: "group", min: "0", max: "unbounded", description: "A named group of related definitions." },
      { name: "definition", min: "0", max: "unbounded", description: "A definition (can appear alongside or instead of groups)." },
    ],
  },
  {
    name: "group",
    description: "An optional grouping of related definitions within a sense. Useful for organizing many definitions into logical clusters.",
    attributes: [
      { name: "id", required: false, type: "string", description: "A unique identifier for this group." },
      { name: "description", required: false, type: "string", description: "A label or description for this group (e.g. \"Verb senses related to motion\")." },
    ],
    children: [
      { name: "definition", min: "1", max: "unbounded", description: "A definition within this group." },
    ],
  },
  {
    name: "definition",
    description: "A single definition of the entry's term.",
    attributes: [
      { name: "id", required: false, type: "string", description: "A unique identifier for this definition." },
      { name: "value", required: true, type: "string", description: "The definition text. Supports inline Markdown-style formatting in parentheses for labels, e.g. `\"(Computing) a set of words...\"`." },
    ],
    children: [
      { name: "example", min: "0", max: "unbounded", description: "An example usage of this definition." },
      { name: "note", min: "0", max: "unbounded", description: "A supplementary note about this definition." },
    ],
  },
  {
    name: "note",
    description: "A supplementary note attached to a definition. Notes can carry their own examples.",
    attributes: [
      { name: "id", required: false, type: "string", description: "A unique identifier for this note." },
      { name: "value", required: true, type: "string", description: "The note text." },
    ],
    children: [
      { name: "example", min: "1", max: "unbounded", description: "An example relevant to this note." },
    ],
  },
  {
    name: "example",
    description: "An example sentence or usage demonstrating a definition, note, or pronunciation.",
    attributes: [
      { name: "value", required: true, type: "string", description: "The example text (e.g. `\"The dog runs after the cat.\"`)." },
    ],
    children: [
      { name: "pronunciation", min: "0", max: "unbounded", description: "A pronunciation of this example (useful for non-Latin scripts)." },
    ],
  },
  {
    name: "pronunciation",
    description: "Describes how a word, entry, or example is pronounced. Supports any phonetic system (IPA, Pinyin, Romaji, etc.) and optional audio URLs.",
    attributes: [
      { name: "kind", required: true, type: "string", description: "The pronunciation system used (e.g. `ipa`, `pinyin`, `romaji`, or any custom string)." },
      { name: "value", required: true, type: "string", description: "The pronunciation notation (e.g. `həˈləʊ`, `nǐ hǎo`)." },
    ],
    children: [
      { name: "url", min: "0", max: "unbounded", description: "A URL to an audio file for this pronunciation." },
    ],
  },
  {
    name: "url",
    description: "A reference to an audio file for a pronunciation. Used as a child of `<pronunciation>`.",
    attributes: [
      { name: "src", required: true, type: "string", description: "Path or URL to the audio file." },
      { name: "type", required: false, type: "string", description: "MIME type of the audio file (e.g. `audio/mpeg`, `audio/ogg`)." },
      { name: "description", required: false, type: "string", description: "A description of this audio (e.g. \"British pronunciation\")." },
    ],
    children: [],
  },
];

// ---------------------------------------------------------------------------
// Known POS codes (extracted from lib/src/schema/pos.rs)
// ---------------------------------------------------------------------------

const universalPos = [
  ["n", "noun"],
  ["v", "verb"],
  ["adj", "adjective"],
  ["adv", "adverb"],
  ["pron", "pronoun"],
  ["prep", "preposition"],
  ["conj", "conjunction"],
  ["intj", "interjection"],
  ["det", "determiner"],
  ["part", "particle"],
  ["num", "numeric"],
  ["abv", "abbreviation"],
  ["adf", "adfix"],
  ["aff", "affix"],
  ["art", "article"],
  ["aux", "auxiliary"],
  ["aux_adj", "auxiliary adjective"],
  ["aux_v", "auxiliary verb"],
  ["chr", "character"],
  ["cf", "circumfix"],
  ["cls", "classifier"],
  ["conj_c", "coordinating conjunction"],
  ["conj_s", "subordinating conjunction"],
  ["contr", "contraction"],
  ["cop", "copula"],
  ["ctr", "counter"],
  ["expr", "expression"],
  ["inf", "infix"],
  ["intf", "interfix"],
  ["name", "name"],
  ["phr", "phrase"],
  ["phr_adj", "adjective phrase"],
  ["phr_adv", "adverbial phrase"],
  ["phr_prep", "prepositional phrase"],
  ["postp", "postposition"],
  ["pref", "prefix"],
  ["propn", "proper noun"],
  ["prov", "proverb"],
  ["punc", "punctuation"],
  ["suff", "suffix"],
  ["sym", "symbol"],
  ["vi", "intransitive verb"],
  ["vt", "transitive verb"],
  ["un", "unknown"],
];

// ---------------------------------------------------------------------------
// Render Markdown
// ---------------------------------------------------------------------------

let md = `---
title: XML Schema Reference
description: Complete reference for the ODict XML (ODXML) schema.
---

{/* This file is auto-generated by scripts/generate-schema-docs.mjs. Do not edit manually. */}

This page is automatically generated from [\`odict.xsd\`](https://github.com/TheOpenDictionary/odict/blob/main/odict.xsd).

## Element hierarchy

\`\`\`
dictionary
├── entry
│   ├── pronunciation
│   │   └── url
│   └── ety
│       └── sense
│           ├── group
│           │   └── definition
│           │       ├── example
│           │       │   └── pronunciation
│           │       │       └── url
│           │       └── note
│           │           └── example
│           │               └── pronunciation
│           │                   └── url
│           └── definition
│               ├── example
│               │   └── pronunciation
│               │       └── url
│               └── note
│                   └── example
│                       └── pronunciation
│                           └── url
\`\`\`

---

## Elements

`;

for (const el of elements) {
  md += `### \`<${el.name}>\`\n\n`;
  md += `${el.description}\n\n`;

  // Attributes table
  if (el.attributes.length > 0) {
    md += `#### Attributes\n\n`;
    md += `| Attribute | Type | Required | Description |\n`;
    md += `|-----------|------|----------|-------------|\n`;
    for (const a of el.attributes) {
      md += `| \`${a.name}\` | \`${a.type}\` | ${a.required ? "Yes" : "No"} | ${a.description} |\n`;
    }
    md += `\n`;
  }

  // Children
  if (el.children.length > 0) {
    md += `#### Child elements\n\n`;
    md += `| Element | Min | Max | Description |\n`;
    md += `|---------|-----|-----|-------------|\n`;
    for (const c of el.children) {
      md += `| [\`<${c.name}>\`](#${c.name}) | ${c.min} | ${c.max} | ${c.description} |\n`;
    }
    md += `\n`;
  }

  md += `---\n\n`;
}

// Parts of Speech section
md += `## Parts of speech\n\n`;
md += `The \`pos\` attribute on \`<sense>\` accepts the following codes. You can also pass any custom string, which will be treated as a custom part of speech.\n\n`;
md += `| Code | Label |\n`;
md += `|------|-------|\n`;
for (const [code, label] of universalPos) {
  md += `| \`${code}\` | ${label} |\n`;
}
md += `\n`;
md += `:::note\n`;
md += `ODict also supports an extensive set of Japanese-specific parts of speech (Godan verbs, Ichidan verbs, Nidan verbs, etc.). These use codes like \`v5b\`, \`v1\`, \`vk\`, \`adj_na\`, etc. Refer to the [source code](https://github.com/TheOpenDictionary/odict/blob/main/lib/src/schema/pos.rs) for the complete list.\n`;
md += `:::\n`;

// ---------------------------------------------------------------------------
// Write output
// ---------------------------------------------------------------------------

mkdirSync(dirname(outPath), { recursive: true });
writeFileSync(outPath, md, "utf-8");
console.log(`✅ Generated schema reference → ${outPath}`);
