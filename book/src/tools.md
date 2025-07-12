# Rust Max: Tools

---

<!-- order of tools here is same is in library.md -->
- Standard Rust tools:
  [`cargo`](#user-content--cargo)
  [`rustc`](#user-content--rustc)
  [`rustup`](#user-content--rustup)
  [`rustdoc`](#user-content--rustdoc)
  [`rustfmt`](#user-content--rustfmt)
  [`clippy`](#user-content--clippy)
  [`just`](#user-content--just)
  [`mdbook`](#user-content--mdbook)
  [`bindgen`](#user-content--bindgen)
  [`miri`](#user-content--miri)
- Cargo plugins:
  [`cargo-edit`](#user-content--cargo-edit)
  [`cargo-clean-all`](#user-content--cargo-clean-all)
  [`cargo-deny`](#user-content--cargo-deny)
  [`cargo-license`](#user-content--cargo-license)
  [`cargo-audit`](#user-content--cargo-audit)
  [`cargo-generate`](#user-content--cargo-generate)
- More Rust tools:
  [`clippy-control`](#user-content--clippy-control)
- More tools:
  [`ripgrep`](#user-content--ripgrep)
  [`tokei`](#user-content--tokei)
  [`basic-http-server`](#user-content--clippy-control)
  [`gist`](#user-content-gist)
  [`jaq`](#user-content-jaq)
  [`jsonxf`](#user-content-jsonxf)
  [`fd`](#user-content--fd)
  [`sd`](#user-content--sd)

---

todo say something here


## Standard Rust tools

### 📦 `cargo`

The Rust build and packaging tool.
It is the central tool in most Rust development workflows.
It is part of every Rust toolchain,
usually managed by `rustup`.

> 👁️  [The `cargo` Book](https://doc.rust-lang.org/cargo/index.html)


### 📦 `rustc`
### 📦 `rustup`
### 📦 `rustfmt`
### 📦 `mdbook`
### 📦 `bindgen`
### 📦 `miri`




## Cargo plugins


### 📦 `cargo-edit`

Extra `cargo` subcommands for editing `Cargo.toml`.

```
cargo install cargo-edit
```

> 🥡 [`crates.io` Page](https://crates.io/crates/cargo-edit)\
> 👁️  [Source Repository](https://github.com/killercup/cargo-edit)

---

Installing `cargo-edit` provides two `cargo` subcommands:

- [`cargo upgrade`](https://github.com/killercup/cargo-edit#cargo-upgrade)
- [`cargo set-version`](https://github.com/killercup/cargo-edit#cargo-set-version)

[`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html)
was once provided by `cargo-edit` but since Rust [`1.62.0`](https://blog.rust-lang.org/2022/06/30/Rust-1.62.0.html)
is built into `cargo` itself.



### 📦 `cargo-clean-all`
### 📦 `cargo-deny`
### 📦 `cargo-license`
### 📦 `cargo-audit`
### 📦 `cargo-generate`




## More Rust-specific tools

### 📦 `clippy-control`




## More Rust tools

### 📦 `ripgrep`

### 📦 `just`

A simple and suprisingly useful command runner with `make`-like syntax.

```
cargo install just
```

> 🥡 [`crates.io` Page](https://crates.io/crates/just)\
> 👁️  [Source Repository](https://github.com/casey/just)

---

Almost every project has a handful of commands the developer(s)
uses frequently. Put these in a `justfile` so the menu of
commands for this project is always obvious, which
can be extra helpful after years away from a project.

`just` runs commands listed in a file named `justfile`.
The `justfile` lives your project's root directory,
and is configured with a `make`-like syntax:

```just
default:
    just --list

install-tools:
    cargo install mdbook
    cargo install mdbook-yapp

clean: doc-clean
    cargo clean

doc-clean:
    rm -rf out
```

It's a simple idea, but suprisingly useful. And don't worry that it looks like
a `Makefile` &mdash; it is much more fun and sensible in use than `make`.

When you come back to a project and see there's a justfile you
know to run `just --list` and you'll immediately see what
was on the previous maintainer's mind.

```
$ just --list
Available recipes:
    build
    check
    clean
    default
    doc-book
    doc-build
    doc-clean
    doc-crates
    install-tools
    lint
    maint-audit
    maint-duplicates
    maint-lock-minimum-versions # useful prior to running `cargo audit`
    maint-outdated
    maint-upgrade
    prebuild
    publish
    publish-dry
    replace-version old new
    test
    test-min-version-build
```




### 📦 `tokei`
### 📦 `basic-http-server`
### 📦 `gist`
### 📦 `jaq`
### 📦 `jsonxf`
### 📦 `fd`
### 📦 `sd`
