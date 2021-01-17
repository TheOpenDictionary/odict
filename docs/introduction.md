# Introduction

> Don't feel like reading the B.S. below and want to just jump right in? Have a look at our [quick start](quick-start.md) guide!

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

Still not sold? Well...

1. **ODict is open-source.** Yep. 100% available for anyone to help build or extend, and not to mention 100% free. Most development happens in the repository you're looking at right now!

2. **ODict is layout agnostic.** Because ODict is structured lexical data as opposed to a bunch of indexed HTML, you can display ODict dictionary entries in any way you want, without any special CSS selectors. Both the CLI and language-specific ODict bindings return JSON for all fuzzy search and entry lookups.

3. **ODict is tiny (relatively speaking).** Seeing we're not storing any large HTML text bodies and instead just store compressed binary data, ODict files can be expected to be smaller than your standard dictionary files.
