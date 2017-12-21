#!/usr/bin/env bash
mkdir -p bin
buck build :libodict --out ./bin/libodict.a
buck build :odict#strip-all --out ./bin/odict
