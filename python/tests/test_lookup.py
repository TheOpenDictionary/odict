import pytest

from pathlib import Path
from pyodict import Dictionary


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
    assert dict1.path.replace(".odict", "") in dict1_path
    assert dict2.path.replace(".odict", "") in dict2_path


def test_lookup(dict1, snapshot):
    result = dict1.lookup("cat (cat)")
    assert result == snapshot


def test_lookup_no_split(dict1):
    # Test lookup without splitting
    result = dict1.lookup("catdog")
    assert len(result[0]) == 0


def test_lookup_with_split(dict1, snapshot):
    # Test lookup with splitting
    result = dict1.lookup("catdog", split=3)
    assert result == snapshot  # Use snapshot testing


def test_lexicon(dict1):
    # Test lexicon retrieval
    result = dict1.lexicon()
    assert result == ["cat", "dog", "poo", "ran", "run"]
