default:
    just --list


install-tools:
    cargo install mdbook
    cargo install mdbook-yapp

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
    cargo check -p rustmax --features=rmx-profile-wasm --target=wasm32-unknown-unknown

test:
    cargo test -p rustmax
    #cargo test -p rmx --features rmx-profile-no-std
    cargo test -p rustmax --features rmx-profile-std
    cargo test -p rustmax --features rmx-profile-full
    cargo test -p rustmax --features rmx-profile-max
    cargo check -p rustmax --features=rmx-profile-wasm --target=wasm32-unknown-unknown

test-min-version-build: maint-lock-minimum-versions
    cargo test -p rustmax --features rmx-profile-std


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

doc-crates: prebuild
    # Copy standard library docs first (dynamically find toolchain location)
    mkdir -p target/doc
    cp -a "$(rustup which rustc | sed 's|/bin/rustc|/share/doc/rust/html|')/"* target/doc/
    RUSTDOCFLAGS="--html-in-header $(pwd)/www/mixins/mixin-rustdoc-header.html" \
      cargo doc -p rustmax --features rmx-profile-max
    cp www/mixins/mixin-rustdoc-themes.css target/doc/
    cp www/mixins/mixin-rustdoc-script.js target/doc/
    cp www/rustmax-themes.css target/doc/
    cp work/crates.json target/doc/
    cp -r www/fonts target/doc/

doc-book:
    rm -rf book/book
    cd book && mdbook build
    # same as in mod_book_style
    cp www/mixins/mixin-mdbook-style.css book/book/
    cp www/mixins/mixin-mdbook-script.js book/book/
    cp www/rustmax-themes.css book/book/

doc-install-library-deps:
    cargo run -- install-library-deps

doc-library: prebuild
    @if [ -n "${RUSTMAX_CI:-}" ]; then \
        cargo run -- refresh-library; \
        cargo run -- install-library-deps; \
    fi
    cargo run -- build-library --generate-library-page;
    @find work/library -mindepth 1 -maxdepth 1 -type d -exec cp www/mixins/mixin-mdbook-style.css {} \;
    @find work/library -mindepth 1 -maxdepth 1 -type d -exec cp www/mixins/mixin-mdbook-script.js {} \;
    @find work/library -mindepth 1 -maxdepth 1 -type d -exec cp www/rustmax-themes.css {} \;

doc-www: prebuild
    mkdir -p out/
    cp -r www/* out/
    cp work/crates.html out/
    cp work/build-info.json out/
    cp work/latest-post.html out/ || true
    cp work/feed.html out/ || true
    cp work/feed.xml out/ || true

doc-build: doc-www doc-crates doc-book doc-library
    mkdir -p out/book
    cp -r book/book/* out/book/
    mkdir -p out/api
    cp -rlf target/doc/* out/api/
    mkdir -p out/library
    cp -r work/library/* out/library/

anthology-list:
    cargo run -p rustmax-anthology -- --metadata-dir crates/rustmax-anthology/metadata \
        --fetched-dir crates/rustmax-anthology/fetched list

anthology-status:
    cargo run -p rustmax-anthology -- --metadata-dir crates/rustmax-anthology/metadata \
        --fetched-dir crates/rustmax-anthology/fetched status

anthology-process id:
    cargo run -p rustmax-anthology -- --metadata-dir crates/rustmax-anthology/metadata \
        --fetched-dir crates/rustmax-anthology/fetched process {{id}}

anthology-process-all:
    cargo run -p rustmax-anthology -- --metadata-dir crates/rustmax-anthology/metadata \
        --fetched-dir crates/rustmax-anthology/fetched process all

anthology-build:
    cargo run -p rustmax-anthology -- --metadata-dir crates/rustmax-anthology/metadata \
        --fetched-dir crates/rustmax-anthology/fetched \
        --book-dir crates/rustmax-anthology/book build

anthology-index:
    cargo run -p rustmax-anthology -- --metadata-dir crates/rustmax-anthology/metadata \
        --fetched-dir crates/rustmax-anthology/fetched generate-index

publish-dry:
    cargo publish -p rustmax --dry-run
    cargo publish -p rustmax-cli --dry-run

publish:
    cargo publish -p rustmax
    cargo publish -p rustmax-cli

replace-version old new:
    sd "^version = \"{{old}}\"" "version = \"{{new}}\"" Cargo.toml
    fd --type file --exec sd "^rmx\.version = \"{{old}}\"" "rmx.version = \"{{new}}\""
    cargo check # update lockfile
