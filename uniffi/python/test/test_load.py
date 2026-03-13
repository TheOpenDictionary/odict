import pytest
import os
from odict_uniffi import load_dictionary, dictionary_from_bytes

@pytest.mark.asyncio
async def test_load_dictionary(dict1, tmp_path):
    storage_path = str(tmp_path / "storage")
    os.makedirs(storage_path, exist_ok=True)
    odict_path = str(tmp_path / "test.odict")
    dict1.save(odict_path, None, None)
    
    loaded_dict = await load_dictionary(storage_path, odict_path, None)
    assert loaded_dict is not None
    assert loaded_dict.lexicon() == dict1.lexicon()

def test_to_bytes(dict1, tmp_path):
    storage_path = str(tmp_path / "storage")
    os.makedirs(storage_path, exist_ok=True)
    bytes_data = dict1.to_bytes()
    assert len(bytes_data) > 0
    
    loaded_dict = dictionary_from_bytes(storage_path, bytes_data)
    assert loaded_dict.lexicon() == dict1.lexicon()

def test_save(dict1, tmp_path):
    storage_path = str(tmp_path / "storage")
    os.makedirs(storage_path, exist_ok=True)
    odict_path = str(tmp_path / "saved.odict")
    dict1.save(odict_path, 9, 1024)
    assert os.path.exists(odict_path)
    
    with open(odict_path, "rb") as f:
        loaded_dict = dictionary_from_bytes(storage_path, f.read())
    assert loaded_dict.lexicon() == dict1.lexicon()
