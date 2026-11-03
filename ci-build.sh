#!/bin/bash

set -euxo pipefail

TOOLCHAIN=1.97.1
rustup default $TOOLCHAIN

# The pinned nightly for API docs is installed by the justfile's doc-api recipe.
cargo install just
RUSTMAX_CI=1 just build
