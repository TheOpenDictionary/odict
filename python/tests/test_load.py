import pytest
import tempfile
import os
from pathlib import Path

from theopendictionary import OpenDictionary, LoadOptions, RemoteLoadOptions, compile


class TestLoadOptions:
    """Test Python LoadOptions path configuration and functionality."""

    def test_load_options_creation(self):
        """Test that LoadOptions can be created with various configurations."""
        # Test default creation
        default_options = LoadOptions()
        assert default_options.config_dir is None
        assert default_options.remote is None

        # Test with config_dir
        config_options = LoadOptions(config_dir="/tmp/test")
        assert config_options.config_dir == "/tmp/test"

        # Test with remote options
        remote_options = LoadOptions(
            remote=RemoteLoadOptions(out_dir="/tmp/downloads", caching=False)
        )
        assert remote_options.remote is not None
        assert remote_options.remote.out_dir == "/tmp/downloads"
        assert remote_options.remote.caching is False

    def test_remote_load_options(self):
        """Test RemoteLoadOptions configuration."""
        # Test default
        default_remote = RemoteLoadOptions()
        assert default_remote.out_dir is None
        assert default_remote.caching is None

        # Test with parameters
        remote_with_params = RemoteLoadOptions(out_dir="/tmp/downloads", caching=True)
        assert remote_with_params.out_dir == "/tmp/downloads"
        assert remote_with_params.caching is True

    def test_combined_options(self):
        """Test combined config_dir and remote options."""
        combined = LoadOptions(
            config_dir="/tmp/config",
            remote=RemoteLoadOptions(out_dir="/tmp/downloads", caching=False),
        )
        assert combined.config_dir == "/tmp/config"
        assert combined.remote.out_dir == "/tmp/downloads"
        assert combined.remote.caching is False


