import os
import sys
import tempfile
import uuid

# Add parent directory to path for imports
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from theopendictionary import (  # noqa: E402
    OpenDictionary,
    compile,
)


def test_save_with_compression_options():
    """Test saving with compression options"""
    xml = """
    <dictionary>
      <entry term="compression-test">
        <ety>
          <sense pos="n">
            <definition value="A test for compression options" />
          </sense>
        </ety>
      </entry>
    </dictionary>
    """

    # Create a temporary file
    temp_dir = tempfile.gettempdir()
    temp_file = os.path.join(temp_dir, f"{uuid.uuid4()}.odict")

    try:
        # Compile and create dictionary
        compiled_bytes = compile(xml)
        dictionary = OpenDictionary(compiled_bytes)

        # Test save with compression options using kwargs
        dictionary.save(temp_file, quality=5, window_size=15)

        # Verify file was created and can be loaded
        assert os.path.exists(temp_file)

        # Load the saved dictionary using async method
        async def load_and_test():
            loaded_dict = await OpenDictionary.load(temp_file)
            results = loaded_dict.lookup("compression-test")

            assert len(results) == 1
            assert results[0].entry.term == "compression-test"

        import asyncio

        asyncio.run(load_and_test())

    finally:
        # Clean up
        if os.path.exists(temp_file):
            os.remove(temp_file)


def test_save_with_high_compression():
    """Test saving with high compression settings"""
    xml = """
    <dictionary>
      <entry term="high-compression">
        <ety>
          <sense pos="n">
            <definition value="A test for high compression" />
          </sense>
        </ety>
      </entry>
    </dictionary>
    """

    temp_dir = tempfile.gettempdir()
    temp_file = os.path.join(temp_dir, f"{uuid.uuid4()}.odict")

    try:
        compiled_bytes = compile(xml)
        dictionary = OpenDictionary(compiled_bytes)

        # Test with maximum compression using kwargs
        dictionary.save(temp_file, quality=9, window_size=32)

        assert os.path.exists(temp_file)

        async def load_and_test():
            loaded_dict = await OpenDictionary.load(temp_file)
            results = loaded_dict.lookup("high-compression")

            assert len(results) == 1
            assert results[0].entry.term == "high-compression"

        import asyncio

        asyncio.run(load_and_test())

    finally:
        if os.path.exists(temp_file):
            os.remove(temp_file)


def test_save_with_low_compression():
    """Test saving with low compression settings"""
    xml = """
    <dictionary>
      <entry term="low-compression">
        <ety>
          <sense pos="n">
            <definition value="A test for low compression" />
          </sense>
        </ety>
      </entry>
    </dictionary>
    """

    temp_dir = tempfile.gettempdir()
    temp_file = os.path.join(temp_dir, f"{uuid.uuid4()}.odict")

    try:
        compiled_bytes = compile(xml)
        dictionary = OpenDictionary(compiled_bytes)

        # Test with minimal compression using kwargs
        dictionary.save(temp_file, quality=1, window_size=8)

        assert os.path.exists(temp_file)

        async def load_and_test():
            loaded_dict = await OpenDictionary.load(temp_file)
            results = loaded_dict.lookup("low-compression")

            assert len(results) == 1
            assert results[0].entry.term == "low-compression"

        import asyncio

        asyncio.run(load_and_test())

    finally:
        if os.path.exists(temp_file):
            os.remove(temp_file)


def test_multiple_save_formats():
    """Test saving the same dictionary multiple times with different options"""
    xml = """
    <dictionary>
      <entry term="multi-save-test">
        <ety>
          <sense pos="n">
            <definition value="A test for multiple save operations" />
          </sense>
        </ety>
      </entry>
    </dictionary>
    """

    # Create temporary files
    temp_dir = tempfile.gettempdir()
    temp_file1 = os.path.join(temp_dir, f"{uuid.uuid4()}-1.odict")
    temp_file2 = os.path.join(temp_dir, f"{uuid.uuid4()}-2.odict")

    try:
        # Compile and create dictionary
        compiled_bytes = compile(xml)
        dictionary = OpenDictionary(compiled_bytes)

        # Save without compression
        dictionary.save(temp_file1)

        # Save with compression using kwargs
        dictionary.save(temp_file2, quality=1)

        # Verify both files were created
        assert os.path.exists(temp_file1)
        assert os.path.exists(temp_file2)

        # Both should be loadable and functional
        async def load_and_test():
            dict1 = await OpenDictionary.load(temp_file1)
            dict2 = await OpenDictionary.load(temp_file2)

            results1 = dict1.lookup("multi-save-test")
            results2 = dict2.lookup("multi-save-test")

            assert len(results1) == 1
            assert len(results2) == 1
            assert results1[0].entry.term == results2[0].entry.term

        import asyncio

        asyncio.run(load_and_test())

    finally:
        # Clean up
        for file_path in [temp_file1, temp_file2]:
            if os.path.exists(file_path):
                os.remove(file_path)


def test_save_without_options():
    """Test saving without any options (default behavior)"""
    xml = """
    <dictionary>
      <entry term="no-options-save">
        <ety>
          <sense pos="n">
            <definition value="A test for saving without options" />
          </sense>
        </ety>
      </entry>
    </dictionary>
    """

    temp_dir = tempfile.gettempdir()
    temp_file = os.path.join(temp_dir, f"{uuid.uuid4()}.odict")

    try:
        compiled_bytes = compile(xml)
        dictionary = OpenDictionary(compiled_bytes)

        # Save without any options
        dictionary.save(temp_file)

        assert os.path.exists(temp_file)

        async def load_and_test():
            loaded_dict = await OpenDictionary.load(temp_file)
            results = loaded_dict.lookup("no-options-save")

            assert len(results) == 1
            assert results[0].entry.term == "no-options-save"

        import asyncio

        asyncio.run(load_and_test())

    finally:
        if os.path.exists(temp_file):
            os.remove(temp_file)


def test_save_with_empty_save_options():
    """Test saving with empty SaveOptions object"""
    xml = """
    <dictionary>
      <entry term="empty-save-options">
        <ety>
          <sense pos="n">
            <definition value="A test for empty save options" />
          </sense>
        </ety>
      </entry>
    </dictionary>
    """

    temp_dir = tempfile.gettempdir()
    temp_file = os.path.join(temp_dir, f"{uuid.uuid4()}.odict")

    try:
        compiled_bytes = compile(xml)
        dictionary = OpenDictionary(compiled_bytes)

        # Save without any compression options (default behavior)
        dictionary.save(temp_file)

        assert os.path.exists(temp_file)

        async def load_and_test():
            loaded_dict = await OpenDictionary.load(temp_file)
            results = loaded_dict.lookup("empty-save-options")

            assert len(results) == 1
            assert results[0].entry.term == "empty-save-options"

        import asyncio

        asyncio.run(load_and_test())

    finally:
        if os.path.exists(temp_file):
            os.remove(temp_file)
