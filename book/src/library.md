<!-- GENERATED FILE DO NOT EDIT -->

# The Rust Max Library

The Rust language and its ecosystem is documented in "books"
(rendered with [`mdbook`]), and most of these links are to books.

Links with a bookmark icon, 🔖, are to particularly
notable or useful chapters within a book.

## The Rust language

- **[The Rust Programming Language](../library/trpl/)** ([upstream](https://doc.rust-lang.org/book/))
- **[Rust By Example](../library/rust-by-example/)** ([upstream](https://doc.rust-lang.org/rust-by-example/))
- **[The Rust Reference](../library/reference/)** ([upstream](https://doc.rust-lang.org/reference/))
  - 🔖 [Conditional compilation](../library/reference/conditional-compilation.html).
       Including which cfgs are set by rustc.
  - 🔖 [Behavior considered undefined](../library/reference/behavior-considered-undefined.html)
- **[The Rustonomicon](../library/nomicon/)** ([upstream](https://doc.rust-lang.org/nomicon/))
- [The Rust Edition Guide](../library/edition-guide/) ([upstream](https://doc.rust-lang.org/edition-guide/))
- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/)
- [Rust API Guidelines](../library/api-guidelines/) ([upstream](https://rust-lang.github.io/api-guidelines/))
- [Rust Unsafe Code Guidelines](../library/unsafe-code-guidelines/) ([upstream](https://rust-lang.github.io/unsafe-code-guidelines/))
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
- [The Cargo Book](../library/cargo-book/) ([upstream](https://doc.rust-lang.org/cargo/))
  - 🔖 [The manifest format](../library/cargo-book/reference/manifest.html)
  - 🔖 [Environment variables](../library/cargo-book/reference/environment-variables.html)
    that affect the Cargo build process.
  - 🔖 [Configuration format](../library/cargo-book/reference/config.html).
    Cargo has many interesting configuration options.
- [The rustc Book](../library/rustc-book/) ([upstream](https://doc.rust-lang.org/rustc/))
  - 🔖 [The lint system and built-in lints](../library/rustc-book/lints/index.html)
  - 🔖 [Rust platform support tiers](../library/rustc-book/platform-support.html)
- [The `rustup` Book](https://rust-lang.github.io/rustup/index.html)
- [The rustdoc Book](../library/rustdoc-book/) ([upstream](https://doc.rust-lang.org/rustdoc/))
- rustfmt (todo)
- [The `clippy` Book](https://doc.rust-lang.org/nightly/clippy/development/infrastructure/book.html)
- [The `just` Programmer's Manual](https://just.systems/man/en/)
- [The mdBook Book](../library/mdbook/) ([upstream](https://rust-lang.github.io/mdBook/))
- [The bindgen User Guide](../library/bindgen/) ([upstream](https://rust-lang.github.io/rust-bindgen/))
- miri (todo)

## The Rust crate ecosystem

- [The Rand Book](../library/rand-book/) ([upstream](https://rust-random.github.io/book/))
- [The `proptest` Book](https://proptest-rs.github.io/proptest/intro.html)
- [The `serde` Book](https://serde.rs/)
- [Rust Cookbook](../library/rust-cookbook/) ([upstream](https://rust-lang-nursery.github.io/rust-cookbook/))

## Domain-specific Rust

- [The Embedded Rust Book](../library/embedded-book/) ([upstream](https://doc.rust-lang.org/stable/embedded-book/))

## The Rust Project internals

- [Rust Project Goals](https://rust-lang.github.io/rust-project-goals/)
- [Guide to rustc Development](../library/rustc-dev-guide/) ([upstream](https://rustc-dev-guide.rust-lang.org/))
- [Standard Library Developers Guide](../library/std-dev-guide/) ([upstream](https://std-dev-guide.rust-lang.org/))
- [Rust Forge](../library/rust-forge/) ([upstream](https://forge.rust-lang.org/))
  - 🔖 [Alternative Rust Installation Methods](../library/rust-forge/infra/other-installation-methods.html)
- [Rust RFCs](../library/rfcs/) ([upstream](https://rust-lang.github.io/rfcs/))



[`mdbook`]: https://github.com/rust-lang/mdBook
