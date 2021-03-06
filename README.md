<div align="center">

<img src="https://raw.githubusercontent.com/odict/odict/master/logo.jpg" width="350" />
<br/><br/>

![Go](https://github.com/TheOpenDictionary/odict/workflows/Go/badge.svg?branch=master)

</div>

The Open Dictionary Project (ODict for short), is an open-source alternative to proprietary dictionary file formats Babylon and Apple Dictionaries. Similar to other dictionaries, Open Dictionary files are converted from XML to compressed binary files that can be easily indexed, searched, and modified using the Open Dictionary [CLI tool](cli.md) or via its open-source Go bindings.

## Motivation

So your first thought is most likely, why would I need this? Why would someone take the time to build this? Who gives a shit about dictionaries?

Turns out, a lot of people. Amazing products like [freedict](https://freedict.org), [Wiktionary](https://wiktionary.org), and [Linguee](https://linguee.com) are aimed at providing people with completely free dictionaries to aid NLP research, language learning, and just share knowledge. The only issue is... this data is either only accessible online, not machine readable, not well structured, or some combination of the three.

So... how does ODict help?

ODict was developed out of the need for a fast, readily available, and open solution to cross-platform dictionary formats such as Apple Dictionaries, Babylon, Dictd, StarDict and others. Current formats are specifically designed to work with specific applications (usually developed by the same company that made the format), and as a result are somewhat uni-directional (there is official documentation on how to write a dictionary in their format, but not on how to read one). This forces users to write dictionaries that only work with one specific dictionary app.

Certains formats, like StarDict or Slab, can be read by multiple dictionary apps, but like most dictionary formats are HTML-based. The data inside is not structured, and the HTML markup might be inconsistent between dictionaries.

_Wouldn't it be nice if there was a completely open-source format, with documentation on both reading and writing the format, that anyone could use in **any** dictionary app?_

That's where ODict comes in.

## Key Advantages

1. **ODict is open-source.** Yep. 100% available for anyone to help build or extend, and not to mention 100% free. Most development happens in the repository you're looking at right now!

2. **ODict is layout agnostic.** Because ODict is structured lexical data as opposed to a bunch of indexed HTML, you can display ODict dictionary entries in any way you want, without any special CSS selectors. Both the CLI and language-specific ODict bindings return JSON for all fuzzy search and entry lookups.

3. **ODict is tiny (relatively speaking).** Seeing we're not storing any large HTML text bodies and instead just store compressed binary data, ODict files can be expected to be smaller than your standard dictionary files.

## ODXML

ODict XML (ODXML) is the XML used to define and structure ODict dictionaries. All compiled ODict dictionaries originate from ODXML files, and can easily be reverted back to XML via the CLI [`dump` command](cli.md#dumping-dictionaries).

An example dictionary might look like this:

```xml
<!-- Dictionary Root -->
<dictionary name="My Dictionary">
  <!-- Entry -->
  <entry term="Doggo">
    <!-- Etymology -->
    <ety>
      <!-- Usage (typically determined by part-of-speech) -->
      <usage pos="n">
        <!-- Definition -->
        <definition>A cute pupper</definition>
        <!-- Definition Group -->
        <group description="Slang for dog">
          <definition>Common way of saying a dog is a cutie</definition>
        </group>
      </usage>
    </ety>
  </entry>
</dictionary>
```

For more info on writing ODXML, check out the [official specification](docs/odxml.md).

## Language Bindings

ODict can currently be used in [Go](docs/api.md#go), [Java](docs/api.md#java), and [Python](docs/api.md#python). If you're just interested in compiling or reading dictionaries without writing any code, you can just the official command-line tool below.

## CLI

The ODict command-line interface (CLI) is a Go program you can execute in any terminal to create, dump, merge, search, and index ODict dictionaries.

The CLI's primary distribution channel is currently through its [Homebrew](homebrew.sh) formula:

```
$ brew tap TheOpenDictionary/odict
$ brew install odict
```

While you most likely would interface with dictionaries via a language-specific library, the CLI exists as a convenience tool that can be used to help debug or rapidly produce new dictionaries. For full docs on the CLI, [see here](docs/cli.md).

## File Format (For Nerds)

> **NOTE:** You can probably skip this section unless you're trying to debug changes to this code-base, or are writing an ODict parser in a language not currently supported.

Compiled .odict files are relatively straightforward, as they utilize the [ODict Flatbuffer schema](../schema/schema.fbs).

The buffer generated by this schema take up over 90% of the compiled file, however, addition header information still exists. The table below illustrates the full breakdown of a compiled .odict file, in the order in which the values are written to the file.

All values written in Little Endian byte order.

| Name           | Type    | Bytes    | Description                                                                                             |
| -------------- | ------- | -------- | ------------------------------------------------------------------------------------------------------- |
| Signature      | char[6] | 6        | Signature for the ODict format. Assertions fail if this signature is missing. Should always be `ODICT`. |
| Version        | ushort  | 2        | Represents the major version of ODict with which the file was created.                                  |
| Content Length | long    | 8        | Size (in bytes) of the compressed content to read. Used in assertions to validate file length.          |
| Content        | []byte  | Variable | Snappy-compressed FlatBuffer object. Must be decompressed by Snappy before it can be used.              |

A design decision was made to keep the structural data of the ODict format as a cross-platform [Flatbuffers](https://google.github.io/flatbuffers/) schema as opposed to simply encoding a
Go struct so that the format could be used by anyone, even without necessarily using any of the core ODict libraries.

For an example of how the files are written, you can look at the official [Go code that does so](go/read.go).
