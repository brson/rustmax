#!/bin/bash

set -euxo pipefail

TOOLCHAIN=1.97.1
rustup default $TOOLCHAIN

# The pinned nightly for API docs is installed by the justfile's doc-api recipe.
cargo install just
# Netlify sets CONTEXT to "production" only for the main site deploy,
# not for deploy previews or branch deploys.
if [ "${CONTEXT:-}" = production ]; then
    RUSTMAX_CI=1 just build-production
else
    RUSTMAX_CI=1 just build
fi
