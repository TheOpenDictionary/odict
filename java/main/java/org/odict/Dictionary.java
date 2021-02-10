package org.odict;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;
import cz.adamh.utils.NativeUtils;
import java.io.*;
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

  // private native String lookup(String term, String dictionary);

  // private native String search(String query, String dictionary);

  private native void index(String dictionaryPath);

  // private native String read(String path);

  private schema.Dictionary dict;

  private ObjectMapper mapper;

  private short version;

  public Dictionary(String path) throws IOException {
    this.dict = this.read(path);
    this.mapper = new ObjectMapper();
  }

  public String lookup(String term) throws JsonProcessingException {
    schema.Entry found = this.dict.entriesByKey(term);

    if (found == null) {
      found = this.dict.entriesByKey(term.toLowerCase());
    }

    return found != null ? this.mapper.writeValueAsString(new Entry(found)) : "{}";
  }

  public short getVersion() {
    return this.version;
  }

  private schema.Dictionary read(String filePath) throws IOException {

    BufferedInputStream stream = new BufferedInputStream(new FileInputStream(filePath));

    try {
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
      System.out.println(version);
      // Read in length of compressed data
      byte[] compressed_size_b = new byte[8];

      stream.read(compressed_size_b);

      long compressed_size = ByteBuffer.wrap(compressed_size_b).order(ByteOrder.LITTLE_ENDIAN).getLong();

      // Read in compressed data
      byte[] compressed = new byte[(int) compressed_size];
      System.out.println(compressed_size);
      stream.read(compressed);

      // Decompress data
      // byte[] uncompressed = Snappy.uncompress(compressed);

      this.version = version;

      // Convert to dictionary and return
      return schema.Dictionary.getRootAsDictionary(ByteBuffer.wrap(compressed));
    } finally {
      stream.close();
    }
  }
}