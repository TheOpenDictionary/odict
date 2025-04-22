import os
import sys
import pytest

sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from theopendictionary import Dictionary, PronunciationKind, MediaURL, Pronunciation


def test_entry_with_pronunciation():
    xml = """
    <dictionary>
      <entry term="你好">
        <pronunciation kind="pinyin" value="ni hao">
          <url src="./audio.mp3" />
        </pronunciation>
      </entry>
    </dictionary>
    """

    dictionary = Dictionary.from_xml(xml)
    results = dictionary.lookup("你好")

    assert len(results) == 1
    entry = results[0].entry
    assert len(entry.pronunciations) == 1
    assert entry.pronunciations[0].value == "ni hao"
    assert isinstance(entry.pronunciations[0].kind, PronunciationKind)
    assert str(entry.pronunciations[0].kind) == "pinyin"
    assert len(entry.pronunciations[0].urls) == 1
    assert entry.pronunciations[0].urls[0].src == "./audio.mp3"


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

    dictionary = Dictionary.from_xml(xml)
    results = dictionary.lookup("example")

    assert len(results) == 1
    entry = results[0].entry

    # Access the example through the definition
    definition = next(iter(entry.etymologies[0].senses.values())).definitions[0]
    example = definition.examples[0]

    assert len(example.pronunciations) == 1
    assert example.pronunciations[0].value == "ɪɡˈzæmpl ˈsɛntəns"
    assert str(example.pronunciations[0].kind) == "ipa"
    assert len(example.pronunciations[0].urls) == 1
    assert example.pronunciations[0].urls[0].src == "./example.mp3"
    assert example.pronunciations[0].urls[0].mime_type == "audio/mpeg"


def test_multiple_pronunciations():
    xml = """
    <dictionary>
      <entry term="hello">
        <pronunciation kind="ipa" value="həˈləʊ">
          <url src="./hello-british.mp3" />
        </pronunciation>
        <pronunciation kind="ipa" value="hɛˈloʊ">
          <url src="./hello-american.mp3" />
        </pronunciation>
        <ety>
          <sense pos="int">
            <definition value="A greeting" />
          </sense>
        </ety>
      </entry>
    </dictionary>
    """

    dictionary = Dictionary.from_xml(xml)
    results = dictionary.lookup("hello")

    assert len(results) == 1
    entry = results[0].entry
    assert len(entry.pronunciations) == 2
    assert entry.pronunciations[0].value == "həˈləʊ"
    assert entry.pronunciations[1].value == "hɛˈloʊ"


def test_custom_pronunciation_kind():
    xml = """
    <dictionary>
      <entry term="北京">
        <pronunciation kind="wadegiles" value="pei-ching">
          <url src="./beijing-wade.mp3" />
        </pronunciation>
      </entry>
    </dictionary>
    """

    dictionary = Dictionary.from_xml(xml)
    results = dictionary.lookup("北京")

    assert len(results) == 1
    entry = results[0].entry
    assert len(entry.pronunciations) == 1

    # Should be converted to an Other variant with the custom value
    assert isinstance(entry.pronunciations[0].kind, PronunciationKind)
    assert str(entry.pronunciations[0].kind) == "wadegiles"
    assert entry.pronunciations[0].value == "pei-ching"


def test_create_pronunciation_objects():
    """Test creating pronunciation objects programmatically"""

    # Create a pronunciation with a built-in kind
    ipa_pron = Pronunciation("ipa", "ˈtɛstɪŋ")
    assert str(ipa_pron.kind) == "ipa"
    assert ipa_pron.value == "ˈtɛstɪŋ"
    assert len(ipa_pron.urls) == 0

    # Add a URL
    url = MediaURL("./test.mp3", "audio/mpeg", "Test audio")
    ipa_pron.add_url(url)
    assert len(ipa_pron.urls) == 1
    assert ipa_pron.urls[0].src == "./test.mp3"
    assert ipa_pron.urls[0].mime_type == "audio/mpeg"
    assert ipa_pron.urls[0].description == "Test audio"

    # Create a pronunciation with a custom kind
    custom_pron = Pronunciation("customSystem", "test pronunciation")
    assert str(custom_pron.kind) == "customSystem"
