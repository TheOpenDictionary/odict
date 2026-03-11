from odict_uniffi import dictionary_from_xml

def test_min_max_rank(dict1):
    # example1 has "run" with rank 100
    assert dict1.min_rank() == 100
    assert dict1.max_rank() == 100

def test_no_rank(dict2):
    assert dict2.min_rank() is None
    assert dict2.max_rank() is None

def test_mixed_rank():
    xml = """<?xml version="1.0" encoding="UTF-8"?>
<dictionary>
  <entry term="low" rank="10"><ety><sense pos="n"><definition value="L"/></sense></ety></entry>
  <entry term="high" rank="100"><ety><sense pos="n"><definition value="H"/></sense></ety></entry>
  <entry term="none"><ety><sense pos="n"><definition value="N"/></sense></ety></entry>
</dictionary>"""
    d = dictionary_from_xml(xml)
    assert d.min_rank() == 10
    assert d.max_rank() == 100
