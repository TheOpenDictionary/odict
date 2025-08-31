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


@pytest.fixture(scope="module")
def mixed_rank_dict():
    """Create a dictionary with mixed rank entries for testing."""
    mixed_xml = """<?xml version="1.0" encoding="UTF-8"?>
<dictionary>
  <entry term="high">
    <ety>
      <pos>noun</pos>
      <def>High ranking</def>
    </ety>
  </entry>
  <entry term="medium" rank="50">
    <ety>
      <pos>noun</pos>
      <def>Medium ranking</def>
    </ety>
  </entry>
  <entry term="low" rank="10">
    <ety>
      <pos>noun</pos>
      <def>Low ranking</def>
    </ety>
  </entry>
  <entry term="highest" rank="100">
    <ety>
      <pos>noun</pos>
      <def>Highest ranking</def>
    </ety>
  </entry>
</dictionary>"""

    with tempfile.NamedTemporaryFile(mode="w", suffix=".xml", delete=False) as f:
        f.write(mixed_xml)
        temp_xml_path = f.name

    try:
        # Read the XML content and compile it
        with open(temp_xml_path, "r") as f:
            xml_content = f.read()

        compiled_bytes = compile(xml_content)
        dict_obj = OpenDictionary(compiled_bytes)
        yield dict_obj
    finally:
        # Clean up temp files
        os.unlink(temp_xml_path)


def test_min_rank_with_ranks(dict1):
    """Test min_rank returns correct value for dictionary with ranks."""
    # example1 has one entry with rank=100 (the "run" entry)
    assert dict1.min_rank == 100


def test_max_rank_with_ranks(dict1):
    """Test max_rank returns correct value for dictionary with ranks."""
    # example1 has one entry with rank=100 (the "run" entry)
    assert dict1.max_rank == 100


def test_min_rank_without_ranks(dict2):
    """Test min_rank returns None for dictionary without ranks."""
    # example2 has no rank attributes
    assert dict2.min_rank is None


def test_max_rank_without_ranks(dict2):
    """Test max_rank returns None for dictionary without ranks."""
    # example2 has no rank attributes
    assert dict2.max_rank is None


def test_mixed_entries_with_and_without_ranks(mixed_rank_dict):
    """Test min_rank and max_rank with mixed entries."""
    assert mixed_rank_dict.min_rank == 10
    assert mixed_rank_dict.max_rank == 100


def test_rank_properties_are_getters(dict1):
    """Test that min_rank and max_rank are properties (getters)."""
    # Verify they can be accessed as properties
    min_val = dict1.min_rank
    max_val = dict1.max_rank

    # Verify they return consistent values on multiple accesses
    assert dict1.min_rank == min_val
    assert dict1.max_rank == max_val

    # Verify they are the same for this dictionary
    assert min_val == max_val == 100


def test_rank_values_type(mixed_rank_dict):
    """Test that rank values are returned as proper types."""
    min_rank = mixed_rank_dict.min_rank
    max_rank = mixed_rank_dict.max_rank

    assert isinstance(min_rank, int)
    assert isinstance(max_rank, int)
    assert min_rank >= 0
    assert max_rank >= 0
    assert min_rank <= max_rank
