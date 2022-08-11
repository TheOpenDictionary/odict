from distutils.sysconfig import get_config_var
from glob import glob
import os
from pathlib import Path
from struct import Struct, unpack
import sys

from ctypes import *
from os import path

here = Path(__file__).absolute().parent

if os.environ["RUNTIME_ENV"] == "test":
    here = list(here.parent.joinpath("build").glob("**/*.so"))[0].parent

ext_suffix = get_config_var("EXT_SUFFIX")
so_file = here / ("_odict" + ext_suffix)


class DictionaryFile(Structure):
    _fields_ = [("version", c_uint), ("length", c_uint)]


lib = cdll.LoadLibrary(so_file)

lib.SearchDictionary.restype = c_void_p
lib.LookupEntry.restype = c_void_p
lib.ReadDictionary.restype = DictionaryFile
lib.free.argtypes = [c_void_p]
lib.free.restype = None


class Dictionary:
    def __init__(self, path, should_index=False):
        self.p = path.encode("utf-8")
        self.__encoded_dict = lib.ReadDictionary(self.p)
        print("poop", self.__encoded_dict.version, self.__encoded_dict.length)
        if should_index:
            self.index()

    # def __del__(self):
    #     lib.free(self.__encoded_dict)

    @staticmethod
    def compile(path):
        p = path.encode("utf-8")
        lib.CompileDictionary(p)

    @staticmethod
    def write(xml, path):
        try:
            lib.WriteDictionary(xml.encode("utf-8"), path.encode("utf-8"))
        except:
            print("An exception occurred")

    def search(self, query):
        v = lib.SearchDictionary(self.__encode(query), self.__get_dictionary())
        d = self.__decode(cast(v, c_char_p).value)

        lib.free(v)

        return d

    def index(self, force=False):
        lib.IndexDictionary(self.p, force)

    def lookup(self, term):
        e = self.__encode(term)
        v = lib.LookupEntry(e, self.__encoded_dict)
        print(v)
        # d = self.__decode(cast(v, c_char_p).value)

        lib.free(v)

        return v

    def __encode(self, str):
        return str.encode("utf-8")

    def __decode(self, str):
        return str.decode("utf-8")
