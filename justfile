default:
    just --list


install-tools:
    cargo install mdbook
    cargo install mdbook-yapp


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

doc-book:
    rm -rf book/book
    cd book && mdbook build
    # same as in mod_book_style
    cp www/mixins/mixin-mdbook-style.css book/book/
    cp www/mixins/mixin-mdbook-script.js book/book/
    cp www/rustmax-themes.css book/book/

doc-build: doc-crates doc-book
    rm -rf book/yapp~
    #rm -rf out
    mkdir -p out/book
    mkdir -p out/api
    cp -r www/* out/
    cp -r book/book/* out/book/
    cp -rlf target/doc/* out/api/
    cp work/crates.html out/

publish-dry:
    cargo publish -p rustmax --dry-run
    cargo publish -p rustmax-cli --dry-run

publish:
    cargo publish -p rustmax
    cargo publish -p rustmax-cli

replace-version old new:
    sd "^version = \"{{old}}\"" "version = \"{{new}}\"" Cargo.toml
    fd --type file --exec sd "^rmx\.version = \"{{old}}\"" "version = \"{{new}}\"" Cargo.toml