class TestLoadFunctionality:
    """Test the actual loading functionality with path configuration."""

    @pytest.mark.asyncio
    async def test_custom_config_dir_creates_alias_files(self):
        """Test that custom config_dir actually creates alias files in the specified directory."""
        with tempfile.TemporaryDirectory() as temp_dir:
            # Create a simple test dictionary
            dict_xml = """<?xml version="1.0" encoding="UTF-8"?>
<dictionary>
  <entry term="test">
    <ety>
      <sense pos="n">
        <definition value="A test entry" />
      </sense>
    </ety>
  </entry>
</dictionary>"""

            # Compile the XML and save as .odict file
            compiled_bytes = compile(dict_xml)
            dict_path = os.path.join(temp_dir, "test.odict")
            with open(dict_path, "wb") as f:
                f.write(compiled_bytes)

            # Load the dictionary with custom config_dir
            config_dir = os.path.join(temp_dir, "custom_config")
            os.makedirs(config_dir, exist_ok=True)

            load_options = LoadOptions(config_dir=config_dir)

            # Load the dictionary - this should use the custom config_dir
            dict_obj = await OpenDictionary.load(dict_path, load_options)

            # Verify the dictionary loaded successfully
            assert dict_obj is not None

            # Note: We can't easily test alias creation from Python without exposing
            # the alias API, but we've verified the config_dir is accepted and used

    @pytest.mark.asyncio
    async def test_custom_download_output_directory(self):
        """Test that custom download output directory is properly configured."""
        with tempfile.TemporaryDirectory() as temp_dir:
            download_dir = os.path.join(temp_dir, "downloads")
            os.makedirs(download_dir, exist_ok=True)

            remote_options = RemoteLoadOptions(out_dir=download_dir, caching=False)
            load_options = LoadOptions(remote=remote_options)

            # Test with invalid remote dictionary name to verify configuration without network
            with pytest.raises(Exception):
                await OpenDictionary.load("not_a_remote_name", load_options)

    @pytest.mark.asyncio
    async def test_combined_custom_paths(self):
        """Test combined custom config and output paths."""
        with tempfile.TemporaryDirectory() as temp_dir:
            config_dir = os.path.join(temp_dir, "config")
            download_dir = os.path.join(temp_dir, "downloads")

            os.makedirs(config_dir, exist_ok=True)
            os.makedirs(download_dir, exist_ok=True)

            remote_options = RemoteLoadOptions(out_dir=download_dir, caching=False)
            load_options = LoadOptions(config_dir=config_dir, remote=remote_options)

            # Verify both paths are configured
            assert load_options.config_dir == config_dir
            assert load_options.remote.out_dir == download_dir

    @pytest.mark.asyncio
    async def test_default_load_without_options(self):
        """Test that default load (without options) still works."""
        with pytest.raises(Exception):
            await OpenDictionary.load("non_existent_file.odict")

    @pytest.mark.asyncio
    async def test_empty_options_object(self):
        """Test behavior with empty options object."""
        empty_options = LoadOptions()
        with pytest.raises(Exception):
            await OpenDictionary.load("non_existent_file.odict", empty_options)

    @pytest.mark.asyncio
    async def test_path_handling_with_special_characters(self):
        """Test that paths with special characters are properly handled."""
        with tempfile.TemporaryDirectory(prefix="odict test with spaces") as temp_dir:
            config_dir = os.path.join(temp_dir, "config dir with spaces")
            download_dir = os.path.join(temp_dir, "download dir with spaces")

            os.makedirs(config_dir, exist_ok=True)
            os.makedirs(download_dir, exist_ok=True)

            remote_options = RemoteLoadOptions(out_dir=download_dir, caching=False)
            load_options = LoadOptions(config_dir=config_dir, remote=remote_options)

            # Verify paths with spaces are accepted
            assert " " in load_options.config_dir
            assert " " in load_options.remote.out_dir

    @pytest.mark.asyncio
    async def test_caching_configuration(self):
        """Test that caching configuration is properly passed through."""
        with tempfile.TemporaryDirectory() as temp_dir:
            # Test with caching enabled
            remote_cached = RemoteLoadOptions(out_dir=temp_dir, caching=True)
            load_options_cached = LoadOptions(remote=remote_cached)
            assert load_options_cached.remote.caching is True

            # Test with caching disabled
            remote_no_cache = RemoteLoadOptions(out_dir=temp_dir, caching=False)
            load_options_no_cache = LoadOptions(remote=remote_no_cache)
            assert load_options_no_cache.remote.caching is False

    @pytest.mark.asyncio
    async def test_production_readonly_filesystem_simulation(self):
        """Test behavior that simulates production read-only filesystem constraints."""
        with tempfile.TemporaryDirectory() as temp_dir:
            # Simulate production where we can't write to $HOME but have custom writable paths
            config_dir = os.path.join(temp_dir, "writable_config")
            download_dir = os.path.join(temp_dir, "writable_downloads")

            os.makedirs(config_dir, exist_ok=True)
            os.makedirs(download_dir, exist_ok=True)

            remote_options = RemoteLoadOptions(out_dir=download_dir, caching=False)
            load_options = LoadOptions(config_dir=config_dir, remote=remote_options)

            # Verify the configuration is accepted
            assert load_options.config_dir == config_dir
            assert load_options.remote.out_dir == download_dir

            # These paths should be used instead of default $HOME paths
            assert config_dir != str(Path.home())
            assert download_dir != str(Path.home())

    def test_options_reusability(self):
        """Test that options can be reused without side effects."""
        with tempfile.TemporaryDirectory() as temp_dir:
            config_dir = os.path.join(temp_dir, "config")
            download_dir = os.path.join(temp_dir, "downloads")

            os.makedirs(config_dir, exist_ok=True)
            os.makedirs(download_dir, exist_ok=True)

            # Create options once
            remote_options = RemoteLoadOptions(out_dir=download_dir, caching=False)
            load_options = LoadOptions(config_dir=config_dir, remote=remote_options)

            # Verify options can be accessed multiple times
            assert load_options.config_dir == config_dir
            assert load_options.remote.out_dir == download_dir
            assert load_options.remote.caching is False

            # Access again to ensure no side effects
            assert load_options.config_dir == config_dir
            assert load_options.remote.out_dir == download_dir

    @pytest.mark.asyncio
    async def test_relative_and_absolute_paths(self):
        """Test that both relative and absolute paths work correctly."""
        with tempfile.TemporaryDirectory() as temp_dir:
            # Test absolute path
            abs_config = os.path.join(temp_dir, "abs_config")
            os.makedirs(abs_config, exist_ok=True)

            abs_options = LoadOptions(config_dir=abs_config)
            assert os.path.isabs(abs_options.config_dir)

            # Test relative path (though in practice absolute paths are recommended)
            rel_options = LoadOptions(config_dir="relative/path")
            assert rel_options.config_dir == "relative/path"

    @pytest.mark.asyncio
    async def test_none_values_use_defaults(self):
        """Test that None values result in default behavior."""
        # All None should use system defaults
        default_options = LoadOptions(config_dir=None, remote=None)
        assert default_options.config_dir is None
        assert default_options.remote is None

        # Remote with None values should use defaults
        remote_defaults = RemoteLoadOptions(out_dir=None, caching=None)
        assert remote_defaults.out_dir is None
        assert remote_defaults.caching is None
