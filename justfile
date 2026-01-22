default:
    just --list


install-cli:
    cargo install --path crates/rustmax-cli

clean: doc-clean
    cargo clean

check:
    cargo check -p rustmax
    cargo check -p rustmax --features rmx-profile-no-std
    cargo check -p rustmax --features rmx-profile-std
    cargo check -p rustmax --features rmx-profile-full
    cargo check -p rustmax --features rmx-profile-max
    cargo check -p rustmax-cli
    cargo check -p rustmax --features=rmx-profile-portable --target=wasm32-unknown-unknown

test:
    cargo test -p rustmax --lib --tests --bins
    cargo test -p rustmax --features rmx-profile-std --lib --tests --bins
    cargo test -p rustmax --features rmx-profile-full --lib --tests --bins
    cargo test -p rustmax --features rmx-profile-max --lib --tests --bins
    cargo check -p rustmax --features=rmx-profile-portable --target=wasm32-unknown-unknown
    cargo run -p rustmax-cli -- doctest

test-rustdoc:
    cargo test -p rustmax --features rmx-profile-max --doc

test-musl:
    cargo check -p rustmax --target x86_64-unknown-linux-musl --features rmx-profile-portable

test-wasm:
    cargo check -p rustmax --target wasm32-unknown-unknown --features rmx-profile-portable

test-min-version-build: maint-lock-minimum-versions
    cargo test -p rustmax --features rmx-profile-std

test-ci-linux: test test-musl

test-ci-win: test

lint:
    cargo deny check
    cargo audit
    cargo clippy
    #cargo fmt --check

build: doc-build

maint-outdated:
    cargo outdated

maint-duplicates:
    cargo duplicates

maint-upgrade:
    cargo upgrade --incompatible --compatible=ignore

# useful prior to running `cargo audit`
maint-lock-minimum-versions:
    cargo +nightly update -Z minimal-versions

maint-audit:
    cargo audit



prebuild:
    cargo run -p rustmax-prebuild


doc-clean:
    rm -rf out

doc-api: prebuild
    # Generate JSON for rustmax and all dependencies.
    RUSTDOCFLAGS="-Z unstable-options --output-format json" cargo +nightly doc -p rustmax --features rmx-profile-max
    # Copy std library JSON files from nightly toolchain (requires rust-docs-json component).
    cp "$(rustup +nightly which rustc | sed 's|/bin/rustc|/share/doc/rust/json|')"/*.json target/doc/ 2>/dev/null || echo "Warning: rust-docs-json not installed, std links won't resolve"
    # Build HTML docs with rustmax-rustdoc.
    rm -rf out/api
    mkdir -p out/api
    cargo run -p rustmax-cli --release -- rustdoc build target/doc/ -o out/api

doc-book:
    rm -rf book/book
    cargo run --release -- rmxbook book book/book

doc-library: prebuild
    @if [ -n "${RUSTMAX_CI:-}" ]; then \
        cargo run -- refresh-library; \
    fi
    cargo run --release -- build-library --generate-library-page

doc-www: prebuild
    mkdir -p out/
    cp -r www/* out/
    cp work/crates.html out/
    cp work/build-info.json out/
    cp work/latest-post.html out/ || true
    cp work/news.html out/ || true
    cp work/news.xml out/ || true

doc-build: doc-www doc-api doc-book doc-library
    mkdir -p out/book
    cp -r book/book/* out/book/
    mkdir -p out/library
    cp -r work/library/* out/library/

publish-dry:
    cargo publish -p rustmax --dry-run
    cargo publish -p rustmax-doctest --dry-run
    cargo publish -p rustmax-rustdoc --dry-run
    cargo publish -p rustmax-cli --dry-run

publish:
    cargo publish -p rustmax
    cargo publish -p rustmax-doctest
    cargo publish -p rustmax-rustdoc
    cargo publish -p rustmax-cli --allow-dirty

replace-version old new:
    sd "^version = \"{{old}}\"" "version = \"{{new}}\"" Cargo.toml
    fd --type file --exec sd "^rmx\.version = \"{{old}}\"" "rmx.version = \"{{new}}\""
    cargo check # update lockfile

anthology-build: anthology-index

anthology-list:
    cargo run -p rustmax-anthology -- --metadata-dir crates/rustmax-anthology/metadata \
        --fetched-dir crates/rustmax-anthology/fetched list

anthology-status:
    cargo run -p rustmax-anthology -- --metadata-dir crates/rustmax-anthology/metadata \
        --fetched-dir crates/rustmax-anthology/fetched status

anthology-process id:
    cargo run -p rustmax-anthology -- --metadata-dir crates/rustmax-anthology/metadata \
        --fetched-dir crates/rustmax-anthology/fetched process {{id}}
    just anthology-index

anthology-process-all:
    cargo run -p rustmax-anthology -- --metadata-dir crates/rustmax-anthology/metadata \
        --fetched-dir crates/rustmax-anthology/fetched process all
    just anthology-index

anthology-build-book: anthology-index
    cargo run -p rustmax-anthology -- --metadata-dir crates/rustmax-anthology/metadata \
        --fetched-dir crates/rustmax-anthology/fetched \
        --book-dir crates/rustmax-anthology/book build

anthology-index:
    cargo run -p rustmax-anthology -- --metadata-dir crates/rustmax-anthology/metadata \
        --fetched-dir crates/rustmax-anthology/fetched generate-index
