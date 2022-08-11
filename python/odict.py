from os import environ, path
from distutils.sysconfig import get_config_var
from glob import glob
from pathlib import Path
from ctypes import *

here = Path(__file__).absolute().parent

if environ["RUNTIME_ENV"] == "test":
    here = list(here.parent.joinpath("build").glob("**/*.so"))[0].parent

ext_suffix = get_config_var("EXT_SUFFIX")
so_file = here / ("_odict" + ext_suffix)


class DictionaryFile(Structure):
    _fields_ = [("version", c_uint16), ("length", c_uint16)]


lib = cdll.LoadLibrary(so_file)

lib.SearchDictionary.restype = c_void_p
lib.LookupEntry.argtypes = [c_char_p, POINTER(DictionaryFile)]
lib.LookupEntry.restype = c_void_p
lib.IndexDictionary.restype = c_char_p
lib.ReadDictionary.restype = POINTER(DictionaryFile)
lib.free.argtypes = [c_void_p]
lib.free.restype = None


class Dictionary:
    def __init__(self, path, should_index=False):
        self.__path = path.encode("utf-8")
        self.__dict_p = lib.ReadDictionary(self.__path)
        self.__dict = self.__dict_p.contents
        self.__id = None

        if should_index:
            self.index()

    def __del__(self):
        lib.free(self.__dict_p)

    @staticmethod
    def compile(path):
        __path = path.encode("utf-8")
        lib.CompileDictionary(__path)

    @staticmethod
    def write(xml, path):
        try:
            lib.WriteDictionary(xml.encode("utf-8"), path.encode("utf-8"))
        except:
            print("An exception occurred")

    def search(self, query):
        if self.__id == None:
            self.index(force=True)

        v = lib.SearchDictionary(self.__encode(query), self.__id)
        d = self.__decode(cast(v, c_char_p).value)

        lib.free(v)

        return d

    def index(self, force=False):
        self.__id = lib.IndexDictionary(self.__path, force)

    def lookup(self, term):
        e = self.__encode(term)
        v = lib.LookupEntry(e, self.__dict_p)
        d = self.__decode(cast(v, c_char_p).value)

        lib.free(v)

        return d

    def __encode(self, str):
        return str.encode("utf-8")

    def __decode(self, str):
        return str.decode("utf-8")
