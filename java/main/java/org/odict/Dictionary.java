package org.odict;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.io.*;
import java.nio.file.Paths;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import cz.adamh.utils.NativeUtils;
import org.xerial.snappy.Snappy;
import org.odict.models.Entry;

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

  // private native void index(String dictionary);

  // private native String read(String path);

  private schema.Dictionary dict;

  private ObjectMapper mapper;

  public Dictionary(String path) throws IOException {
    this.dict = this.read(path);
    this.mapper = new ObjectMapper();
  }

  public String lookup(String term) throws JsonProcessingException {
    schema.Entry found = this.dict.entriesByKey(term);
    System.out.print(term);
    System.out.println(found);
    return found != null ? this.mapper.writeValueAsString(new Entry(found)) : "{}";
  }

  private schema.Dictionary read(String filePath) throws IOException {
    BufferedInputStream stream = new BufferedInputStream(new FileInputStream(filePath));

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

    stream.close();

    // Decompress data
    byte[] uncompressed = Snappy.uncompress(compressed);

    // Convert to dictionary and return
    return schema.Dictionary.getRootAsDictionary(ByteBuffer.wrap(uncompressed));
  }
}