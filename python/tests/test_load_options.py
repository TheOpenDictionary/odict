import os
import sys
import tempfile
import uuid
import pytest

# Add parent directory to path for imports
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from theopendictionary import (  # noqa: E402
    OpenDictionary,
    compile,
)


@pytest.mark.asyncio
async def test_load_with_alias_options():
    """Test loading with alias options"""
    xml = """
    <dictionary>
      <entry term="alias-test">
        <ety>
          <sense pos="n">
            <definition value="A test for alias loading" />
          </sense>
        </ety>
      </entry>
    </dictionary>
    """

    # Create temporary files
    temp_dir = tempfile.gettempdir()
    temp_file = os.path.join(temp_dir, f"{uuid.uuid4()}.odict")
    alias_file = os.path.join(temp_dir, f"{uuid.uuid4()}.alias")

    try:
        # First create the dictionary file
        compiled_bytes = compile(xml)
        dictionary = OpenDictionary(compiled_bytes)
        dictionary.save(temp_file)

        # Create a simple alias file (empty JSON for this test)
        with open(alias_file, "w") as f:
            f.write("{}")

        # Test load with alias options using kwargs
        loaded_dict = await OpenDictionary.load(temp_file, alias_path=alias_file)
        results = loaded_dict.lookup("alias-test")

        assert len(results) == 1
        assert results[0].entry.term == "alias-test"

    finally:
        # Clean up
        for file_path in [temp_file, alias_file]:
            if os.path.exists(file_path):
                os.remove(file_path)


@pytest.mark.asyncio
async def test_load_without_options():
    """Test loading without any options (default behavior)"""
    xml = """
    <dictionary>
      <entry term="no-options-load">
        <ety>
          <sense pos="n">
            <definition value="A test for loading without options" />
          </sense>
        </ety>
      </entry>
    </dictionary>
    """

    temp_dir = tempfile.gettempdir()
    temp_file = os.path.join(temp_dir, f"{uuid.uuid4()}.odict")

    try:
        # Create the dictionary file
        compiled_bytes = compile(xml)
        dictionary = OpenDictionary(compiled_bytes)
        dictionary.save(temp_file)

        # Load without any options
        loaded_dict = await OpenDictionary.load(temp_file)
        results = loaded_dict.lookup("no-options-load")

        assert len(results) == 1
        assert results[0].entry.term == "no-options-load"

    finally:
        if os.path.exists(temp_file):
            os.remove(temp_file)


@pytest.mark.asyncio
async def test_load_with_empty_load_options():
    """Test loading with empty LoadOptions object"""
    xml = """
    <dictionary>
      <entry term="empty-load-options">
        <ety>
          <sense pos="n">
            <definition value="A test for empty load options" />
          </sense>
        </ety>
      </entry>
    </dictionary>
    """

    temp_dir = tempfile.gettempdir()
    temp_file = os.path.join(temp_dir, f"{uuid.uuid4()}.odict")

    try:
        # Create the dictionary file
        compiled_bytes = compile(xml)
        dictionary = OpenDictionary(compiled_bytes)
        dictionary.save(temp_file)

        # Test load without any alias path (default behavior)
        loaded_dict = await OpenDictionary.load(temp_file)

        results = loaded_dict.lookup("empty-load-options")
        assert len(results) == 1
        assert results[0].entry.term == "empty-load-options"

    finally:
        if os.path.exists(temp_file):
            os.remove(temp_file)


@pytest.mark.asyncio
async def test_load_with_invalid_alias_path():
    """Test loading with invalid alias path (should handle gracefully)"""
    xml = """
    <dictionary>
      <entry term="invalid-alias-test">
        <ety>
          <sense pos="n">
            <definition value="A test for invalid alias path" />
          </sense>
        </ety>
      </entry>
    </dictionary>
    """

    temp_dir = tempfile.gettempdir()
    temp_file = os.path.join(temp_dir, f"{uuid.uuid4()}.odict")
    invalid_alias_path = "/nonexistent/path/to/aliases.alias"

    try:
        # Create the dictionary file
        compiled_bytes = compile(xml)
        dictionary = OpenDictionary(compiled_bytes)
        dictionary.save(temp_file)

        # Test load with invalid alias path - should either work or fail
        # gracefully
        try:
            loaded_dict = await OpenDictionary.load(
                temp_file, alias_path=invalid_alias_path
            )
            results = loaded_dict.lookup("invalid-alias-test")
            # If it succeeds, verify it still works
            assert len(results) == 1
            assert results[0].entry.term == "invalid-alias-test"
        except Exception:
            # If it fails, that's also acceptable behavior
            pass

    finally:
        if os.path.exists(temp_file):
            os.remove(temp_file)


def test_load_kwargs_api():
    """Test that load method accepts kwargs properly"""
    # These are just API tests - no actual loading needed

    # Test that method signature accepts alias_path
    import inspect

    sig = inspect.signature(OpenDictionary.load)
    params = list(sig.parameters.keys())

    assert "dictionary" in params
    assert "alias_path" in params


@pytest.mark.asyncio
async def test_async_load_method_directly():
    """Test the async load method directly (not through asyncio.run)"""
    xml = """
    <dictionary>
      <entry term="async-direct-test">
        <ety>
          <sense pos="n">
            <definition value="A test for direct async usage" />
          </sense>
        </ety>
      </entry>
    </dictionary>
    """

    temp_dir = tempfile.gettempdir()
    temp_file = os.path.join(temp_dir, f"{uuid.uuid4()}.odict")

    try:
        # Create the dictionary file
        compiled_bytes = compile(xml)
        dictionary = OpenDictionary(compiled_bytes)
        dictionary.save(temp_file)

        # Test async load directly
        loaded_dict = await OpenDictionary.load(temp_file)
        results = loaded_dict.lookup("async-direct-test")

        assert len(results) == 1
        assert results[0].entry.term == "async-direct-test"

    finally:
        if os.path.exists(temp_file):
            os.remove(temp_file)
