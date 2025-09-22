#!/bin/bash

set +euxo pipefail

# Unfortunate this is needed.
sudo apt install libasound2-dev

TOOLCHAIN=1.88.0
rustup toolchain default $TOOLCHAIN

cargo install just@1.36.0

just install-tools
RUSTMAX_CI=1 just build
