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

  private native String lookupEntry(String term, String dictionaryPath);

  private native String search(String query, byte[] bytes);

  private native void index(byte[] bytes);

  private native byte[] read(String path);

  private byte[] bytes;

  public Dictionary(String path) {
    this(path, false);
  }

  public Dictionary(String path, Boolean shouldIndex) {
    this.bytes = this.read(path);

    if (shouldIndex) {
      this.index(this.bytes);
    }
  }

  // public String lookup(String term) {

  // }

  public void index() {
    this.index(this.bytes);
  }

  public String search(String query) {
    return this.search(query, this.bytes);
  }
}