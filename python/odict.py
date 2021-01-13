import sys

from ctypes import *
from os import path, getcwd


def __library_name():
    names = {
        "win32": "bridge.dll",
        "darwin": "libbridge.dylib",
    }
    return names.get(sys.platform, "libbridge.so")


lib = cdll.LoadLibrary(path.abspath(
    path.join(path.dirname(getcwd()), "__main__", "bridge", "bridge_", __library_name())))

lib.SearchDictionary.restype = c_char_p
lib.LookupEntry.restype = c_char_p


def __encode(str):
    return str.encode('utf-8')


def __decode(str):
    return str.decode('utf-8')


def compile_dictionary(path):
    lib.CompileDictionary(__encode(path))


def search_dictionary(query, path):
    return __decode(lib.SearchDictionary(__encode(query), __encode(path)))


def index_dictionary(path):
    return lib.IndexDictionary(__encode(path))


def lookup_entry(term, path):
    return __decode(lib.LookupEntry(__encode(term), __encode(path)))


def write_dictionary(xml, path):
    lib.WriteDictionary(__encode(xml), __encode(path))
