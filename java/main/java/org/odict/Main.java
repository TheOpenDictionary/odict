package org.odict;

import java.io.IOException;
import java.nio.file.Paths;

public class Main {
  public static void main(String[] args) throws IOException {
    System.out.println(new ODict().searchDictionary("hello", "/Users/tnickerson/base/freedict/dictionaries/eng-jpn.odict"));
  }
}
