clean:
    cargo clean
    rm -rf out

check:
    #cargo deny check
    cargo check
    cargo check --features rmx-profile-no-std
    cargo check --features rmx-profile-std
    cargo check --features rmx-profile-full
    cargo check --features rmx-profile-max

doc-crate:
    cargo doc -p rmx --features rmx-profile-max

book:
    cd book && mdbook build

build: doc-crate book
    rm -rf out
    mkdir -p out/book
    mkdir -p out/api
    cp -r www/* out/
    cp -r book/book/* out/book/
    cp -r target/doc/* out/api/
