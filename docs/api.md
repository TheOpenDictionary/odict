# API

Currently, it is only possible to use the Java and Python bindings from another Bazel project, as the ODict JAR is not yet on Maven Central and Python's dependency on the shared ODict library makes it difficult to distribute through `pip`. Fortunately, setting up ODict in another Bazel project is easy.

Just add the following to your `WORKSPACE` file:

```bazel
http_archive(
    name = "odict",
    sha256 = "b58fd3432a6f84865c67a16ef6718be12ecd6b9b32c12dfd917c0a899807062f",
    strip_prefix = "odict-1.4",
    url = "https://github.com/TheOpenDictionary/odict/archive/1.4.tar.gz",
)

load("@odict//bazel:odict_deps.bzl", "odict_deps")

odict_deps()

load("@odict//bazel:odict_extra_deps.bzl", "odict_extra_deps")

odict_extra_deps()
```

then require either `@odict//java` or `@odict//python` in your respective Bazel rules.

## Java

While a [standalone Java client](https://github.com/TheOpenDictionary/odict-java) for ODict _used_ to exist, it has since been superseded by a Java binding that uses the ODict core's cgo dynamic library that lives in this repo.

Fortunately, the new ODict Java interface is extremely easy to use and will always stay up-to-date with the latest upstream changes to the ODict format.

```java
// Import statement
import org.odict.Dictionary;

void main() {
  // Compile a dictionary
  Dictionary.compile("path/to/file");

  // Write a new dictionary
  Dictionary.write("an XML string", "path/to/output.odict");

  // Load a dictionary
  Dictionary dict = new Dictionary("path/to/dictionary.odict");

  // Lookup an entry by word
  System.out.println(dict.lookup("giraffe"));

  // Index the dictionary
  dict.index();

  // Perform a fuzzy-search
  System.out.println(dict.search("full text"));
}
```

## Python

The Python interface for ODict is similar to that of Java:

```python
# Import statement
from python.odict import Dictionary

def main():
  # Compile a dictionary
  Dictionary.compile("path/to/file")

  # Write a new dictionary
  Dictionary.write("an XML string", "path/to/output.odict")

  # Load a dictionary
  dict = Dictionary("path/to/dictionary.odict")

  # Lookup an entry by word
  print(dict.lookup("giraffe"))

  # Index the dictionary
  dict.index()

  # Perform a fuzzy-search
  print(dict.search("full text"))
```
