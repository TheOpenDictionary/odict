from odict_uniffi import compile, dictionary_from_bytes, dictionary_from_xml

def test_compile(dict1_xml):
    compiled_bytes = compile(dict1_xml)
    assert isinstance(compiled_bytes, bytes)
    assert len(compiled_bytes) > 0

def test_from_bytes(dict1_xml):
    compiled_bytes = compile(dict1_xml)
    dict_obj = dictionary_from_bytes(compiled_bytes)
    assert dict_obj is not None
    assert "cat" in dict_obj.lexicon()

def test_from_xml(dict1_xml):
    dict_obj = dictionary_from_xml(dict1_xml)
    assert dict_obj is not None
    assert "cat" in dict_obj.lexicon()
