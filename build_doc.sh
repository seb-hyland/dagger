#!/bin/bash
set -eu pipefail

cargo clean --doc
cargo doc --no-deps --workspace
