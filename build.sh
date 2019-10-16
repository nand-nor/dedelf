#!/bin/bash

set -e

BASE_DIR=$(dirname "$(readlink -f "$0")")

function build_tests {
    cd tests
    make tests
    exit 0

}

function build_nasm {
    cd tests
    make nasm
    exit 0
}

function build_ldpreload {
    cd shell_asm
    make ldpreload
    exit 0
}

function build_all {
    cd tests
    make all
    exit 0
}

function clean {
    cd tests
    make clean
    exit 0
}

function build_dissassemble {
    cd tests
    make dissassemble
    exit 0
}

#function invalid_input{

#}

function unknown_subcommand {
  echo Input subcommand: "$1"  unknown or invalid
  exit 1

}


if [ "$1" == "c_tests" ]; then
        build_tests
elif [ "$1" == "nasm" ]; then
        build_nasm
elif [ "$1" == "ldpreload" ]; then
        build_ldpreload
elif [ "$1" == "dissassemble" ]; then
        build_dissassemble
elif [ "$1" == "all" ]; then
        build_all
elif [ "$1" == "help" -o "$1" == "-h" -o "$1" == "--help" ]; then
        echo "$(basename $0) [subcommand]"
        echo
        echo "Valid subcommands:"
        echo "c_tests     - build test C code for injection"
        echo "nasm        - build assembly code for injection"
        echo "ldpreload   - build ldpreload libraries for hooking"
        echo "all         - build all files for c_tests, nasm, and ldpreload"
        echo "clean       - remove all executable and linkable object files from test subdir"
        echo "help        - display this message"
else
    unknown_subcommand

fi