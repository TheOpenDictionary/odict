from os import environ, path, remove
from distutils.sysconfig import get_config_var
from glob import glob
from pathlib import Path
from subprocess import run
from tempfile import NamedTemporaryFile

def exec(*args: list[str]):
  out = run(["../build/odict", "--quiet", *args], capture_output=True)

  if out.stderr:
    raise Exception(out.stderr)
    
  return str(out.stdout.decode('utf-8'))


class Dictionary:
    def __init__(self, path):
        self.__path = path

    @staticmethod
    def compile(path):
        exec("compile", path)

    @staticmethod
    def write(xml, path):
        tmp = NamedTemporaryFile(delete=False)
        
        with open(tmp.name, 'wb') as f:
          f.write(xml.encode('utf-8'))    
        
        exec("compile", "-o", path, tmp.name)
        remove(tmp.name)
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

    def lookup(self, *terms):
        output = exec("lookup", self.__path, *terms)
        return output

    # def __encode(self, str):
    #     return str.encode("utf-8")

    # def __decode(self, str):
    #     return str.decode("utf-8")
    

