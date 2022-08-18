from os import environ, path
from distutils.sysconfig import get_config_var
from glob import glob
from pathlib import Path
from subprocess import run
from tempfile import NamedTemporaryFile

def exec(*args: list[str]):
  out = run(["../build/odict", *args], capture_output=True)

  if (out):
    print(out)


class Dictionary:
    def __init__(self, path):
        self.__path = path

    @staticmethod
    def compile(path):
        exec("compile", path)

    @staticmethod
    def write(xml, path):
        tmp = NamedTemporaryFile()
        
        with open(tmp.name, 'wb') as f:
          f.write(xml.encode('utf-8'))    
          exec("compile", "-o", path, tmp.name)
          print(path)
    # def search(self, query):
    #     if self.__id == None:
    #         self.index(force=True)

    #     __exec("search")

    #     v = lib.SearchDictionary(self.__encode(query), self.__id)
    #     d = self.__decode(cast(v, c_char_p).value)

    #     lib.free(v)

    #     return d

    # def index(self, force=False):
    #     self.__id = lib.IndexDictionary(self.__path, force)

    # def lookup(self, term):
    #     e = self.__encode(term)
    #     v = lib.LookupEntry(e, self.__dict_p)
    #     d = self.__decode(cast(v, c_char_p).value)

    #     lib.free(v)

    #     return d

    # def __encode(self, str):
    #     return str.encode("utf-8")

    # def __decode(self, str):
    #     return str.decode("utf-8")
    

