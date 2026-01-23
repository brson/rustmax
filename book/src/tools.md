# Rustmax: Tools

---

<!-- order of tools here is same is in library.html -->
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
- Non-Rust tools for Rust:
  [`mold`](#user-content--mold)
- More general developer tools:
  [`ripgrep`](#user-content--ripgrep)
  [`tokei`](#user-content--tokei)
  [`basic-http-server`](#user-content--basic-http-server)
  [`gist`](#user-content--gist)
  [`jaq`](#user-content--jaq)
  [`jsonxf`](#user-content--jsonxf)
  [`fd`](#user-content--fd)
  [`sd`](#user-content--sd)
  [`dust`](#user-content--dust)

---


## Standard Rust tools

### 🌞 `cargo`

The Rust build and packaging tool.
It is the central tool in most Rust development workflows.
It is part of every Rust toolchain,
usually managed by `rustup`.

> 👁️  [The `cargo` Book](https://doc.rust-lang.org/cargo/index.html)


### 🌞 `rustc`

The Rust compiler.
It is usually invoked through `cargo`.

> 👁️  [The `rustc` Book](https://doc.rust-lang.org/rustc/)


### 🌞 `rustup`

The Rust toolchain installer and version manager.

> 👁️  [The `rustup` Book](https://rust-lang.github.io/rustup/)


### 🌞 `rustfmt`

A tool for formatting Rust code.
Included with Rust toolchains.

```
rustup component add rustfmt
```

> 👁️  [The `rustfmt` Book](https://rust-lang.github.io/rustfmt/)


### 🌞 `clippy`

A collection of lints to catch common mistakes and improve your Rust code.

```
rustup component add clippy
```

> 👁️  [The `clippy` Book](https://doc.rust-lang.org/nightly/clippy/)


### 🌞 `rustdoc`

The Rust documentation generator.
Usually invoked through `cargo doc`.

> 👁️  [The `rustdoc` Book](https://doc.rust-lang.org/rustdoc/)


### 🌞 `mdbook`

A utility to create modern online books from Markdown files.

```
cargo install mdbook
```

> 🌞 [`crates.io` Page](https://crates.io/crates/mdbook)\
> 👁️  [The mdBook Book](https://rust-lang.github.io/mdBook/)


### 🌞 `bindgen`

Automatically generates Rust FFI bindings to C libraries.

```
cargo install bindgen-cli
```

> 🌞 [`crates.io` Page](https://crates.io/crates/bindgen)\
> 👁️  [The bindgen User Guide](https://rust-lang.github.io/rust-bindgen/)


### 🌞 `miri`

An interpreter for Rust's mid-level intermediate representation.
Useful for detecting undefined behavior.

```
rustup component add miri
```

> 👁️  [Source Repository](https://github.com/rust-lang/miri)






## Cargo plugins


### 🌞 `cargo-edit`

Extra `cargo` subcommands for editing `Cargo.toml`.

```
cargo install cargo-edit
```

> 🌞 [`crates.io` Page](https://crates.io/crates/cargo-edit)\
> 👁️  [Source Repository](https://github.com/killercup/cargo-edit)

---

Installing `cargo-edit` provides two `cargo` subcommands:

- [`cargo upgrade`](https://github.com/killercup/cargo-edit#cargo-upgrade)
- [`cargo set-version`](https://github.com/killercup/cargo-edit#cargo-set-version)

[`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html)
was once provided by `cargo-edit` but since Rust [`1.62.0`](https://blog.rust-lang.org/2022/06/30/Rust-1.62.0.html)
is built into `cargo` itself.



### 🌞 `cargo-clean-all`

Recursively clean all Cargo projects in a directory tree.

```
cargo install cargo-clean-all
```

> 🌞 [`crates.io` Page](https://crates.io/crates/cargo-clean-all)\
> 👁️  [Source Repository](https://github.com/dnaka91/cargo-clean-all)


### 🌞 `cargo-deny`

Cargo plugin for linting your dependencies.
Checks for security vulnerabilities, licenses, and more.

```
cargo install cargo-deny
```

> 🌞 [`crates.io` Page](https://crates.io/crates/cargo-deny)\
> 👁️  [Source Repository](https://github.com/EmbarkStudios/cargo-deny)


### 🌞 `cargo-license`

Displays the license of dependencies.

```
cargo install cargo-license
```

> 🌞 [`crates.io` Page](https://crates.io/crates/cargo-license)\
> 👁️  [Source Repository](https://github.com/onur/cargo-license)


### 🌞 `cargo-audit`

Audit Cargo.lock files for known security vulnerabilities.

```
cargo install cargo-audit
```

> 🌞 [`crates.io` Page](https://crates.io/crates/cargo-audit)\
> 👁️  [Source Repository](https://github.com/RustSec/rustsec)


### 🌞 `cargo-generate`

Generate a new Rust project from a template.

```
cargo install cargo-generate
```

> 🌞 [`crates.io` Page](https://crates.io/crates/cargo-generate)\
> 👁️  [Source Repository](https://github.com/cargo-generate/cargo-generate)






## More Rust tools

### 🌞 `clippy-control`

Temporarily allow/deny clippy lints from the command line.

```
cargo install clippy-control
```

> 🌞 [`crates.io` Page](https://crates.io/crates/clippy-control)\
> 👁️  [Source Repository](https://github.com/Ogeon/clippy-control)





## Non-Rust tools for Rust

### 🌞 `mold`

A high-performance linker that significantly speeds up Rust builds on Linux.

```
rustmax install-tool mold
```

> 👁️  [Source Repository](https://github.com/rui314/mold)

---

Linking is one of the most time-consuming stages of a Rust build,
and it has to be redone every time you test your program.
On Linux the `mold` linker is faster than the default linker.

Setting up `mold` manually requires configuring `.cargo/config.toml`
and ensuring the linker is properly installed,
but the Rustmax CLI tool handles this setup automatically.




## More general developer tools

### 🌞 `ripgrep`

A line-oriented search tool that recursively searches your current directory for a regex pattern.
Faster than grep and respects gitignore.

Documented in the legendary blog post, ["ripgrep is faster than ..."](https://burntsushi.net/ripgrep/).

```
cargo install ripgrep
```

> 🌞 [`crates.io` Page](https://crates.io/crates/ripgrep)\
> 👁️  [Source Repository](https://github.com/BurntSushi/ripgrep)



### 🌞 `just`

A simple and suprisingly useful command runner with `make`-like syntax.

```
cargo install just
```

> 🌞 [`crates.io` Page](https://crates.io/crates/just)\
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

$ just build
   Compiling rustmax-cli v0.0.5 (…/rustmax/crates/rustmax-cli)
   …
```




### 🌞 `tokei`

A program for counting lines of code quickly.

```
cargo install tokei
```

> 🌞 [`crates.io` Page](https://crates.io/crates/tokei)\
> 👁️  [Source Repository](https://github.com/XAMPPRocky/tokei)


### 🌞 `basic-http-server`

A simple HTTP server for serving static files.

```
cargo install basic-http-server
```

> 🌞 [`crates.io` Page](https://crates.io/crates/basic-http-server)\
> 👁️  [Source Repository](https://github.com/brson/basic-http-server)


### 🌞 `gist`

Upload code to GitHub Gist from the command line.

```
cargo install gist
```

> 🌞 [`crates.io` Page](https://crates.io/crates/gist)\
> 👁️  [Source Repository](https://github.com/defuz/gist)


### 🌞 `jaq`

A jq clone focused on correctness, speed, and simplicity.

```
cargo install jaq
```

> 🌞 [`crates.io` Page](https://crates.io/crates/jaq)\
> 👁️  [Source Repository](https://github.com/01mf02/jaq)


### 🌞 `jsonxf`

A JSON transformer and formatter.

```
cargo install jsonxf
```

> 🌞 [`crates.io` Page](https://crates.io/crates/jsonxf)\
> 👁️  [Source Repository](https://github.com/gamache/jsonxf)


### 🌞 `fd`

Find files recursively. A simple, fast and user-friendly alternative to 'find'.
Pair with [`sd`](#user-content--sd) to search and replace.

```
cargo install fd-find
```

> 🌞 [`crates.io` Page](https://crates.io/crates/fd-find)\
> 👁️  [Source Repository](https://github.com/sharkdp/fd)


### 🌞 `sd`

Intuitive find & replace CLI, `sed` alternative,
pair with [`fd`](#user-content--fd).

```
cargo install sd
```

> 🌞 [`crates.io` Page](https://crates.io/crates/sd)\
> 👁️  [Source Repository](https://github.com/chmln/sd)


### 🌞 `dust`

Show disk usage. A more intuitive version of `du`.

```
cargo install du-dust
```

> 🌞 [`crates.io` Page](https://crates.io/crates/du-dust)\
> 👁️  [Source Repository](https://github.com/bootandy/dust)


