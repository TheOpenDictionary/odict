package org.odict;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

import org.odict.ODict;
import org.junit.Test;

public class TestODict {
  @Test
  public void testLookupEntry() throws Exception {
    ODict.compileDictionary("examples/example1.xml");
    String json = ODict.lookupEntry("run", "examples/example1.odict");
    String expected = "{\"term\":\"run\",\"etymologies\":[{\"id\":\"0\",\"usages\":{\"n\":{\"pos\":\"n\",\"definitions\":[\"Act or instance of running, of moving rapidly using the feet.\",\"Act or instance of hurrying (to or from a place) (not necessarily by foot); dash or errand, trip.\",\"A pleasure trip.\",\"Flight, instance or period of fleeing.\",\"Migration (of fish).\",\"A group of fish that migrate, or ascend a river for the purpose of spawning.\"]},\"v\":{\"pos\":\"v\",\"groups\":[{\"id\":\"\",\"description\":\"A number of verb usages\",\"definitions\":[\"(vertebrates) To move swiftly.\",\"(fluids) To flow.\",\"(nautical, of a vessel) To sail before the wind, in distinction from reaching or sailing close-hauled.\",\"(social) To carry out an activity.\",\"To extend or persist, statically or dynamically, through space or time.\",\"(transitive) To execute or carry out a plan, procedure or program.\"]}]}}}]}";
    assertEquals(json, expected);
  }

  @Test
  public void testSearch() throws Exception {
    ODict.compileDictionary("examples/example1.xml");
    ODict.indexDictionary("examples/example1.odict");
    String json = ODict.searchDictionary("run", "examples/example1.odict");
    assertTrue(json.length() > 2);
  }

  @Test
  public void testWrite() throws Exception {
    ODict.writeDictionary(
        "<dictionary><entry term=\"hello\"><ety><usage pos=\"v\"><definition>hello world</definition></usage></ety></entry></dictionary>",
        "test.odict");

    String json = ODict.lookupEntry("hello", "test.odict");

    assertEquals(json,
        "{\"term\":\"hello\",\"etymologies\":[{\"id\":\"0\",\"usages\":{\"v\":{\"pos\":\"v\",\"definitions\":[\"hello world\"]}}}]}");
  }
}