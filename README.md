# Rust Extra

A collection of useful Rust crates and tools.


## The `rustx` crate

The `rustx` crate documents and reexports selected high-quality Rust crates
suitable for typical Rust programs.
It can be thought of as a "batteries-included" standard library.

[Documentation](https://docs.rs/rustx).


## The `rustx` tool

Install with

    cargo install rustx-cli


## More documentation

The `rustx-doc` tool produces TODO


## Do not contribute

**This project is not open to contribution at this time.
Pull requests adding crates are not welcome and will be closed.**


## License

`rustx` is licensed

    Apache-2.0 OR Apache-2.0 with LLVM-exception OR MIT OR CC0-1.0

All libraries included as dependencies of the `rustx` crate
are permissively licensed under Apache-2.0 or weaker (e.g. MIT).

Tools installed by the `rustx` command may have other licenses.


## TODO

- xtask
- more terminal crates
- fs2 - unmaintained?
- cli
  - list tools
  - install tools
  - show tool status
  - show rustup proxies status
  - where tool
  - build docs
  - search docs
  - initialize dep-frozen rustx project
- workspace template
  - overflow on
- documentation
  - crate docs
  - tool docs
  - tool --help pages
  - rustup components
  - toolchain descriptions
  - dev tool tutorial
  - easy-mode rust
  - official docs
- documentation generator
- documentation search
- tracing
- tungstenite / tungstenite-tokio?
- charting / graphing?
- fuzzing / profiling / flamegraphs
- notify
- time
- wasm profile
- wasm-bindgen
- build-script profile improvements
- unix/linux/macos/windows profiles?
- compression
- tests
  - cargo-license / cargo-deny
  - cargo-audit
- datatest_stable
- criterion
- include_dir
- scripting languages
  - RustPython
- rustyline / termion?
- semver
- home directory
- grpc / protoc
- mold setup
