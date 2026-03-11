from odict_uniffi import IndexOptions, SearchOptions

def test_index_and_search(dict1):
    dict1.index(IndexOptions(overwrite=True))
    results = dict1.search("run", None)
    assert len(results) > 0
    assert any(r.term == "run" for r in results)

def test_search_limit(dict1):
    dict1.index(IndexOptions(overwrite=True))
    opts = SearchOptions(limit=1)
    results = dict1.search("run", opts)
    assert len(results) == 1
