# The Rustmax CLI

To install:

```
cargo install rustmax-cli --locked
```

The Rustmax CLI provides a number of small
tools that help manage a Rust development environment,
Rust tools, and Rust projects.

- Print Rust sytem status
- Install the `mold` linker
- Install all Rustmax tools
- Create a Rustmax project from template
- Build the Rustmax documentation
- Run all lint-style checks
- Emit opinionated `rustfmt.toml`
- Emit opinionated `deny.toml`
- Emit opinionated `clippy-control.toml`

## Print Rust system status

Rustmax understands your rustup toolchain status,
Rustmax tools installed via cargo,
and other tools like the `mold` linker.

```
$ rustmax status
... todo ...
```
