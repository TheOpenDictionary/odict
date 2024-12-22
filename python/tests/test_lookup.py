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
    # Check that the compiled dictionary path contains the original XML path with .odict extension
    assert dict1.path.replace(".odict", "") in dict1_path
    assert dict2.path.replace(".odict", "") in dict2_path


def test_lookup(dict1, snapshot):
    # Test basic lookup
    result = dict1.lookup("cat (cat)")
    assert (
        result == snapshot
    )  # You'll need to use a snapshot testing library like pytest-snapshot
