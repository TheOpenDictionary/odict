from odict_uniffi import (
    Translation, 
    MediaUrl, 
    EnumWrapper, 
    Sense, 
    Etymology, 
    Entry
)

def test_translation_record():
    t = Translation(lang="en", value="test")
    assert t.lang == "en"
    assert t.value == "test"

def test_media_url_record():
    m = MediaUrl(src="http://test.com/img.png", mime_type="image/png", description="desc")
    assert m.src == "http://test.com/img.png"
    assert m.mime_type == "image/png"

def test_enum_wrapper_record():
    e = EnumWrapper(name="POS", variant="noun", value="Noun")
    assert e.name == "POS"
    assert e.variant == "noun"

def test_entry_deep_structure(dict1):
    results = dict1.lookup(["cat"], None)
    entry = results[0].entry
    assert isinstance(entry, Entry)
    assert entry.term == "cat"
    assert isinstance(entry.etymologies[0], Etymology)
    assert isinstance(entry.etymologies[0].senses[0], Sense)
    assert isinstance(entry.etymologies[0].senses[0].pos, EnumWrapper)
