from pyodict import sum_as_string


def test_write_lookup_dictionary():
    assert sum_as_string(1, 2) == "3"
    # xml = '<dictionary><entry term="hello"><ety><sense pos="v"><definition>hello world</definition></sense></ety></entry></dictionary>'

    # Dictionary.write(xml, "test.odict")

    # dict = Dictionary("test.odict")

    # output = dict.lookup("hello")

    # expected = '[{"term":"hello","etymologies":[{"id":"0","senses":{"v":{"pos":"v","definitions":["hello world"]}}}]}]'

    # assert len(output) == 1, "there should only be one result"

    # assert output[0][0].get("term") == "hello", "json should be %s, received: %s" % (
    #     expected,
    #     json,
    # )
