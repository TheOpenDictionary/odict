<div align="center">

<img src="https://raw.githubusercontent.com/odict/odict/master/logo.jpg" width="350" />
<br/><br/>

![Travis CI](https://img.shields.io/travis/odict/odict.svg)

</div>

The Open Dictionary Project (ODict for short), is an open-source alternative to proprietary dictionary file formats such
 as [Babylon](http://www.babylon-software.com/free-dictionaries/) and 
 [Apple Dictionaries](https://developer.apple.com/library/content/documentation/UserExperience/Conceptual/DictionaryServicesProgGuide/Introduction/Introduction.html). 
  Similar to Apple dictionaries, Open Dictionary files are converted from XML (currently in a draft specification described
  below) to compressed, serialized, bite-sized files. ODict is built on some of the fastest technologies to ensure maximum
  speed when performing lookups: Google Snappy (fastest compression), RapidXML (fastest XML parsing), and Flatbuffers 
  (fastest serialization). Entries are searched in log(*n*) time, [un-coincidentally the time complexity of an ordered map 
   lookup](https://google.github.io/flatbuffers/flatbuffers_guide_use_cpp.html).
   
## Schema / Specification
For detailed information on the raw XML schema ODict uses, or the specification of its compiled file
format, see [here](spec/SCHEMA.md) and [here](spec/SPEC.md).

## Building
*Please note that you must have CMake and `flatc` installed and should preferably be using a UNIX system*

Building is super simple due to a handy little build script. Just run:
  
```bash
$ ./init
```

This will automatically download all required dependencies and start the build.
To just rebuild the project, run:

```bash
$ ./init build
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