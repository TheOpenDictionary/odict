import pytest
from pathlib import Path
from theopendictionary import Dictionary


@pytest.fixture(scope="module")
def dict3_path():
    current_file = Path(__file__).resolve()
    return str(current_file.parent.parent.parent / "examples" / "example3.xml")


@pytest.fixture(scope="module")
def dict3(dict3_path):
    return Dictionary.compile(dict3_path)


@pytest.fixture(scope="module")
def dict1_path():
    current_file = Path(__file__).resolve()
    return str(current_file.parent.parent.parent / "examples" / "example1.xml")


@pytest.fixture(scope="module")
def dict1(dict1_path):
    return Dictionary.compile(dict1_path)


def test_tokenize(dict3, snapshot):
    # Test tokenization similar to the Node.js test
    tokens = dict3.tokenize("你好！你是谁？")

    # Verify we got some tokens
    assert len(tokens) > 0
    print(tokens[0])
    # Check specific token values
    assert tokens[0].lemma == "你好"
    assert tokens[0].entries[0].entry.term == "你"
    assert tokens[0].entries[1].entry.term == "好"

    # Use snapshot testing for the full result
    assert tokens == snapshot


def test_tokenize_case_sensitive(dict1):
    # By default tokenize should be case-sensitive
    tokens = dict1.tokenize("DOG cat")

    assert len(tokens) == 2
    assert tokens[0].lemma == "DOG"
    assert len(tokens[0].entries) == 0  # "DOG" shouldn't match "dog"
    assert tokens[1].lemma == "cat"
    assert len(tokens[1].entries) == 1
    assert tokens[1].entries[0].entry.term == "cat"


def test_tokenize_case_insensitive(dict1):
    # Test case-insensitive tokenization
    tokens = dict1.tokenize("DOG cat", insensitive=True)

    assert len(tokens) == 2
    assert tokens[0].lemma == "DOG"
    assert (
        len(tokens[0].entries) == 1
    )  # Now "DOG" should match "dog" with insensitivity
    assert tokens[0].entries[0].entry.term == "dog"
    assert tokens[1].lemma == "cat"
    assert tokens[1].entries[0].entry.term == "cat"


def test_tokenize_case_insensitive_mixed_case(dict1):
    # Test with mixed case text
    tokens = dict1.tokenize("DoG CaT", insensitive=True)

    assert len(tokens) == 2
    assert tokens[0].lemma == "DoG"
    assert len(tokens[0].entries) == 1
    assert tokens[0].entries[0].entry.term == "dog"
    assert tokens[1].lemma == "CaT"
    assert len(tokens[1].entries) == 1
    assert tokens[1].entries[0].entry.term == "cat"


def test_tokenize_case_insensitive_with_follow():
    # Create a dictionary with aliases for this test
    xml_content = """
    <dictionary>
        <entry term="run">
            <ety>
                <sense pos="v">
                    <definition value="To move quickly" />
                </sense>
            </ety>
        </entry>
        <entry term="runs" see="run" />
    </dictionary>
    """

    # Create a temporary dictionary
    import tempfile
    import os

    with tempfile.NamedTemporaryFile(suffix=".odict", delete=False) as temp_file:
        temp_path = temp_file.name

    Dictionary.write(xml_content, temp_path)

    try:
        dict_instance = Dictionary(temp_path)

        # Test case insensitivity with follow option
        tokens = dict_instance.tokenize("RUNS", follow=True, insensitive=True)

        assert len(tokens) == 1
        assert tokens[0].lemma == "RUNS"
        assert len(tokens[0].entries) == 1
        assert tokens[0].entries[0].entry.term == "run"
        assert tokens[0].entries[0].directed_from.term == "runs"

    finally:
        # Clean up the temporary file
        os.unlink(temp_path)
