# Rust Max: Tools

<!-- order of tools here is same is in library.md -->
- Standard Rust tools:
  [`cargo`](#user-content--cargo)
  [`rustc`](#user-content--rustc)
  [`rustup`](#user-content--rustup)
  [`rustdoc`](#user-content--rustdoc)
  [`rustfmt`](#user-content--rustfmt)
  [`clippy`](#user-content--clippy)
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
  [`just`](#user-content--just)
  [`tokei`](#user-content--tokei)
  [`basic-http-server`](#user-content--clippy-control)
  [`gist`](#user-content-gist)
  [`jaq`](#user-content-jaq)
  [`jsonxf`](#user-content-jsonxf)
  [`fd`](#user-content--fd)
  [`sd`](#user-content--sd)



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
### 📦 `tokei`
### 📦 `basic-http-server`
### 📦 `gist`
### 📦 `jaq`
### 📦 `jsonxf`
### 📦 `fd`
### 📦 `sd`
