#!/bin/bash

set -euxo pipefail

TOOLCHAIN=1.90.0
rustup default $TOOLCHAIN

# The pinned nightly for API docs is installed by the justfile's doc-api recipe.
cargo install just
RUSTMAX_CI=1 just build
