package org.odict;

import static org.junit.Assert.assertEquals;

import org.odict.ODict;
import org.junit.Test;

public class TestODict {
  @Test
  public void testLookupEntry() throws Exception {
    ODict.compileDictionary("examples/example1.xml");
    // ByteArrayOutputStream out = new ByteArrayOutputStream();
    // Greeter.out = new PrintStream(out);
    // Greeter.main();
    // assertEquals("Hello world", new String(out.toByteArray(),
    // StandardCharsets.UTF_8).trim());
  }
}