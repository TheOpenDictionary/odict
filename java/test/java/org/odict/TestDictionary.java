package org.odict;

import static java.lang.Thread.sleep;
import static org.junit.Assert.*;

import java.nio.ByteBuffer;
import java.nio.charset.StandardCharsets;

import org.odict.Dictionary;
import org.junit.Test;
import org.odict.models.Entry;

public class TestDictionary {
  @Test
  public void testLookup() throws Exception {
    Dictionary.compile("examples/example1.xml");
    Dictionary dict = new Dictionary("examples/example1.odict");
    Entry entry = dict.lookup("run");

    assertNotNull(entry);
    assertNotEquals(entry.toJSON(), "{}");
  }

  @Test
  public void testSearch() throws Exception {
    Dictionary.compile("examples/example1.xml");

    Dictionary dict = new Dictionary("examples/example1.odict");

    dict.index();

    String json = dict.search("run");

    assertTrue(json.length() > 2);
  }

  @Test
  public void testWrite() throws Exception {
    Dictionary.write(
        "<dictionary><entry term=\"hello\"><ety><usage pos=\"v\"><definition>hello world</definition></usage></ety></entry></dictionary>",
        "test.odict");

    Dictionary dict = new Dictionary("test.odict");
    Entry entry = dict.lookup("hello");

    assertNotNull(entry);
  }
}