from odict_uniffi import LookupOptions

def test_lookup_basic(dict1):
    results = dict1.lookup(["cat"], None)
    assert len(results) == 1
    assert results[0].entry.term == "cat"
    assert len(results[0].entry.etymologies) > 0

def test_lookup_multiple(dict1):
    results = dict1.lookup(["cat", "dog"], None)
    assert len(results) == 2
    assert results[0].entry.term == "cat"
    assert results[1].entry.term == "dog"

def test_lookup_case_sensitive(dict1):
    results = dict1.lookup(["CAT"], None)
    assert len(results) == 0

def test_lookup_case_insensitive(dict1):
    opts = LookupOptions(insensitive=True, follow=None, split=None)
    results = dict1.lookup(["CAT"], opts)
    assert len(results) == 1
    assert results[0].entry.term == "cat"

def test_lookup_follow(dict1):
    # "ran" redirects to "run" in example1.xml
    opts = LookupOptions(follow=True, insensitive=None, split=None)
    results = dict1.lookup(["ran"], opts)
    assert len(results) == 1
    assert results[0].entry.term == "run"
    assert results[0].directed_from.term == "ran"

def test_lookup_no_follow(dict1):
    opts = LookupOptions(follow=False, insensitive=None, split=None)
    results = dict1.lookup(["ran"], opts)
    assert len(results) == 1
    assert results[0].entry.term == "ran"
    assert results[0].directed_from is None

def test_lookup_split(dict1):
    opts = LookupOptions(split=3, follow=None, insensitive=None)
    results = dict1.lookup(["catdog"], opts)
    assert len(results) == 2
    assert results[0].entry.term == "cat"
    assert results[1].entry.term == "dog"
