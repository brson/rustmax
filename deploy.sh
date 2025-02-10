#!/bin/bash

set +euxo pipefail

cargo install mdbook@0.4.43
cargo install just@1.36.0

just build
