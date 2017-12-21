CLI Usage
---------

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