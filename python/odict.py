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


class Dictionary:

    def __init__(self, path, should_index=False):
        self.__encoded_dict = lib.ReadDictionary(__encode(path))

        if should_index:
            self.index()

    @staticmethod
    def compile(path):
        lib.CompileDictionary(path.encode('utf-8'))

    @staticmethod
    def write(xml, path):
        lib.WriteDictionary(xml.encode('utf-8'), path.encode('utf-8'))

    def search(self, query):
        return __decode(lib.SearchDictionary(__encode(query), self.__encoded_dict))

    def index(self):
        lib.IndexDictionary(self.__encoded_dict)

    def lookup_entry(self, term):
        return __decode(lib.LookupEntry(__encode(term), self.__encoded_dict))
