# Using ODict in Java

While a [standalone Java client](https://github.com/TheOpenDictionary/odict-java) for ODict _used_ to exist, it has since been superseded by a Java binding that uses the ODict core's cgo dynamic library that lives in this repo.

Due to the Java client's dependency on this library file, it is pretty difficult to distribute through Maven, so currently it is only accessible for projects that also rely on the [Bazel](https://bazel.build) build system.

Fortunately, setting up ODict with Bazel is easy.
