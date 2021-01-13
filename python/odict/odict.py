import sys

from ctypes import *
from os import path, getcwd


def __library_name():
    names = {
        "win32": "bridge.dll",
        "darwin": "libbridge.dylib",
    }
    return names.get(sys.platform, "libbridge.so")


odict = cdll.LoadLibrary(path.abspath(
    path.join(path.dirname(getcwd()), "__main__", "bridge", "bridge_", __library_name())))
