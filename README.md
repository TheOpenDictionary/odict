<div align="center">

<img src="https://raw.githubusercontent.com/odict/odict/master/logo.jpg" width="350" />
<br/><br/>

![Go](https://github.com/odict/odict/workflows/Go/badge.svg?branch=master)

</div>

The Open Dictionary Project (ODict for short), is an open-source alternative to proprietary dictionary file formats such
as [Babylon](http://www.babylon-software.com/free-dictionaries/) and
[Apple Dictionaries](https://developer.apple.com/library/content/documentation/UserExperience/Conceptual/DictionaryServicesProgGuide/Introduction/Introduction.html).

Similar to Apple dictionaries, Open Dictionary files are converted from XML (sometimes referred to as ODXML) to compressed, serialized, bite-sized files. Originally written in C++, ODict has since been ported to Go for portability and maintainability purposes. Each compiled dictionary consists some basic header information, as well as a [Snappy](https://github.com/google/snappy)-compressed [Flatbuffers](https://github.com/google/flatbuffers) schema that contains all of the dictionary's entries and definitions.

The ODict CLI uses [Bleve](https://github.com/blevesearch/bleve) to perform ad-hoc indexing on the local file system for rapid full-text searching of entries. ODict has a number of sister repos of varying completeness. As of this writing, there is a pretty comprehensive [Java port](https://github.com/odict/odict-java) of the project as well as an example of how to use the ODict CGo extension [in Python](https://github.com/odict/freedict/blob/master/odictlib.py).

Full documentation available at https://odict.org.
