package org.odict;

import com.fasterxml.jackson.databind.ObjectMapper;
import cz.adamh.utils.NativeUtils;
import java.io.*;
import java.nio.charset.StandardCharsets;
import java.nio.file.Paths;
import java.util.HashMap;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import org.odict.models.Entry;
import org.xerial.snappy.Snappy;

import java.util.*;

public class Dictionary {
  static {
    try {
      NativeUtils.loadLibraryFromJar("/main/cpp/libodict.so");
    } catch (IOException e) {
      e.printStackTrace();
    }
  }

  public native static void compile(String path);

  public native static void write(String xml, String outputPath);

  private native String lookup(String term, String dictionaryID);

  private native String search(String query, String dictionaryID);

  private native void index(String dictionaryPath, boolean force);

  private String dictID;

  private String path;

  private short version;

  private schema.Dictionary dict;

  public Dictionary(String path) throws IOException {
    this(path, false);
  }

  public Dictionary(String path, Boolean index) throws IOException {
    this.path = path;
    this.dict = this.read(path);
    this.dictID = dict.id();

    if (index) {
      this.index(false);
    }
  }

  public Entry lookup(String term) {
    System.out.println(Arrays.toString(term.getBytes(StandardCharsets.UTF_8)));
    schema.Entry entry = this.dict.entriesByKey(term.toLowerCase());

    if (entry == null) {
      return null;
    }

    return new Entry(entry);
  }

  public short getVersion() {
    return this.version;
  }

  public void index() {
    this.index(false);
  }

  public void index(boolean force) {
    this.index(this.path, force);
  }

  public String search(String query) {
    return this.search(query, this.dictID);
  }

  private schema.Dictionary read(String filePath) throws IOException {
    try (FileInputStream fis = new FileInputStream(filePath)) {
      try (BufferedInputStream stream = new BufferedInputStream(fis)) {
        // Read in signature and validate it
        byte[] signature = new byte[5];

        stream.read(signature, 0, 5);

        // Validate file signature
        if (!new String(signature).equals("ODICT")) {
          throw new Error("Invalid ODict file signature");
        }

        // Read in version number
        byte[] version_b = new byte[2];

        stream.read(version_b);

        short version = ByteBuffer.wrap(version_b).order(ByteOrder.LITTLE_ENDIAN).getShort();

        // Read in length of compressed data
        byte[] compressed_size_b = new byte[8];

        stream.read(compressed_size_b);

        long compressed_size = ByteBuffer.wrap(compressed_size_b).order(ByteOrder.LITTLE_ENDIAN).getLong();

        // Read in compressed data
        byte[] compressed = new byte[(int) compressed_size];

        stream.read(compressed);

        // Decompress data
        byte[] uncompressed = Snappy.uncompress(compressed);

        this.version = version;

        // Convert to dictionary and return
        return schema.Dictionary.getRootAsDictionary(ByteBuffer.wrap(uncompressed));
      }
    }
  }
}