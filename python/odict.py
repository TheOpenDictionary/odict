import sys

from ctypes import *
from os import path


def __library_name():
    names = {
        "win32": "bridge.dll",
        "darwin": "libbridge.dylib",
    }
    return names.get(sys.platform, "libbridge.so")


def __find_library():
    # Workaround for for https://github.com/bazelbuild/rules_python/issues/382

    opt1 = path.abspath(path.join(path.dirname(__file__),
                                  "..", "bridge", "bridge_", __library_name()))

    opt2 = path.abspath(path.join("bridge", "bridge_", __library_name()))

    if path.isfile(opt2):
        return opt2
    else:
        return opt1


lib = cdll.LoadLibrary(__find_library())

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
