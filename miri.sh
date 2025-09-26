#!/bin/sh
export MIRIFLAGS="-Zmiri-many-seeds=0..10 \
                -Zmiri-strict-provenance \
                -Zmiri-symbolic-alignment-check \
                -Zmiri-tree-borrows \
                -Zmiri-backtrace=full"

cargo +nightly miri run "$@"
