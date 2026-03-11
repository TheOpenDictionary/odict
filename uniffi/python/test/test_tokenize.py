def test_tokenize_basic(dict3):
    tokens = dict3.tokenize("你好！你是谁？", None, None)
    assert len(tokens) > 0
    assert tokens[0].lemma == "你好"
    assert isinstance(tokens[0].start, int)

def test_tokenize_case_insensitive(dict1):
    tokens = dict1.tokenize("DOG cat", False, True)
    assert len(tokens) == 2
    assert tokens[0].lemma == "DOG"
    assert len(tokens[0].entries) == 1
    assert tokens[0].entries[0].entry.term == "dog"

def test_tokenize_japanese(dict1):
    tokens = dict1.tokenize("今日は良い天気です", None, None)
    assert len(tokens) > 0
    lemmas = [t.lemma for t in tokens]
    assert "今日" in lemmas
    for t in tokens:
        assert t.language == "jpn"
