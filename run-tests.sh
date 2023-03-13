#!/bin/bash

THIS_DIR=$(cd $(dirname $0) && pwd -P)

function test()
{
    cd $THIS_DIR
    cd $1 || exit 1
    echo
    echo "========================================"
    echo "Running tests in $1 ..."
    cargo test || exit 1
}

test practice-01-solution
test practice-02-solution
test practice-03-solution
