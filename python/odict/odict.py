from json import loads
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
    
  print(str(out.stdout.decode('utf-8')))
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
      
    def search(self, query, index: bool = False):
        if index:
          return loads(exec("search", "-i", self.__path, query))
        else:
          return loads(exec("search", self.__path, query))

    def index(self):
        exec("index", self.__path)

    def lookup(self, *terms):
        return loads(exec("lookup", self.__path, *terms))

