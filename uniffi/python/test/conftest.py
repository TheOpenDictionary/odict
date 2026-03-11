import pytest
import os
import sys
from pathlib import Path

# Ensure we can import odict_uniffi from parent dir
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from odict_uniffi import (
    dictionary_from_xml
)

@pytest.fixture(scope="module")
def examples_path():
    # Workspace root examples
    return Path(__file__).resolve().parent.parent.parent.parent / "examples"

@pytest.fixture(scope="module")
def dict1_xml(examples_path):
    with open(examples_path / "example1.xml", "r", encoding="utf-8") as f:
        return f.read()

@pytest.fixture(scope="module")
def dict2_xml(examples_path):
    with open(examples_path / "example2.xml", "r", encoding="utf-8") as f:
        return f.read()

@pytest.fixture(scope="module")
def dict3_xml(examples_path):
    with open(examples_path / "example3.xml", "r", encoding="utf-8") as f:
        return f.read()

@pytest.fixture(scope="module")
def dict1(dict1_xml):
    return dictionary_from_xml(dict1_xml)

@pytest.fixture(scope="module")
def dict2(dict2_xml):
    return dictionary_from_xml(dict2_xml)

@pytest.fixture(scope="module")
def dict3(dict3_xml):
    return dictionary_from_xml(dict3_xml)
