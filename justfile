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
    RUSTDOCFLAGS="--html-in-header $(pwd)/www/rustdoc-mixins/mixin-header.html" \
      cargo doc -p rmx --features rmx-profile-max
    cp www/rustdoc-mixins/mixin-themes.css target/doc/
    cp www/rustmax-themes.css target/doc/

doc-book:
    rm -rf book/book
    cd book && mdbook build

build: doc-crates doc-book
    rm -rf book/yapp~
    rm -rf out
    mkdir -p out/book
    mkdir -p out/api
    cp -r www/* out/
    cp -r book/book/* out/book/
    cp -r target/doc/* out/api/
