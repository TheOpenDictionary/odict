Building
--------

### Requirements ###
To build `odict`, you must have Facebook's [Buck build tool](https://buckbuild.com/setup/getting_started.html) installed.
Additionally, you'll need a recent version of [CMake](https://cmake.org/cmake/), Google's
 [flatc](https://google.github.io/flatbuffers/flatbuffers_guide_using_schema_compiler.html) compiler (instructions on
  how to install can be found [here](https://rwinslow.com/posts/how-to-install-flatbuffers/)),  and perhaps some additional
  libraries, depending on any warnings or errors you get while building.
  
### Generating The Schema ###

While this repo contains a pre-generated schema header file, if you make any modification to `schema.fbs` you will have 
to re-generate this header file before committing your changes. To do so, use the Flatbuffers `flatc` compiler:

```bash
$ flatc -c -o src src/schema.fbs
```

### Running Buck ###

To build using Buck, just run in build script in the root directory:
  
```bash
$ ./build.sh
```

This will output `libodict.a` and the `odict` executable in ./bin. Please note that
building has own been tested on OSX. While it should compile without issue on other Unix 
systems, there is currently no way to compile on Windows.