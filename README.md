# Rust Max

A collection of crates, tools, and documentation for the Rust programming language.


## 🚧 Do not contribute, do not use 🚧

This project is not open to contribution.
Issues and pull requests will be closed without consideration.
Do not use this project.
It is neither stable nor supported.


## The crates

The [`rmx` crate](https://docs.rs/rmx)
documents and reexports other useful Rust crates.


## The tools

- [A collection of useful Rust tools](book/src/tools.md).
- [An opinionated workspace template](template),
  for [cargo-generate](https://github.com/cargo-generate/cargo-generate).
- [An opinionated config file](rustfmt.toml)
  for [rustfmt](https://github.com/rust-lang/rustfmt).
- [An opinionated config file](configs/deny.toml)
  for [cargo-deny](https://github.com/EmbarkStudios/cargo-deny).
- [An opinionated config file](clippy-control.toml)
  for [clippy-control](https://github.com/brson/clippy-control).
- [The `rmx` CLI for managing the above.](https://docs.rs/rmx-cli).


## The documentation

This project is documented as

- [Markdown](book/src/SUMMARY.md),
- [HTML](todo),
- and [PDF](todo).


## License

Rust Max is licensed

    CC0-1.0 OR MIT OR Apache-2.0 WITH LLVM-exception OR Apache-2.0

All libraries included as dependencies of the `rmx` crate
are permissively licensed under BSD-3-Clause, MIT, or weaker.

Tools installed by the `rmx` command may have other licenses.
