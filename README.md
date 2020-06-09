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
   
Full documentation available at https://odict.org.