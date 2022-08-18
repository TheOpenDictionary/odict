from python.odict import Dictionary


def test_lookup_entry():
    Dictionary.compile("examples/example1.xml")

    dict = Dictionary("examples/example1.odict")

    json = dict.lookup("run")

    expected = '{"term":"run","etymologies":[{"id":"0","usages":{"n":{"pos":"n","definitions":["Act or instance of running, of moving rapidly using the feet.","Act or instance of hurrying (to or from a place) (not necessarily by foot); dash or errand, trip.","A pleasure trip.","Flight, instance or period of fleeing.","Migration (of fish).","A group of fish that migrate, or ascend a river for the purpose of spawning."]},"v":{"pos":"v","groups":[{"id":"","description":"A number of verb usages","definitions":["(vertebrates) To move swiftly.","(fluids) To flow.","(nautical, of a vessel) To sail before the wind, in distinction from reaching or sailing close-hauled.","(social) To carry out an activity.","To extend or persist, statically or dynamically, through space or time.","(transitive) To execute or carry out a plan, procedure or program."]}]}}}]}'

    assert json == expected, "json should be %s, received: %s" % (expected, json)


def test_search_dictionary():
    Dictionary.compile("examples/example1.xml")

    dict = Dictionary("examples/example1.odict")

    dict.index()

    json = dict.search("run")

    expected = '[{"term":"run","etymologies":[{"id":"0","usages":{"n":{"pos":"n","definitions":["Act or instance of running, of moving rapidly using the feet.","Act or instance of hurrying (to or from a place) (not necessarily by foot); dash or errand, trip.","A pleasure trip.","Flight, instance or period of fleeing.","Migration (of fish).","A group of fish that migrate, or ascend a river for the purpose of spawning."]},"v":{"pos":"v","groups":[{"id":"","description":"A number of verb usages","definitions":["(vertebrates) To move swiftly.","(fluids) To flow.","(nautical, of a vessel) To sail before the wind, in distinction from reaching or sailing close-hauled.","(social) To carry out an activity.","To extend or persist, statically or dynamically, through space or time.","(transitive) To execute or carry out a plan, procedure or program."]}]}}}]}]'

    assert json == expected, "json should be %s, received: %s" % (expected, json)


def test_write_dictionary():
    xml = '<dictionary><entry term="hello"><ety><usage pos="v"><definition>hello world</definition></usage></ety></entry></dictionary>'

    Dictionary.write(xml, "test.odict")

    dict = Dictionary("test.odict")

    json = dict.lookup("hello")

    expected = '{"term":"hello","etymologies":[{"id":"0","usages":{"v":{"pos":"v","definitions":["hello world"]}}}]}'

    assert json == expected, "json should be %s, received: %s" % (expected, json)
