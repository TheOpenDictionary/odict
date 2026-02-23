import pytest
import tempfile
import os

from pathlib import Path
from theopendictionary import OpenDictionary, compile


@pytest.fixture(scope="module")
def dict1_path():
    current_file = Path(__file__).resolve()
    return str(current_file.parent.parent.parent / "examples" / "example1.xml")


@pytest.fixture(scope="module")
def dict2_path():
    current_file = Path(__file__).resolve()
    return str(current_file.parent.parent.parent / "examples" / "example2.xml")


@pytest.fixture(scope="module")
def dict1(dict1_path):
    # Read XML file and compile to bytes
    with open(dict1_path, "r") as f:
        xml_content = f.read()
    compiled_bytes = compile(xml_content)
    return OpenDictionary(compiled_bytes)


@pytest.fixture(scope="module")
def dict2(dict2_path):
    # Read XML file and compile to bytes
    with open(dict2_path, "r") as f:
        xml_content = f.read()
    compiled_bytes = compile(xml_content)
    return OpenDictionary(compiled_bytes)


def test_dictionary_creation(dict1, dict2):
    # Test that dictionaries were created successfully
    assert dict1 is not None
    assert dict2 is not None


def test_lookup(dict1, snapshot):
    result = dict1.lookup("cat")
    assert result[0].entry.term == "cat"
    assert result == snapshot


def test_lookup_no_split(dict1):
    # Test lookup without splitting
    result = dict1.lookup("catdog")
    assert len(result) == 0


def test_lookup_with_split(dict1, snapshot):
    # Test lookup with splitting
    result = dict1.lookup("catdog", split=3)
    assert result == snapshot  # Use snapshot testing


def test_lexicon(dict1):
    # Test lexicon retrieval
    result = dict1.lexicon()
    assert result == ["cat", "dog", "poo", "ran", "run"]


def test_write_raw_xml():
    # Test writing raw XML and creating a dictionary
    xml_content = """
    <dictionary>
        <entry term="hello">
            <ety>
                <sense pos="v">
                    <definition value="hello world" />
                </sense>
            </ety>
        </entry>
        <entry term="world">
            <ety>
                <sense pos="v">
                    <definition value="hello world" />
                </sense>
            </ety>
        </entry>
    </dictionary>
    """

    with tempfile.NamedTemporaryFile(suffix=".odict", delete=False) as temp_file:
        temp_path = temp_file.name

    # Compile XML and create dictionary
    compiled_bytes = compile(xml_content)
    dict_instance = OpenDictionary(compiled_bytes)

    # Save to temp file
    dict_instance.save(temp_path)

    try:
        assert os.path.exists(temp_path)
        assert len(dict_instance.lookup("hello")) == 1

    finally:
        # Clean up the temporary file
        os.unlink(temp_path)


def test_index_and_search(dict1, snapshot):
    # Test indexing and searching
    dict1.index()
    results = dict1.search("run")
    assert results == snapshot  # Use snapshot testing


def test_lookup_case_sensitive(dict1):
    # By default lookups should be case-sensitive
    result = dict1.lookup("CAT")
    assert len(result) == 0  # Shouldn't find "CAT" (only "cat" exists)


def test_lookup_case_insensitive(dict1):
    # Test case-insensitive lookup
    result = dict1.lookup("CAT", insensitive=True)
    assert len(result) == 1
    assert result[0].entry.term == "cat"

    # Test with mixed case
    result = dict1.lookup("DoG", insensitive=True)
    assert len(result) == 1
    assert result[0].entry.term == "dog"


def test_lookup_case_insensitive_with_follow(dict1):
    # Test case-insensitive lookup combined with follow option
    # (using high number for infinite following)
    result = dict1.lookup("RaN", follow=True, insensitive=True)
    assert len(result) == 1
    assert result[0].entry.term == "run"
    assert result[0].directed_from.term == "ran"


def test_lookup_follow_boolean_true(dict1):
    # Test follow=True (should be equivalent to u32::MAX)
    result = dict1.lookup("ran", follow=True)
    assert len(result) == 1
    assert result[0].entry.term == "run"
    assert result[0].directed_from.term == "ran"


def test_lookup_follow_boolean_false(dict1):
    # Test follow=False (should disable following)
    result = dict1.lookup("ran", follow=False)
    assert result[0].entry.term == "ran"


def test_lookup_follow_number(dict1):
    # Test follow with specific number
    result = dict1.lookup("ran", follow=True)
    assert len(result) == 1
    assert result[0].entry.term == "run"
    assert result[0].directed_from.term == "ran"
