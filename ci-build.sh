#!/bin/bash

set +euxo pipefail

TOOLCHAIN=1.90.0
rustup default $TOOLCHAIN

# Nightly toolchain with rust-docs-json for API doc generation.
rustup toolchain install nightly
rustup component add --toolchain nightly rust-docs-json

cargo install just
RUSTMAX_CI=1 just build
