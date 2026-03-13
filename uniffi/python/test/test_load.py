import pytest
import os
from odict_uniffi import load_dictionary, dictionary_from_bytes

@pytest.mark.asyncio
async def test_load_dictionary(dict1, tmp_path):
    odict_path = str(tmp_path / "test.odict")
    dict1.save(odict_path, None, None)
    
    loaded_dict = await load_dictionary(odict_path, None)
    assert loaded_dict is not None
    assert loaded_dict.lexicon() == dict1.lexicon()

def test_to_bytes(dict1):
    bytes_data = dict1.to_bytes()
    assert len(bytes_data) > 0
    
    loaded_dict = dictionary_from_bytes(bytes_data)
    assert loaded_dict.lexicon() == dict1.lexicon()

def test_to_bytes_with_options(dict1):
    bytes_data = dict1.to_bytes_with_options(9, 32)
    assert len(bytes_data) > 0
    
    loaded_dict = dictionary_from_bytes(bytes_data)
    assert loaded_dict.lexicon() == dict1.lexicon()

def test_save(dict1, tmp_path):
    odict_path = str(tmp_path / "saved.odict")
    dict1.save(odict_path, 9, 1024)
    assert os.path.exists(odict_path)
    
    with open(odict_path, "rb") as f:
        loaded_dict = dictionary_from_bytes(f.read())
    assert loaded_dict.lexicon() == dict1.lexicon()
