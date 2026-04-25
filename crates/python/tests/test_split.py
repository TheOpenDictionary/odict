import pytest

from pathlib import Path
from theopendictionary import OpenDictionary, compile


@pytest.fixture(scope="module")
def dict1():
    current_file = Path(__file__).resolve()
    xml_path = current_file.parent.parent.parent / "examples" / "example1.xml"
    with open(xml_path, "r") as f:
        xml_content = f.read()
    compiled_bytes = compile(xml_content)
    return OpenDictionary(compiled_bytes)


def test_split_compound_into_words(dict1):
    result = dict1.split("catdog")
    assert len(result) == 2
    assert result[0].entry.term == "cat"
    assert result[1].entry.term == "dog"


def test_split_does_not_attempt_whole_word_lookup(dict1):
    # 'catdog' does not exist in the dictionary — lookup returns nothing
    lookup_result = dict1.lookup("catdog")
    assert len(lookup_result) == 0

    # split() finds the components regardless
    split_result = dict1.split("catdog")
    assert len(split_result) == 2


def test_split_single_word(dict1):
    result = dict1.split("cat")
    assert len(result) == 1
    assert result[0].entry.term == "cat"


def test_split_with_min_length(dict1):
    # min_length=4 means segments shorter than 4 chars are skipped
    # 'cat' is 3 chars, 'dog' is 3 chars — neither qualifies
    result = dict1.split("catdog", min_length=4)
    assert len(result) == 0


def test_split_with_numbers_in_text(dict1):
    # Non-dictionary characters between words should be skipped
    result = dict1.split("2cat8dog")
    assert len(result) == 2
    assert result[0].entry.term == "cat"
    assert result[1].entry.term == "dog"
