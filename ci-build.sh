#!/bin/bash

set +euxo pipefail

TOOLCHAIN=1.90.0
rustup default $TOOLCHAIN

cargo install just
RUSTMAX_CI=1 just build
