default:
    just --list



clean:
    cargo clean
    rm -rf out

check:
    cargo check
    cargo check --features rmx-profile-no-std
    cargo check --features rmx-profile-std
    cargo check --features rmx-profile-full
    cargo check --features rmx-profile-max

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



doc-crates:
    RUSTDOCFLAGS="--html-in-header $(pwd)/www/mixins/mixin-rustdoc-header.html" \
      cargo doc -p rmx --features rmx-profile-max
    cp www/mixins/mixin-rustdoc-themes.css target/doc/
    cp www/mixins/mixin-rustdoc-script.js target/doc/
    cp www/rustmax-themes.css target/doc/

doc-book:
    rm -rf book/book
    cd book && mdbook build
    cp www/mixins/mixin-mdbook-style.css book/book/
    cp www/mixins/mixin-mdbook-script.js book/book/
    cp www/rustmax-themes.css book/book/

doc-build: doc-crates doc-book
    rm -rf book/yapp~
    rm -rf out
    mkdir -p out/book
    mkdir -p out/api
    cp -r www/* out/
    cp -r book/book/* out/book/
    cp -r target/doc/* out/api/
