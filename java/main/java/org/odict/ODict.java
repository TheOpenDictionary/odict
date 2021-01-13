package org.odict;

import java.io.File;
import java.nio.file.Paths;

public class ODict {
  static {
    // Can't use loadLibrary() due to this dumb bug:
    // https://github.com/bazelbuild/bazel/issues/11082
    // so we're using a hardcoded filename (contents NOT guaranteed to be an actual
    // .so file)
    System.load(Paths.get(System.getProperty("user.dir"), "java", "main", "cpp", "libodict.so")
        .toAbsolutePath().toString());
  }

  public native static String lookupEntry(String term, String dictionaryPath);

  public native static String searchDictionary(String query, String dictionaryPath);

  public native static void indexDictionary(String dictionaryPath);

  public native static void compileDictionary(String path);

  public native static void writeDictionary(String xml, String outputPath);
}