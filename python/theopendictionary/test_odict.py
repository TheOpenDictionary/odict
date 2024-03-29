import json

from theopendictionary import Dictionary


def test_search_dictionary():
    Dictionary.compile("../examples/example1.xml")

    dict = Dictionary("../examples/example1.odict")

    dict.index()

    json = dict.search("run")

    expected = '[{"term":"run","etymologies":[{"id":"0","senses":{"n":{"pos":"n","definitions":["Act or instance of running, of moving rapidly using the feet.","Act or instance of hurrying (to or from a place) (not necessarily by foot); dash or errand, trip.","A pleasure trip.","Flight, instance or period of fleeing.","Migration (of fish).","A group of fish that migrate, or ascend a river for the purpose of spawning."]},"v":{"pos":"v","groups":[{"id":"","description":"A number of verb senses","definitions":["(vertebrates) To move swiftly.","(fluids) To flow.","(nautical, of a vessel) To sail before the wind, in distinction from reaching or sailing close-hauled.","(social) To carry out an activity.","To extend or persist, statically or dynamically, through space or time.","(transitive) To execute or carry out a plan, procedure or program."]}]}}}]}]'

    assert len(json) == 2, "there should only be 2 items in the array"

    assert json[0].get("term") == "ran", "json should be %s, received: %s" % (
        expected,
        json,
    )

    assert json[1].get("term") == "run", "json should be %s, received: %s" % (
        expected,
        json,
    )


def test_lexicon():
    xml = '<dictionary><entry term="hello"><ety><sense pos="v"><definition>hello world</definition></sense></ety></entry><entry term="world"><ety><sense pos="v"><definition>hello world</definition></sense></ety></entry></dictionary>'

    Dictionary.write(xml, "test.odict")

    dict = Dictionary("test.odict")

    output = dict.lexicon()

    expected = ["hello", "world"]

    assert output == expected, "lexicon should be %s, received: %s" % (
        expected,
        output,
    )


def test_write_lookup_dictionary():
    xml = '<dictionary><entry term="hello"><ety><sense pos="v"><definition>hello world</definition></sense></ety></entry></dictionary>'

    Dictionary.write(xml, "test.odict")

    dict = Dictionary("test.odict")

    output = dict.lookup("hello")

    expected = '[{"term":"hello","etymologies":[{"id":"0","senses":{"v":{"pos":"v","definitions":["hello world"]}}}]}]'

    assert len(output) == 1, "there should only be one result"

    assert output[0][0].get("term") == "hello", "json should be %s, received: %s" % (
        expected,
        json,
    )


def test_write_lookup_dictionary_no_markdown():
    Dictionary.compile("../examples/example2.xml")

    dict = Dictionary("../examples/example2.odict")

    expected1 = "This <strong>is</strong> a <em>markdown</em> test"
    expected2 = "This **is** a _markdown_ test"
    expected3 = "This is a markdown test"

    output1 = dict.lookup("markdown")
    output2 = dict.lookup("markdown", markdown="disable")
    output3 = dict.lookup("markdown", markdown="text")

    actual1 = (
        output1[0][0]
        .get("etymologies")[0]
        .get("senses")
        .get("v")
        .get("definitions")[0]
        .get("value")
    )

    actual2 = (
        output2[0][0]
        .get("etymologies")[0]
        .get("senses")
        .get("v")
        .get("definitions")[0]
        .get("value")
    )

    actual3 = (
        output3[0][0]
        .get("etymologies")[0]
        .get("senses")
        .get("v")
        .get("definitions")[0]
        .get("value")
    )

    assert actual1 == expected1, "definition should be %s, received: %s" % (
        expected1,
        actual1,
    )

    assert actual2 == expected2, "definition should be %s, received: %s" % (
        expected2,
        actual2,
    )

    assert actual3 == expected3, "definition should be %s, received: %s" % (
        expected3,
        actual3,
    )
