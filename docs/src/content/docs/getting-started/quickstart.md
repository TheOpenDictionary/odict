---
title: Quick Start
description: Create, compile, and query your first ODict dictionary.
---

This guide walks you through creating a simple dictionary, compiling it, and querying it with the CLI.

## 1. Create a new dictionary

Use the `odict new` command to scaffold a blank XML file:

```bash
odict new animals -n "Animal Dictionary"
```

This creates `animals.xml`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<dictionary name="Animal Dictionary">
</dictionary>
```

## 2. Add entries

Open `animals.xml` and add some entries:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<dictionary name="Animal Dictionary">
  <entry term="cat">
    <ety description="From Latin cattus">
      <sense pos="n">
        <definition value="A small domesticated carnivorous mammal with soft fur">
          <example value="The cat sat on the mat." />
          <example value="She adopted two cats from the shelter." />
        </definition>
        <definition value="(informal) A person, especially a man">
          <example value="He's a cool cat." />
        </definition>
      </sense>
    </ety>
  </entry>

  <entry term="dog">
    <ety description="From Old English docga">
      <sense pos="n">
        <definition value="A domesticated carnivorous mammal kept as a pet or for work">
          <example value="The dog fetched the ball." />
        </definition>
      </sense>
      <sense pos="v">
        <definition value="To follow someone closely and persistently">
          <example value="Reporters dogged the politician." />
        </definition>
      </sense>
    </ety>
  </entry>

  <!-- Cross-reference: "kitty" redirects to "cat" -->
  <entry term="kitty" see="cat" />
</dictionary>
```

:::tip
The `see` attribute creates a cross-reference. When you look up "kitty", ODict can follow it to the "cat" entry.
:::

## 3. Compile the dictionary

```bash
odict compile animals.xml
```

This produces `animals.odict` — a compact binary file. You can inspect it with:

```bash
odict info animals.odict
```

```
Animal Dictionary
─────────────────

File Version: 3
File Size: 312 B
Entries: 3
```

## 4. Look up entries

```bash
odict lookup animals.odict cat
```

Output:

```
cat (From Latin cattus)

  noun
    1. A small domesticated carnivorous mammal with soft fur
       • "The cat sat on the mat."
       • "She adopted two cats from the shelter."
    2. (informal) A person, especially a man
       • "He's a cool cat."
```

### Follow cross-references

```bash
odict lookup animals.odict kitty -F 1
```

This follows the `see="cat"` redirect and returns the "cat" entry.

### JSON output

```bash
odict lookup animals.odict cat -f json
```

Returns full structured JSON, useful for integration with other tools.

## 5. Full-text search

To search across all definitions, first create an index:

```bash
odict index animals.odict
```

Then search:

```bash
odict search animals.odict "domesticated mammal"
```

This returns all entries whose definitions match the query.

:::note
You can also pass `--index` to `odict search` to auto-create the index on the fly.
:::

## 6. Serve over HTTP

Start a local server to query dictionaries via REST:

```bash
odict serve animals.odict -p 8080
```

Then query from any HTTP client:

```bash
# Lookup
curl "http://localhost:8080/animals/lookup?queries=cat,dog"

# Search
curl "http://localhost:8080/animals/search?query=domesticated"

# Tokenize
curl "http://localhost:8080/animals/tokenize?text=the+cat+and+the+dog"
```

## What's next?

- [XML Schema Reference](/schema/reference/) — learn the full XML format including pronunciations, notes, and groups
- [CLI Reference](/cli/reference/) — complete command-line documentation
- Language bindings: [Python](/api/python/), [JavaScript](/api/javascript/), [Rust](/api/rust/)
