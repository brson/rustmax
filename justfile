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

test:
    cargo test -p rustmax
    #cargo test -p rmx --features rmx-profile-no-std
    cargo test -p rustmax --features rmx-profile-std
    cargo test -p rustmax --features rmx-profile-full
    cargo test -p rustmax --features rmx-profile-max

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
    @if [ "${RUSTMAX_CI:-}" = "true" ]; then \
        cargo run -- refresh-library; \
        cargo run -- install-library-deps; \
    else \
        cargo run -- build-library --generate-library-page; \
    fi

doc-build: doc-crates doc-library doc-book
    rm -rf book/yapp~
    #rm -rf out
    mkdir -p out/book
    mkdir -p out/api
    mkdir -p out/books
    cp -r www/* out/
    cp -r book/book/* out/book/
    cp -rlf target/doc/* out/api/
    cp work/crates.html out/
    cp work/build-info.json out/

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
