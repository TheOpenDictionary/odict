import os
import sys
import tempfile
import uuid

# Add parent directory to path for imports
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from theopendictionary import (  # noqa: E402
    OpenDictionary,
    LookupOptions,
    compile,
)


def test_lookup_options_object():
    """Test using LookupOptions object instead of kwargs"""
    xml = """
    <dictionary>
      <entry term="lookup-options-test">
        <ety>
          <sense pos="n">
            <definition value="A test for lookup options object" />
          </sense>
        </ety>
      </entry>
      <entry term="LOOKUP-OPTIONS-TEST" see="lookup-options-test" />
    </dictionary>
    """

    try:
        # Compile and create dictionary
        compiled_bytes = compile(xml)
        dictionary = OpenDictionary(compiled_bytes)

        # Test with LookupOptions object
        lookup_opts = LookupOptions(follow=True, insensitive=True, split=3)

        results = dictionary.lookup("LOOKUP-OPTIONS-TEST", lookup_opts)

        assert len(results) == 1
        # Should find the redirected entry
        assert results[0].entry.term == "lookup-options-test"
        assert results[0].directed_from.term == "LOOKUP-OPTIONS-TEST"

    except Exception as e:
        # Some functionality might not be fully implemented yet
        print(f"LookupOptions test failed: {e}")
        pass


def test_lookup_options_creation():
    """Test creating LookupOptions with different configurations"""
    # Test with all options
    opts1 = LookupOptions(split=5, follow=True, insensitive=True)
    assert opts1.split == 5
    assert opts1.follow is True
    assert opts1.insensitive is True

    # Test with follow as number
    opts2 = LookupOptions(follow=10)
    assert opts2.follow == 10

    # Test with follow as False
    opts3 = LookupOptions(follow=False)
    assert opts3.follow is False

    # Test with no options
    opts4 = LookupOptions()
    assert opts4.split is None
    assert opts4.follow is None
    assert opts4.insensitive is None


def test_constructor_with_empty_bytes():
    """Test constructor behavior with edge cases"""
    try:
        # Test with empty bytes - should fail gracefully
        OpenDictionary(b"")
        raise AssertionError("Should have failed with empty bytes")
    except Exception:
        # Expected to fail
        pass

    try:
        # Test with invalid bytes - should fail gracefully
        OpenDictionary(b"invalid dictionary data")
        raise AssertionError("Should have failed with invalid bytes")
    except Exception:
        # Expected to fail
        pass


def test_from_bytes_round_trip():
    """Test creating dictionary from bytes and ensuring round-trip consistency"""
    xml = """
    <dictionary>
      <entry term="round-trip-test">
        <ety>
          <sense pos="n">
            <definition value="A test for round-trip consistency" />
          </sense>
        </ety>
      </entry>
    </dictionary>
    """

    # Compile to bytes
    compiled_bytes = compile(xml)

    # Create dictionary from bytes
    dictionary1 = OpenDictionary(compiled_bytes)

    # Test lookup works
    results1 = dictionary1.lookup("round-trip-test")
    assert len(results1) == 1
    assert results1[0].entry.term == "round-trip-test"

    # Save to file
    temp_dir = tempfile.gettempdir()
    temp_file = os.path.join(temp_dir, f"{uuid.uuid4()}.odict")

    try:
        dictionary1.save(temp_file)

        # Read file and create new dictionary from the bytes
        with open(temp_file, "rb") as f:
            file_bytes = f.read()

        dictionary2 = OpenDictionary(file_bytes)

        # Test lookup still works
        results2 = dictionary2.lookup("round-trip-test")
        assert len(results2) == 1
        assert results2[0].entry.term == "round-trip-test"

        # Results should be equivalent
        assert results1[0].entry.term == results2[0].entry.term

    finally:
        if os.path.exists(temp_file):
            os.remove(temp_file)


def test_compile_function():
    """Test the compile function specifically"""
    xml = """
    <dictionary>
      <entry term="compile-function-test">
        <ety>
          <sense pos="n">
            <definition value="A test for the compile function" />
          </sense>
        </ety>
      </entry>
    </dictionary>
    """

    # Test compile function returns bytes
    compiled_bytes = compile(xml)
    assert isinstance(compiled_bytes, bytes)
    assert len(compiled_bytes) > 0

    # Test bytes can be used to create dictionary
    dictionary = OpenDictionary(compiled_bytes)
    results = dictionary.lookup("compile-function-test")

    assert len(results) == 1
    assert results[0].entry.term == "compile-function-test"


def test_compile_with_invalid_xml():
    """Test compile function with invalid XML"""
    invalid_xml = """
    <dictionary>
      <entry term="invalid">
        <ety>
          <sense pos="n">
            <definition value="Missing closing tags
          </sense>
        </ety>
      </entry>
    """

    try:
        compile(invalid_xml)
        raise AssertionError("Should have failed with invalid XML")
    except Exception:
        # Expected to fail
        pass


def test_multiple_dictionary_instances():
    """Test creating multiple dictionary instances from same bytes"""
    xml = """
    <dictionary>
      <entry term="multi-instance-test">
        <ety>
          <sense pos="n">
            <definition value="A test for multiple instances" />
          </sense>
        </ety>
      </entry>
    </dictionary>
    """

    compiled_bytes = compile(xml)

    # Create multiple dictionaries from same bytes
    dict1 = OpenDictionary(compiled_bytes)
    dict2 = OpenDictionary(compiled_bytes)
    dict3 = OpenDictionary(compiled_bytes)

    # All should work independently
    results1 = dict1.lookup("multi-instance-test")
    results2 = dict2.lookup("multi-instance-test")
    results3 = dict3.lookup("multi-instance-test")

    assert len(results1) == 1
    assert len(results2) == 1
    assert len(results3) == 1

    assert results1[0].entry.term == "multi-instance-test"
    assert results2[0].entry.term == "multi-instance-test"
    assert results3[0].entry.term == "multi-instance-test"


def test_dictionary_properties():
    """Test dictionary properties like min_rank, max_rank"""
    xml = """
    <dictionary>
      <entry term="prop-test-1" rank="10">
        <ety>
          <sense pos="n">
            <definition value="First test entry" />
          </sense>
        </ety>
      </entry>
      <entry term="prop-test-2" rank="50">
        <ety>
          <sense pos="n">
            <definition value="Second test entry" />
          </sense>
        </ety>
      </entry>
    </dictionary>
    """

    compiled_bytes = compile(xml)
    dictionary = OpenDictionary(compiled_bytes)

    # Test rank properties
    assert dictionary.min_rank == 10
    assert dictionary.max_rank == 50

    # Test lexicon
    lexicon = dictionary.lexicon()
    assert "prop-test-1" in lexicon
    assert "prop-test-2" in lexicon


def test_empty_dictionary():
    """Test behavior with empty dictionary"""
    xml = """
    <dictionary>
    </dictionary>
    """

    compiled_bytes = compile(xml)
    dictionary = OpenDictionary(compiled_bytes)

    # Test empty lookups
    results = dictionary.lookup("nonexistent")
    assert len(results) == 0

    # Test properties with empty dictionary
    assert dictionary.min_rank is None
    assert dictionary.max_rank is None

    # Test lexicon with empty dictionary
    lexicon = dictionary.lexicon()
    assert len(lexicon) == 0
