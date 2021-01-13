import odict


def test_lookup_entry():
    odict.compile_dictionary("examples/example1.xml")
    json = odict.lookup_entry("run", "examples/example1.odict")
    expected = "{\"term\":\"run\",\"etymologies\":[{\"id\":\"0\",\"usages\":{\"n\":{\"pos\":\"n\",\"definitions\":[\"Act or instance of running, of moving rapidly using the feet.\",\"Act or instance of hurrying (to or from a place) (not necessarily by foot); dash or errand, trip.\",\"A pleasure trip.\",\"Flight, instance or period of fleeing.\",\"Migration (of fish).\",\"A group of fish that migrate, or ascend a river for the purpose of spawning.\"]},\"v\":{\"pos\":\"v\",\"groups\":[{\"id\":\"\",\"description\":\"A number of verb usages\",\"definitions\":[\"(vertebrates) To move swiftly.\",\"(fluids) To flow.\",\"(nautical, of a vessel) To sail before the wind, in distinction from reaching or sailing close-hauled.\",\"(social) To carry out an activity.\",\"To extend or persist, statically or dynamically, through space or time.\",\"(transitive) To execute or carry out a plan, procedure or program.\"]}]}}}]}"
    assert json == expected, "json should be %s, received: %s" % (
        expected, json)


def test_search_dictionary():
    odict.compile_dictionary("examples/example1.xml")
    odict.index_dictionary("examples/example1.odict")
    json = odict.search_dictionary("run", "examples/example1.odict")
    expected = "[{\"term\":\"run\",\"etymologies\":[{\"id\":\"0\",\"usages\":{\"n\":{\"pos\":\"n\",\"definitions\":[\"Act or instance of running, of moving rapidly using the feet.\",\"Act or instance of hurrying (to or from a place) (not necessarily by foot); dash or errand, trip.\",\"A pleasure trip.\",\"Flight, instance or period of fleeing.\",\"Migration (of fish).\",\"A group of fish that migrate, or ascend a river for the purpose of spawning.\"]},\"v\":{\"pos\":\"v\",\"groups\":[{\"id\":\"\",\"description\":\"A number of verb usages\",\"definitions\":[\"(vertebrates) To move swiftly.\",\"(fluids) To flow.\",\"(nautical, of a vessel) To sail before the wind, in distinction from reaching or sailing close-hauled.\",\"(social) To carry out an activity.\",\"To extend or persist, statically or dynamically, through space or time.\",\"(transitive) To execute or carry out a plan, procedure or program.\"]}]}}}]}]"
    assert json == expected, "json should be %s, received: %s" % (
        expected, json)


def test_write_dictionary():
    xml = "<dictionary><entry term=\"hello\"><ety><usage pos=\"v\"><definition>hello world</definition></usage></ety></entry></dictionary>"
    odict.write_dictionary(xml, "test.odict")
    json = odict.lookup_entry("hello", "test.odict")
    expected = "{\"term\":\"hello\",\"etymologies\":[{\"id\":\"0\",\"usages\":{\"v\":{\"pos\":\"v\",\"definitions\":[\"hello world\"]}}}]}"
    assert json == expected, "json should be %s, received: %s" % (
        expected, json)


if __name__ == "__main__":
    test_lookup_entry()
    test_search_dictionary()
    test_write_dictionary()
