# CLI Reference

## Creating Dictionaries

To create a dictionary, you'll first need to write a dictionary using the ODict [markup language](odxml.md) and save it as an ".xml" file. Once you're confident your file is in the correct format, compiling it to an ".odict" dictionary is as simple as running:

```
$ odict c mydictionary.xml
```

The output file will always be a corresponding ".odict" dictionary that appears in the same directory as the source file.

## Searching Dictionaries

There are two ways to search a compiled .odict file: via a case-insensitive entry lookup, which outputs a single entry, or via a full text fuzzy search, which will output an array of matching entries.

Let's look at each respectively.

### Entry Lookup

Looking up entries is super-duper easy. Just run:

```
$ odict l mydictionary.odict "word"
```

and a full-bodied JSON object will print out if there is a match.

### Fuzzy Search

Fuzzy searching uses [Bleve](https://blevesearch.com/) under-the-hood, so an index of your dictionary is required before searching. Dictionary indexes are stored in a temporary directory that differs depending on your OS, so you'll need to re-index the library on any new device you decide to access the dictionary on. Indexing can take quite some time if you have a particularly large dictionary.

There are two ways to index a dictionary. You can index it ahead of time by running:

```
$ odict i mydictionary.odict
```

or you can index it before you run your first search query by passing an `-i` flag to the search command:

```
$ odict s -i mydictionary.odict "my query"
```

If you omit this flag, ODict will automatically use the correct index for the provided file if one already exists. Each .odict file has a unique identifier baked into it, so once you index a dictionary once, ODict will always know where to find that index in the future.

## Dumping Dictionaries

Often times while developing an ODict application, it may be helpful to understand the underlying structure of the dictionary at hand without picking through code. As a result, the ODict CLI has a `dump` command which can be used to convert a compiled binary back into a rough estimation of its [original XML](ODXML.md). I say rough estimation because the library might add back certain XML attributes or ID fields that were not present in the original document used to create the file.

Using `dump` is easy:

```
$ odict d mydictionary.xml outputfile.xml
```

## Merging Dictionaries

The ODict CLI also has the ability to merge two compiled dictionaries and blend their definitions together. This feature uses the [mergo](https://github.com/imdario/mergo) to merge the underlying dictionary structs and is currently not as customizable as it should be. Right now ODict performs a full merge, so you may wind up with an entry with duplicate definitions if your two dictionaries contain similar definitions for the same word.

However, you can always `dump` the merged file, edit it, then re-compile it.

To merge two dictionaries, run:

```
$ odict m mydictionary1.odict mydictionary2.odict output.odict
```
