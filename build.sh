#!/usr/bin/env bash

function buildExec() {
    mkdir -p build/bin && cd build && cmake .. && make
}

function buildSchema() {
    flatc -c -o src src/schema.fbs
}

if [ "$#" -eq "0" ]
    then
        buildSchema
        buildExec
    else
        $1
fi
