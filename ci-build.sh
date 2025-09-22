#!/bin/bash

set +euxo pipefail

TOOLCHAIN=1.88.0
rustup default $TOOLCHAIN

cargo install mdbook
cargo install mdbook-yapp
RUSTMAX_CI=1 just build
