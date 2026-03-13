import pytest
import tempfile
import os
from pathlib import Path

from theopendictionary import OpenDictionary, LoadOptions, RemoteLoadOptions, compile, init

@pytest.fixture
def storage_path():
    with tempfile.TemporaryDirectory() as tmp:
        init(tmp)
        yield tmp

class TestLoadFunctionality:
    @pytest.mark.asyncio
    async def test_load_with_storage_path(self, storage_path):
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
        compiled_bytes = compile(storage_path, dict_xml)
        dict_path = os.path.join(storage_path, "test.odict")
        with open(dict_path, "wb") as f:
            f.write(compiled_bytes)

        dict_obj = await OpenDictionary.load(storage_path, dict_path)
        assert dict_obj is not None

    def test_new_with_storage_path(self, storage_path):
        dict_xml = "<dictionary><entry term="test"><ety><sense><definition value="v"/></sense></ety></entry></dictionary>"
        dict_obj = OpenDictionary(storage_path, dict_xml)
        assert dict_obj is not None
