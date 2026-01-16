<!-- GENERATED FILE DO NOT EDIT -->

# The Rustmax Library

The Rust language and its ecosystem is documented in "books"
(rendered with [`mdbook`]), and most of these links are to books.

Links with a bookmark icon, 🔖, are to particularly
notable or useful chapters within a book.

## The Rust language

{{book:trpl:bold}}
- **[Rust By Example](https://doc.rust-lang.org/rust-by-example/)**
{{book:reference:bold}}
{{#if-book:reference}}
  - 🔖 [Conditional compilation](../library/reference/conditional-compilation.html).
       Including which cfgs are set by rustc.
  - 🔖 [Behavior considered undefined](../library/reference/behavior-considered-undefined.html)
{{/if-book}}
{{book:nomicon:bold}}
{{book:edition-guide}}
- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/)
{{book:api-guidelines}}
{{book:unsafe-code-guidelines}}
- [Rust Error Codes Index](https://doc.rust-lang.org/stable/error_codes/error-index.html)
- [The Rust Unstable Book](https://doc.rust-lang.org/unstable-book/)
- [The Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/index.html)
- [Rust Release Notes](https://doc.rust-lang.org/nightly/releases.html)

## The Rust standard library

- **[`std`](../api/std/index.html)**
  <!-- duplicated in std.md -->
  - 🔖 [`std::collections`](../api/std/collections/index.html)
  - 🔖 [`std::error`](../api/std/error/index.html)
  - 🔖 [`std::ptr`](../api/core/ptr/index.html).
    Safety, undefined behavior, and "provenance".
  - 🔖 [`std::sync`](../api/std/sync/index.html)
  - 🔖 [`std::sync::atomic`](../api/core/sync/atomic/index.html)
- [`core`](../api/core/index.html)
- [`alloc`](../api/alloc/index.html)
- [`proc_macro`](../api/proc_macro/index.html)


## Standard Rust tools

<!-- order here is same is in tools.md -->
{{book:cargo-book}}
{{#if-book:cargo-book}}
  - 🔖 [The manifest format](../library/cargo-book/reference/manifest.html)
  - 🔖 [Environment variables](../library/cargo-book/reference/environment-variables.html)
    that affect the Cargo build process.
  - 🔖 [Configuration format](../library/cargo-book/reference/config.html).
    Cargo has many interesting configuration options.
  - 🔖 [SemVer compatibility](../library/cargo-book/reference/semver.html).
    Guidelines for maintaining semver compatibility.
{{/if-book}}
{{book:rustc-book}}
{{#if-book:rustc-book}}
  - 🔖 [The lint system and built-in lints](../library/rustc-book/lints/index.html)
  - 🔖 [Rust platform support tiers](../library/rustc-book/platform-support.html)
{{/if-book}}
- [The `rustup` Book](https://rust-lang.github.io/rustup/index.html)
{{book:rustdoc-book}}
- rustfmt (todo)
- [The `clippy` Book](https://doc.rust-lang.org/nightly/clippy/development/infrastructure/book.html)
- [The `just` Programmer's Manual](https://just.systems/man/en/)
{{book:mdbook}}
{{book:bindgen}}
- miri (todo)

## The Rust crate ecosystem

{{book:rand-book}}
- [The `proptest` Book](https://proptest-rs.github.io/proptest/intro.html)
- [The `serde` Book](https://serde.rs/)
{{book:rust-cookbook}}

## Domain-specific Rust

{{book:embedded-book}}

## The Rust Project internals

- [Rust Project Goals](https://rust-lang.github.io/rust-project-goals/)
{{book:rustc-dev-guide}}
{{book:std-dev-guide}}
{{book:rust-forge}}
{{#if-book:rust-forge}}
  - 🔖 [Alternative Rust Installation Methods](../library/rust-forge/infra/other-installation-methods.html)
{{/if-book}}
{{book:rfcs}}



[`mdbook`]: https://github.com/rust-lang/mdBook
