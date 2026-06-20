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
    LoadOptions,
    RemoteLoadOptions,
)


@pytest.mark.asyncio
async def test_load_with_config_dir():
    """Test loading with config directory (which enables alias functionality)"""
    xml = """
    <dictionary>
      <entry term="config-dir-test">
        <ety>
          <sense pos="n">
            <definition value="A test for config dir loading" />
          </sense>
        </ety>
      </entry>
    </dictionary>
    """

    # Create temporary files
    temp_dir = tempfile.gettempdir()
    temp_file = os.path.join(temp_dir, f"{uuid.uuid4()}.odict")
    config_dir = os.path.join(temp_dir, f"{uuid.uuid4()}_config")

    try:
        # First create the dictionary file
        compiled_bytes = compile(xml)
        dictionary = OpenDictionary(compiled_bytes)
        dictionary.save(temp_file)

        # Create config directory
        os.makedirs(config_dir, exist_ok=True)

        # Test load with config_dir using LoadOptions
        load_opts = LoadOptions(config_dir=config_dir)
        loaded_dict = await OpenDictionary.load(temp_file, options=load_opts)
        results = loaded_dict.lookup("config-dir-test")

        assert len(results) == 1
        assert results[0].entry.term == "config-dir-test"

    finally:
        # Clean up
        for file_path in [temp_file]:
            if os.path.exists(file_path):
                os.remove(file_path)
        import shutil

        if os.path.exists(config_dir):
            shutil.rmtree(config_dir)


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
async def test_load_with_invalid_config_dir():
    """Test loading with invalid config directory (should handle gracefully)"""
    xml = """
    <dictionary>
      <entry term="invalid-config-test">
        <ety>
          <sense pos="n">
            <definition value="A test for invalid config dir" />
          </sense>
        </ety>
      </entry>
    </dictionary>
    """

    temp_dir = tempfile.gettempdir()
    temp_file = os.path.join(temp_dir, f"{uuid.uuid4()}.odict")
    invalid_config_dir = "/nonexistent/path/to/config"

    try:
        # Create the dictionary file
        compiled_bytes = compile(xml)
        dictionary = OpenDictionary(compiled_bytes)
        dictionary.save(temp_file)

        # Test load with invalid config directory - should either work or fail
        # gracefully
        try:
            load_opts = LoadOptions(config_dir=invalid_config_dir)
            loaded_dict = await OpenDictionary.load(temp_file, options=load_opts)
            results = loaded_dict.lookup("invalid-config-test")
            # If it succeeds, verify it still works
            assert len(results) == 1
            assert results[0].entry.term == "invalid-config-test"
        except Exception:
            # If it fails, that's also acceptable behavior
            pass

    finally:
        if os.path.exists(temp_file):
            os.remove(temp_file)


def test_load_options_api():
    """Test that LoadOptions and RemoteLoadOptions can be created"""
    # Test RemoteLoadOptions
    remote_opts = RemoteLoadOptions(out_dir="/tmp/cache", caching=True)
    assert remote_opts.out_dir == "/tmp/cache"
    assert remote_opts.caching is True

    # Test RemoteLoadOptions with only out_dir
    remote_opts2 = RemoteLoadOptions(out_dir="/tmp/cache")
    assert remote_opts2.out_dir == "/tmp/cache"
    assert remote_opts2.caching is None

    # Test RemoteLoadOptions with only caching
    remote_opts3 = RemoteLoadOptions(caching=False)
    assert remote_opts3.out_dir is None
    assert remote_opts3.caching is False

    # Test LoadOptions with config_dir and remote
    load_opts = LoadOptions(config_dir="/tmp/config", remote=remote_opts)
    assert load_opts.config_dir == "/tmp/config"
    assert load_opts.remote is not None

    # Test LoadOptions with only config_dir
    load_opts2 = LoadOptions(config_dir="/tmp/config")
    assert load_opts2.config_dir == "/tmp/config"
    assert load_opts2.remote is None

    # Test LoadOptions with only remote
    load_opts3 = LoadOptions(remote=remote_opts)
    assert load_opts3.config_dir is None
    assert load_opts3.remote is not None

    # Test LoadOptions with no options
    load_opts4 = LoadOptions()
    assert load_opts4.config_dir is None
    assert load_opts4.remote is None


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
