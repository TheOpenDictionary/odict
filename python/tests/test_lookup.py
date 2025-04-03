import pytest
import tempfile
import os

from pathlib import Path
from theopendictionary import Dictionary


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
    return Dictionary.compile(dict1_path)


@pytest.fixture(scope="module")
def dict2(dict2_path):
    return Dictionary.compile(dict2_path)


def test_dictionary_path(dict1, dict1_path, dict2, dict2_path):
    assert dict1_path.replace(".xml", "") in dict1.path
    assert dict2_path.replace(".xml", "") in dict2.path


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

    Dictionary.write(xml_content, temp_path)

    try:
        assert os.path.exists(temp_path)

        dict_instance = Dictionary(temp_path)
        assert len(dict_instance.lookup("hello")) == 1

    finally:
        # Clean up the temporary file
        os.unlink(temp_path)

def test_index_and_search(dict1, snapshot):
    # Test indexing and searching
    dict1.index()
    results = dict1.search("run")
    assert results == snapshot  # Use snapshot testing
