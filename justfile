default:
    just --list


install-tools:
    cargo install mdbook
    cargo install mdbook-yapp


clean: doc-clean
    cargo clean

check:
    cargo check -p rmx
    cargo check -p rmx --features rmx-profile-no-std
    cargo check -p rmx --features rmx-profile-std
    cargo check -p rmx --features rmx-profile-full
    cargo check -p rmx --features rmx-profile-max
    cargo check -p rmx-cli

test:
    cargo test -p rmx
    #cargo test -p rmx --features rmx-profile-no-std
    cargo test -p rmx --features rmx-profile-std
    cargo test -p rmx --features rmx-profile-full
    cargo test -p rmx --features rmx-profile-max

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

maint-update:
    cargo update

maint-upgrade:
    cargo upgrade

maint-upgrade-incompatible:
    cargo upgrade --incompatible


prebuild:
    cargo run -p rmx-prebuild


doc-clean:
    rm -rf out

doc-crates: prebuild
    RUSTDOCFLAGS="--html-in-header $(pwd)/www/mixins/mixin-rustdoc-header.html" \
      cargo doc -p rmx --features rmx-profile-max
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
