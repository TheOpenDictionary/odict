import pytest
from pathlib import Path
from theopendictionary import Dictionary


@pytest.fixture(scope="module")
def dict3_path():
    current_file = Path(__file__).resolve()
    return str(current_file.parent.parent.parent / "examples" / "example3.xml")


@pytest.fixture(scope="module")
def dict3(dict3_path):
    return Dictionary.compile(dict3_path)


def test_tokenize(dict3, snapshot):
    # Test tokenization similar to the Node.js test
    tokens = dict3.tokenize("你好！你是谁？")
    
    # Verify we got some tokens
    assert len(tokens) > 0
    
    # Check specific token values
    assert tokens[0].lemma == "你好"
    assert tokens[0].entries[0].entry.term == "你"
    assert tokens[0].entries[1].entry.term == "好"
    
    # Use snapshot testing for the full result
    assert tokens == snapshot
