package org.odict;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

import java.nio.ByteBuffer;
import org.odict.Dictionary;
import org.junit.Test;

public class TestDictionary {
  @Test
  public void testLookup() throws Exception {
    Dictionary.compile("examples/example1.xml");

    Dictionary dict = new Dictionary("examples/example1.odict");
    String json = dict.lookup("run");
    String expected = "{\"term\":\"run\",\"etymologies\":[{\"id\":\"0\",\"usages\":{\"v\":{\"pos\":\"v\",\"groups\":[{\"id\":\"0\",\"description\":\"A number of verb usages\",\"definitions\":[\"(vertebrates) To move swiftly.\",\"(fluids) To flow.\",\"(nautical, of a vessel) To sail before the wind, in distinction from reaching or sailing close-hauled.\",\"(social) To carry out an activity.\",\"To extend or persist, statically or dynamically, through space or time.\",\"(transitive) To execute or carry out a plan, procedure or program.\"]}]},\"n\":{\"pos\":\"n\",\"definitions\":[\"Act or instance of running, of moving rapidly using the feet.\",\"Act or instance of hurrying (to or from a place) (not necessarily by foot); dash or errand, trip.\",\"A pleasure trip.\",\"Flight, instance or period of fleeing.\",\"Migration (of fish).\",\"A group of fish that migrate, or ascend a river for the purpose of spawning.\"]}},\"description\":\"Latin root\"}]}";

    assertEquals(expected, json);
  }

  // @Test
  // public void testSearch() throws Exception {
  // Dictionary.compile("examples/example1.xml");

  // Dictionary dict = new Dictionary("examples/example1.odict");

  // dict.index();

  // String json = dict.search("run");

  // assertTrue(json.length() > 2);
  // }

  @Test
  public void testWrite() throws Exception {
    Dictionary.write(
        "<dictionary><entry term=\"hello\"><ety><usage pos=\"v\"><definition>hello world</definition></usage></ety></entry></dictionary>",
        "test.odict");

    Dictionary dict = new Dictionary("test.odict");
    String json = dict.lookup("hello");

    assertEquals(
        "{\"term\":\"hello\",\"etymologies\":[{\"id\":\"0\",\"usages\":{\"v\":{\"pos\":\"v\",\"definitions\":[\"hello world\"]}}}]}",
        json);
  }
}