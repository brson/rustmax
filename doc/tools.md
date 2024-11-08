# Rust Extras: Tools

- Key Rust Tools:
  [`rustc`](todo)
  [`cargo`](todo)
  [`rustup`](todo)
  [`rustfmt`](todo)
  [`clippy`](todo)
- Rust Development Tools:
  [`cargo-edit`](todo)
  [`cargo-deny`](todo)
  [`cargo-generate`](todo)
  [`clippy-control`](todo)
  [`miri`](todo)
  [`cargo-clean-all`](todo)
  [`cargo-license`](todo)
  [`tokei`](todo)
- Other Rust tools:
  [`ripgrep`](todo)
  [`jaq`](todo)

## Key Rust Tools

- rustc
- cargo
- rustup
- rustfmt
- clippy


## Rust Development Tools

### 📦 `cargo-edit`

Extra `cargo` subcommands for editing `Cargo.toml`.

```
cargo install cargo-edit
```

> 🥡 [`crates.io` Page](https://crates.io/crates/cargo-edit)\
> 👁️  [Source Repository](https://github.com/killercup/cargo-edit)

---

Installing `cargo-edit` provides two `cargo` subcommands:

- [`cargo upgrade`](https://github.com/killercup/cargo-edit#cargo-upgrade)
- [`cargo set-version`](https://github.com/killercup/cargo-edit#cargo-set-version)

[`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html)
was once provided by `cargo-edit` but since Rust [`1.62.0`](https://blog.rust-lang.org/2022/06/30/Rust-1.62.0.html)
is built into `cargo` itself.

### todo

- [cargo-deny](https://crates.io/crates/cargo-deny) - disallow specific crates, licenses, etc
- [cargo-generate](https://crates.io/crates/cargo-generate) - create cargo projecs from temtplates
- [clippy-control](https://crates.io/crates/clippy-control) - todo
- miri
- [cargo-clean-all](https://crates.io/crates/cargo-clean-all) - clean all projects, leaving recent files
- [cargo-license](https://crates.io/crates/cargo-license) - show licenses of dependencies
- [basic-http-server](https://crates.io/crates/basic-http-server) - serve static HTML and rendered Markdown
- [cargo-audit](https://crates.io/crates/cargo-audit) - find known security vulnerabilities
- [cargo-expand](https://github.com/dtolnay/cargo-expand) - expand macros
- [cargo-hack](https://crates.io/crates/cargo-hack) - build / test with all combinations of cargo features
- [cargo-tree](https://crates.io/crates/cargo-tree) - show the crate graph as a tree


## Other Tools

- [`basic-http-server`](https://crates.io/crates/basic-http-server) - serve static HTML and rendered Markdown
- [du-dust](https://crates.io/crates/du-dust) - lik du, disk usage
- [fd-find](https://crates.io/crates/fd-find) - convenient `find` replacement
- [gist](https://crates.io/crates/gist) - post gists
- [ripgrep](https://crates.io/crates/ripgrep) - fast and convenient code grep
- [tokei](https://crates.io/crates/tokei) - count lines of code
- just
- [`jaq`](https://crates.io/crates/jaq) - jq clone for querying JSON
- sd
- jsonxf
- jaq
