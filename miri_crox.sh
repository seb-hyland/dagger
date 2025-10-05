#!/bin/sh
rm -rf crox/

export MIRIFLAGS="-Zmiri-measureme=crox"
cargo +nightly miri run "$@"

crox_file=$(ls crox/)
crox crox/"$crox_file" --minimum-duration 2500
