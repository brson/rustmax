#!/bin/bash

set +euxo pipefail

TOOLCHAIN=1.88.0

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain $TOOLCHAIN

source ~/.cargo/env

cargo install just@1.36.0

just install-tools
just build
