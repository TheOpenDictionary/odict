import pytest
import os
import tempfile
import odict_uniffi

@pytest.fixture(scope="session", autouse=True)
def init_odict():
    with tempfile.TemporaryDirectory() as tmp:
        odict_uniffi.init(tmp)
        yield tmp

@pytest.fixture
def storage_path(tmp_path):
    path = str(tmp_path / "storage")
    os.makedirs(path, exist_ok=True)
    return path
