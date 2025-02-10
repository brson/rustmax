#!/bin/bash

set +euxo pipefail

TOOLCHAIN=1.84.1

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain $TOOLCHAIN

source ~/.cargo/env

cargo install mdbook@0.4.43
cargo install just@1.36.0

just build
