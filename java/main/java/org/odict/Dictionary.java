package org.odict;

import java.io.File;
import java.nio.file.Paths;
import java.nio.ByteBuffer;

public class Dictionary {
  static {
    // Can't use loadLibrary() due to this dumb bug:
    // https://github.com/bazelbuild/bazel/issues/11082
    // so we're using a hardcoded filename (contents NOT guaranteed to be an actual
    // .so file)
    String libPath = Paths.get(System.getProperty("user.dir"), "java", "main", "cpp", "libodict.so").toAbsolutePath()
        .toString();
    System.load(libPath);
  }

  public native static void compile(String path);

  public native static void write(String xml, String outputPath);

  private native String lookup(String term, String dictionary);

  private native String search(String query, String dictionary);

  private native void index(String dictionary);

  private native String read(String path);

  private String encodedDictionary;

  public Dictionary(String path) {
    this(path, false);
  }

  public Dictionary(String path, Boolean shouldIndex) {
    this.encodedDictionary = this.read(path);

    if (shouldIndex) {
      this.index();
    }
  }

  public String lookup(String term) {
    return this.lookup(term, this.encodedDictionary);
  }

  public void index() {
    this.index(this.encodedDictionary);
  }

  public String search(String query) {
    return this.search(query, this.encodedDictionary);
  }
}