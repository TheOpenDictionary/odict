from json import loads
from os import environ, remove
from subprocess import run
from tempfile import NamedTemporaryFile
from typing import List


def exec(*args: List[str]):
    out = run(
        [
            "../bin/odict" if environ.get("RUNTIME_ENV") == "test" else "odict",
            "--quiet",
            *args,
        ],
        capture_output=True,
    )
    
    if out.stderr:
        raise Exception(out.stderr)

    return str(out.stdout.decode("utf-8"))


class Dictionary:
    def __init__(self, path):
        self.__path = path

    @staticmethod
    def compile(path):
        exec("compile", path)

    @staticmethod
    def write(xml, path):
        tmp = NamedTemporaryFile(delete=False)

        with open(tmp.name, "wb") as f:
            f.write(xml.encode("utf-8"))

        exec("compile", "-o", path, tmp.name)
        remove(tmp.name)

    def lexicon(self):
        list = exec("lexicon", self.__path)
        return list.splitlines()
    
    def search(self, query, index: bool = False):
        if index:
            return loads(exec("search", "-i", self.__path, query))
        else:
            return loads(exec("search", self.__path, query))

    def index(self):
        exec("index", self.__path)

    def lookup(self, *terms, **kwargs):
        args = ["lookup", "-f", "json"]

        if kwargs.get("follow"):
            args.append("-F")

        return loads(exec(*args, self.__path, *terms))
