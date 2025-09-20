# The Rust Max Library

The Rust language and its ecosystem is documented in "books"
(rendered with [`mdbook`]), and most of these links are to books.

Links with a bookmark icon, 🔖, are to particularly
notable or useful chapters within a book.

## The Rust language

- **[The Rust Programming Language](../library/trpl/)** ([source](https://github.com/rust-lang/book))
- **[Rust By Example](../library/rust-by-example/)** ([source](https://github.com/rust-lang/rust-by-example))
- **[The Rust Reference](../library/reference/)** ([source](https://github.com/rust-lang/reference))
  - 🔖 [Conditional compilation](../library/reference/conditional-compilation.html).
       Including which cfgs are set by rustc.
  - 🔖 [Behavior considered undefined](../library/reference/behavior-considered-undefined.html)
- **[The Rustonomicon](../library/nomicon/)** ([source](https://github.com/rust-lang/nomicon))
- [The Rust Edition Guide](../library/edition-guide/) ([source](https://github.com/rust-lang/edition-guide))
- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/)
- [Rust API Guidelines](../library/api-guidelines/) ([source](https://github.com/rust-lang/api-guidelines))
- [Rust Unsafe Code Guidelines](../library/unsafe-code-guidelines/) ([source](https://github.com/rust-lang/unsafe-code-guidelines))
- [Rust Error Codes Index](https://doc.rust-lang.org/stable/error_codes/error-index.html)
- [The Rust Unstable Book](https://doc.rust-lang.org/unstable-book/)
- [The Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/index.html)
- [Rust Release Notes](https://doc.rust-lang.org/nightly/releases.html)

## The Rust standard library

- **[`std`](https://doc.rust-lang.org/std/index.html)**
  <!-- duplicated in std.md -->
  - 🔖 [`std::collections`](https://doc.rust-lang.org/std/collections/index.html)
  - 🔖 [`std::error`](https://doc.rust-lang.org/stable/std/error/index.html)
  - 🔖 [`std::ptr`](https://doc.rust-lang.org/std/ptr/index.html).
    Safety, undefined behavior, and "provenance".
  - 🔖 [`std::sync`](https://doc.rust-lang.org/std/sync/index.html)
  - 🔖 [`std::sync::atomic`](https://doc.rust-lang.org/std/atomic/index.html)
- [`core`](https://doc.rust-lang.org/core/index.html)
- [`alloc`](https://doc.rust-lang.org/alloc/index.html)
- [`proc_macro`](https://doc.rust-lang.org/proc_macro/index.html)


## Standard Rust tools

<!-- order here is same is in tools.md -->
- [The Cargo Book](../library/cargo-book/) ([source](https://github.com/rust-lang/cargo))
  - 🔖 [The manifest format](../library/cargo-book/reference/manifest.html)
  - 🔖 [Environment variables](../library/cargo-book/reference/environment-variables.html)
    that affect the Cargo build process.
  - 🔖 [Configuration format](../library/cargo-book/reference/config.html).
    Cargo has many interesting configuration options.
- [The rustc Book](../library/rustc-book/) ([source](https://github.com/rust-lang/rust))
  - 🔖 [The lint system and built-in lints](../library/rustc-book/lints/index.html)
  - 🔖 [Rust platform support tiers](../library/rustc-book/platform-support.html)
- [The `rustup` Book](https://rust-lang.github.io/rustup/index.html)
- [The rustdoc Book](../library/rustdoc-book/) ([source](https://github.com/rust-lang/rust))
- rustfmt (todo)
- [The `clippy` Book](https://doc.rust-lang.org/nightly/clippy/development/infrastructure/book.html)
- [The `just` Programmer's Manual](https://just.systems/man/en/)
- [The mdBook Book](../library/mdbook/) ([source](https://github.com/rust-lang/mdBook))
- [The bindgen User Guide](../library/bindgen/) ([source](https://github.com/rust-lang/rust-bindgen))
- miri (todo)

## The Rust crate ecosystem

- [The Rand Book](../library/rand-book/) ([source](https://github.com/rust-random/book))
- [The `proptest` Book](https://proptest-rs.github.io/proptest/intro.html)
- [The `serde` Book](https://serde.rs/)
- [Rust Cookbook](../library/rust-cookbook/) ([source](https://github.com/rust-lang-nursery/rust-cookbook))

## Domain-specific Rust

- [The Embedded Rust Book](../library/embedded-book/) ([source](https://github.com/rust-embedded/book))

## The Rust Project internals

- [Rust Project Goals](https://rust-lang.github.io/rust-project-goals/)
- [Guide to rustc Development](../library/rustc-dev-guide/) ([source](https://github.com/rust-lang/rustc-dev-guide))
- [Standard Library Developers Guide](../library/std-dev-guide/) ([source](https://github.com/rust-lang/std-dev-guide))
- [Rust Forge](../library/rust-forge/) ([source](https://github.com/rust-lang/rust-forge))
  - 🔖 [Alternative Rust Installation Methods](../library/rust-forge/infra/other-installation-methods.html)
- [Rust RFCs](../library/rfcs/) ([source](https://github.com/rust-lang/rfcs))



[`mdbook`]: https://github.com/rust-lang/mdBook
