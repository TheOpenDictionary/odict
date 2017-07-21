# The Open Dictionary Project
![Travis CI](https://img.shields.io/travis/Nickersoft/odict.svg)

The Open Dictionary Project (ODict for short), is an open-source alternative to proprietary dictionary file formats such
 as [Babylon](http://www.babylon-software.com/free-dictionaries/) and 
 [Apple Dictionaries](https://developer.apple.com/library/content/documentation/UserExperience/Conceptual/DictionaryServicesProgGuide/Introduction/Introduction.html). 
  Similar to Apple dictionaries, Open Dictionary files are converted from XML (currently in a draft specification described
  below) to compressed, serialized, bite-sized files. ODict is built on some of the fastest technologies to ensure maximum
  speed when performing lookups: Google Snappy (fastest compression), RapidXML (fastest XML parsing), and Flatbuffers 
  (fastest serialization). Entries are searched in log(*n*) time, [un-coincidentally the time complexity of an ordered map 
   lookup](https://google.github.io/flatbuffers/flatbuffers_guide_use_cpp.html).
   
## Schema
ODict schemas are standard XML, however, the features describe below will most likely expand in the future. An example
of a working schema can be found in `example.xml`.

### The Dictionary Node
The base node of the schema is `<dictionary>`, which ODict will base all of its queries on. All other notes and subnodes 
should be contained in this node.

### The Entry Node
The `<entry>` denotes a dictionary entry for a word. Words are used as ordered keys, so there should only be one entry per
word. If there is not, ODict will prefer the most recently defined entry of the word. `<entry>` nodes can have the following
attributes:
- **term (required)**: the word described in the entry

### The Usage Node
`<usage>` nodes are children of entry nodes and denote a manner in which a term is used, usually grouped by
part of speech. 

For example, according to Wikitionary, the English word "run" can be used as a verb, but as a verb it can
mean "to move swiftly" or "to carry out a social activity". These are definitions of a given *usage* of the word. 

Usage nodes can take the following attributes:
- **pos (currently required)**: the part of speech associated with the word usage

### The Definition Nodes
`<definition>` nodes just contain raw definition text, and should appear in usage nodes only.
 
## Building
*Please note that you must have CMake and `flatc` installed and should preferably be using a UNIX system*

Building is super simple due to a handy little build script. Just run:
  
```bash
$ ./build.sh
```

The output library and executable should be found in `build/bin`.

## CLI Usage
The CLI offers two commands: `generate` and `lookup`. `generate` takes an XML file and outputs an ODict dictionary.
  `lookup` takes a term and a dictionary file and looks up the entry for the word, printing the result in JSON. Examples
  of these functions are as follows: 
  
```bash
$ ./odict generate ./mydict.xml                                                                                                                                             [00:35:40]
Wrote 739 bytes (compressed from 880) to ./mydict.odict
Completed in 0.000522 seconds

$ ./odict lookup "elephant" mydict.odict                                                                                                                                       [00:35:57]
Decompressed dictionary from 731 bytes to 880
Completed in 0.000195 seconds

{"usages":[{"pos":"noun","definitions":["an elp...
```

## API Usage
This is still being worked out. Check back soon!

## Known Issues / Roadmap
Currently, ODict does little to validate the input it's given, so it might throw some weird error if you give 
it weird input. Just play by the rules for now and you should be good :)