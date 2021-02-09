import sys

from ctypes import *
from os import path


def __library_name():
    names = {
        "win32": "shared.dll",
        "darwin": "libshared.dylib",
    }
    return names.get(sys.platform, "libshared.so")


def __find_library():
    # Workaround for for https://github.com/bazelbuild/rules_python/issues/382

    opt1 = path.abspath(path.join(path.dirname(__file__),
                                  "..", "bridge", "shared_", __library_name()))

    opt2 = path.abspath(path.join("bridge", "shared_", __library_name()))

    if path.isfile(opt2):
        return opt2
    else:
        return opt1


lib = cdll.LoadLibrary(__find_library())

lib.SearchDictionary.restype = c_void_p
lib.LookupEntry.restype = c_void_p
lib.ReadDictionary.restype = c_void_p
lib.Free.argtypes = [c_void_p]
lib.Free.restype = None


class Dictionary:

    def __init__(self, path, should_index=False):
        self.p = path.encode('utf-8')
        self.__encoded_dict = lib.ReadDictionary(self.p)

        if should_index:
            self.index()

    def __del__(self):
        lib.Free(self.__encoded_dict)

    @staticmethod
    def compile(path):
        p = path.encode('utf-8')
        lib.CompileDictionary(p)

    @staticmethod
    def write(xml, path):
        try:
            lib.WriteDictionary(xml.encode('utf-8'), path.encode('utf-8'))
        except:
            print("An exception occurred")

    def __get_dictionary(self):
        return cast(self.__encoded_dict, c_char_p).value

    def search(self, query):
        v = lib.SearchDictionary(self.__encode(query), self.__get_dictionary())
        d = self.__decode(cast(v, c_char_p).value)

        lib.Free(v)

        return d

    def index(self):
        lib.IndexDictionary(self.p)

    def lookup(self, term):
        e = self.__encode(term)
        v = lib.LookupEntry(e, self.__get_dictionary())
        d = self.__decode(cast(v, c_char_p).value)

        lib.Free(v)

        return d

    def __encode(self, str):
        return str.encode('utf-8')

    def __decode(self, str):
        return str.decode('utf-8')
