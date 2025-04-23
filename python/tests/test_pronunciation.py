import os
import sys
import tempfile
import uuid

# Add parent directory to path for imports
sys.path.insert(
     0,
     os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
)

from theopendictionary import (  # noqa: E402
    Dictionary,
    PronunciationKind,
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
        # Write XML content to an ODICT file
        Dictionary.write(xml, temp_file)

        # Create dictionary from the temporary ODICT file
        dictionary = Dictionary(temp_file)
        results = dictionary.lookup("你好")

        assert len(results) == 1
        entry = results[0].entry
        assert len(entry.etymologies) == 1
        assert len(entry.etymologies[0].pronunciations) == 1
        assert entry.etymologies[0].pronunciations[0].value == "ni hao"
        assert isinstance(
            entry.etymologies[0].pronunciations[0].kind, PronunciationKind
        )
        assert str(entry.etymologies[0].pronunciations[0].kind) == "pinyin"
        assert len(entry.etymologies[0].pronunciations[0].urls) == 1
        assert (
            entry.etymologies[0].pronunciations[0].urls[0].src
            == "./audio.mp3"
        )

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
        # Write XML content to an ODICT file
        Dictionary.write(xml, temp_file)

        # Create dictionary from the temporary ODICT file
        dictionary = Dictionary(temp_file)
        results = dictionary.lookup("example")

        assert len(results) == 1
        entry = results[0].entry

        # Access the example through the definition
        definition = next(
            iter(entry.etymologies[0].senses.values())
        ).definitions[0]
        example = definition.examples[0]

        assert len(example.pronunciations) == 1
        assert example.pronunciations[0].value == "ɪɡˈzæmpl ˈsɛntəns"
        assert str(example.pronunciations[0].kind) == "ipa"
        assert len(example.pronunciations[0].urls) == 1
        assert example.pronunciations[0].urls[0].src == "./example.mp3"
        assert example.pronunciations[0].urls[0].mime_type == "audio/mpeg"

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
        # Write XML content to an ODICT file
        Dictionary.write(xml, temp_file)

        # Create dictionary from the temporary ODICT file
        dictionary = Dictionary(temp_file)
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
