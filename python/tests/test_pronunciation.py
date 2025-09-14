import os
import sys
import tempfile
import uuid
import asyncio

# Add parent directory to path for imports
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from theopendictionary import (  # noqa: E402
    OpenDictionary,
    EnumWrapper,
    compile,
)


def test_entry_with_pronunciation():
    xml = """
    <dictionary>
      <entry term="你好">
        <ety>
          <pronunciation kind="pinyin" value="ni hao">
            <url src="./audio.mp3" />
          </pronunciation>
        </ety>
      </entry>
    </dictionary>
    """

    # Create a temporary file
    temp_dir = tempfile.gettempdir()
    temp_file = os.path.join(temp_dir, f"{uuid.uuid4()}.odict")

    try:
        # Compile XML content to bytes and create dictionary
        compiled_bytes = compile(xml)
        dictionary = OpenDictionary(compiled_bytes)

        # Save to file for testing
        dictionary.save(temp_file)

        results = dictionary.lookup("你好")

        assert len(results) == 1
        entry = results[0].entry
        assert len(entry.etymologies) == 1
        assert len(entry.etymologies[0].pronunciations) == 1

        pronunciation = entry.etymologies[0].pronunciations[0]
        assert pronunciation.value == "ni hao"
        assert isinstance(pronunciation.kind, EnumWrapper)
        assert pronunciation.kind.variant == "pinyin"
        assert pronunciation.kind.value == "pinyin"
        assert len(pronunciation.media) == 1
        assert pronunciation.media[0].src == "./audio.mp3"

    finally:
        # Clean up
        if os.path.exists(temp_file):
            os.remove(temp_file)


def test_example_with_pronunciation():
    xml = """
    <dictionary>
      <entry term="example">
        <ety>
          <sense pos="n">
            <definition value="An example definition">
              <example value="An example sentence">
                <pronunciation kind="ipa" value="ɪɡˈzæmpl ˈsɛntəns">
                  <url src="./example.mp3" type="audio/mpeg" />
                </pronunciation>
              </example>
            </definition>
          </sense>
        </ety>
      </entry>
    </dictionary>
    """

    # Create a temporary file
    temp_dir = tempfile.gettempdir()
    temp_file = os.path.join(temp_dir, f"{uuid.uuid4()}.odict")

    try:
        # Compile XML content to bytes and create dictionary
        compiled_bytes = compile(xml)
        dictionary = OpenDictionary(compiled_bytes)

        # Save to file for testing
        dictionary.save(temp_file)

        results = dictionary.lookup("example")

        assert len(results) == 1
        entry = results[0].entry

        # Access the example through the definition
        first_sense = next(iter(entry.etymologies[0].senses.values()))
        definition = first_sense.definitions[0]
        example = definition.examples[0]

        assert len(example.pronunciations) == 1
        assert example.pronunciations[0].value == "ɪɡˈzæmpl ˈsɛntəns"
        assert example.pronunciations[0].kind.variant == "ipa"
        assert example.pronunciations[0].kind.value == "ipa"
        assert len(example.pronunciations[0].media) == 1
        assert example.pronunciations[0].media[0].src == "./example.mp3"
        assert example.pronunciations[0].media[0].mime_type == "audio/mpeg"

    finally:
        # Clean up
        if os.path.exists(temp_file):
            os.remove(temp_file)


def test_multiple_pronunciations():
    xml = """
    <dictionary>
      <entry term="hello">
        <ety>
          <pronunciation kind="ipa" value="həˈləʊ">
            <url src="./hello-british.mp3" />
          </pronunciation>
          <pronunciation kind="ipa" value="hɛˈloʊ">
            <url src="./hello-american.mp3" />
          </pronunciation>
          <sense pos="adj">
            <definition value="A greeting" />
          </sense>
        </ety>
      </entry>
    </dictionary>
    """

    # Create a temporary file
    temp_dir = tempfile.gettempdir()
    temp_file = os.path.join(temp_dir, f"{uuid.uuid4()}.odict")

    try:
        # Compile XML content to bytes and create dictionary
        compiled_bytes = compile(xml)
        dictionary = OpenDictionary(compiled_bytes)

        # Save to file for testing
        dictionary.save(temp_file)

        results = dictionary.lookup("hello")

        assert len(results) == 1
        entry = results[0].entry
        assert len(entry.etymologies) == 1
        assert len(entry.etymologies[0].pronunciations) == 2
        assert entry.etymologies[0].pronunciations[0].value == "həˈləʊ"
        assert entry.etymologies[0].pronunciations[1].value == "hɛˈloʊ"

    finally:
        # Clean up
        if os.path.exists(temp_file):
            os.remove(temp_file)


def test_load_method_sync_wrapper():
    """Test the load method using asyncio.run for synchronous testing"""

    async def async_test():
        xml = """
        <dictionary>
          <entry term="sync-wrapper">
            <ety>
              <pronunciation kind="pinyin" value="sɪŋk ˈræpər">
                <url src="./sync-wrapper.mp3" />
              </pronunciation>
            </ety>
          </entry>
        </dictionary>
        """

        # Create a temporary file
        temp_dir = tempfile.gettempdir()
        temp_file = os.path.join(temp_dir, f"{uuid.uuid4()}.odict")

        try:
            # First create the dictionary file
            compiled_bytes = compile(xml)
            dictionary = OpenDictionary(compiled_bytes)
            dictionary.save(temp_file)

            # Test the load method
            loaded_dictionary = await OpenDictionary.load(temp_file)
            results = loaded_dictionary.lookup("sync-wrapper")

            assert len(results) == 1
            entry = results[0].entry
            pronunciation = entry.etymologies[0].pronunciations[0]
            assert pronunciation.value == "sɪŋk ˈræpər"
            assert pronunciation.kind.variant == "pinyin"

        finally:
            # Clean up
            if os.path.exists(temp_file):
                os.remove(temp_file)

    # Run the async test
    asyncio.run(async_test())
