from distutils.errors import CompileError
from pathlib import Path
from subprocess import call
from os import path
from setuptools import Extension, setup
from setuptools.command.build_ext import build_ext


class build_go_ext(build_ext):
    def build_extension(self, ext):
        ext_path = Path(self.get_ext_fullpath(ext.name))
        ext_path = ext_path.parent.joinpath("odict", ext_path.name).absolute()
        cmd = ["make", "OUTPUT=%s" % ext_path, "lib", "-B"]
        out = call(cmd)

        if out != 0:
            raise CompileError("Go build failed")


def build(setup_kwargs):
    setup_kwargs.update(
        dict(
            cmdclass=dict(build_ext=build_go_ext),
            ext_modules=[Extension("_odict", ["go", "lib", "Makefile"])],
            zip_safe=False,
        )
    )
