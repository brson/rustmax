# The Rust Max Library

The Rust language and its ecosystem is documented in "books"
(rendered with [`mdbook`](todo)).

Links with a bookmark icon, 🔖, are to particularly
notable or useful chapters within a book.


## The Rust language

- **[The Rust Programming Language (The Book)](https://doc.rust-lang.org/book/)**
- **[Rust By Example](https://doc.rust-lang.org/rust-by-example/)**
- **[The Rust Reference](https://doc.rust-lang.org/reference/)**
  - 🔖 [Behavior considered undefined](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)
- **[The Rustonomicon](https://doc.rust-lang.org/nomicon/)**
- [The Rust Edition Guide](https://doc.rust-lang.org/edition-guide/index.html)
- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Rust Unsafe Code Guidelines](https://rust-lang.github.io/unsafe-code-guidelines/)
- [Rust Error Codes Index](https://doc.rust-lang.org/stable/error_codes/error-index.html)
- [The Rust Unstable Book](https://doc.rust-lang.org/unstable-book/)

> Shout-out to [cheats.rs](https://cheats.rs/),
  summarizing much of Rust on one big page.
  Take it all in!


## The Rust standard library

- **[`std`](https://doc.rust-lang.org/std/index.html)**
  <!-- duplicated in std.md -->
  - 🔖 [`std::collections`](https://doc.rust-lang.org/std/collections/index.html)
  - 🔖 [`std::error`](https://doc.rust-lang.org/stable/std/error/index.html)
  - 🔖 [`std::sync`](https://doc.rust-lang.org/std/sync/index.html)
  - 🔖 [`std::sync::atomic`](https://doc.rust-lang.org/std/atomic/index.html)
- [`core`](https://doc.rust-lang.org/core/index.html)
- [`alloc`](https://doc.rust-lang.org/alloc/index.html)
- [`proc_macro`](https://doc.rust-lang.org/proc_macro/index.html)


## Standard Rust tools

<!-- order here is same is in tools.md -->
- [The `cargo` Book](https://doc.rust-lang.org/cargo/index.html)
  - 🔖 [The manifest format](https://doc.rust-lang.org/cargo/reference/manifest.html)
  - 🔖 [Environment variables](https://doc.rust-lang.org/cargo/reference/environment-variables.html) that affect the Cargo build process.
- [The `rustc` Book](https://doc.rust-lang.org/rustc/index.html)
  - 🔖 [The lint system and built-in lints](https://doc.rust-lang.org/nightly/rustc/lints/index.html)
  - 🔖 [Rust platform support tiers](https://doc.rust-lang.org/nightly/rustc/platform-support.html)
- [The `rustup` Book](https://rust-lang.github.io/rustup/index.html)
- [The `rustdoc` Book](https://doc.rust-lang.org/stable/rustdoc/)
- rustfmt (todo)
- [The `clippy` Book](https://doc.rust-lang.org/nightly/clippy/development/infrastructure/book.html)
- [The `just` Programmer's Manual](https://just.systems/man/en/)
- [The `mdbook` Book](https://rust-lang.github.io/mdBook/)
- [The `bindgen` User Guide](https://rust-lang.github.io/rust-bindgen)
- miri (todo)

## The Rust crate ecosystem

- [The `rand` Book](https://rust-random.github.io/book/)
- [The `serde` Book](https://serde.rs/)
- [The Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)

## The Rust Project internals

- [Rust Project Goals](https://rust-lang.github.io/rust-project-goals/)
- [Guide to `rustc` Development](https://rustc-dev-guide.rust-lang.org/)
- [Standard Library Developers Guide](https://std-dev-guide.rust-lang.org/about.html)
- [Rust Forge](https://forge.rust-lang.org/)
  - 🔖 [Alternative Rust Installation Methods](https://forge.rust-lang.org/infra/other-installation-methods.html)
- [Rust RFCs](https://rust-lang.github.io/rfcs/)
