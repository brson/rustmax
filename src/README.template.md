# Rust Max

A collection of crates, tools, and documentation for the Rust programming language.


## 🚧 Do not contribute, do not use 🚧

This project is not open to contribution.
Issues and pull requests will be closed without consideration.
Do not use this project.
It is neither stable nor supported.

---

*Rust Max is not an official Rust project.*

---

{{cratelist}}

---

## The crates

The [`rustmax` crate](https://docs.rs/rustmax)
documents and reexports other useful Rust crates.


## The tools

- [A collection of useful Rust tools](book/src/tools.md).
- [An opinionated workspace template](template),
  for [cargo-generate](https://github.com/cargo-generate/cargo-generate).
- [An opinionated config file](rustfmt.toml)
  for [rustfmt](https://github.com/rust-lang/rustfmt).
- [An opinionated config file](deny.toml)
  for [cargo-deny](https://github.com/EmbarkStudios/cargo-deny).
- [An opinionated config file](clippy-control.toml)
  for [clippy-control](https://github.com/brson/clippy-control).
- [The `rustmax` CLI for managing the above.](https://docs.rs/rustmax-cli).


## License

Rust Max is licensed

    CC0-1.0 OR MIT OR Apache-2.0 WITH LLVM-exception OR Apache-2.0

All libraries included as dependencies of the `rustmax` crate
are permissively licensed under BSD-3-Clause, MIT, or weaker.

Tools installed by the `rustmax` command may have other licenses.
