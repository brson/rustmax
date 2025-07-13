# The Rust Max CLI

To install:

```
cargo install rustmax-cli --locked
```

The Rust Max CLI provides a number of small
tools that help manage a Rust development environment,
Rust tools, and Rust projects.

- Print Rust sytem status
- Install the `mold` linker
- Install all Rust Max tools
- Create a Rust Max project from template
- Build the Rust Max documentation
- Emit opinionated `rustfmt.toml`
- Emit opinionated `deny.toml`
- Emit opinionated `clippy-control.toml`

## Print Rust system status

Rust Max understands your rustup toolchain status,
Rust Max tools installed via cargo,
and other tools like the `mold` linker.

```
$ rustmax status
... todo ...
```
