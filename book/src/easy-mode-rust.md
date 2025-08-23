# Easy Mode Rust



## Install Rust with `rustup`

Rust is installed with the [`rustup`](https://rust-lang.github.io/rustup) tool.

On Linux, Mac OS, and Unixes,
run the following in your shell then follow the onscreen instructions:

```
curl -sSf https://sh.rustup.rs | sh
```

For Windows and other install see [https://rustup.rs](https://rustup.rs).




## Rust workspace considerations




## Start a new `rustmax` project with `cargo-generate`

This will create a cargo workspace with two crates, one a library, one a CLI,
with a dependency on the `rustmax` crate.

```
$ cargo generate brson/rustmax
⚠️   Favorite `brson/rustmax` not found in config, using it as a git repository: https://github.com/brson/rustmax.git
🤷   Project Name: datalang
🔧   Destination: /home/brian/.homes/dev/megaspace/datalang ...
🔧   project-name: datalang ...
🔧   Generating template ...
[ 1/11]   Done: .gitignore
[ 2/11]   Done: Cargo.toml.liquid
[ 3/11]   Done: crates/datalang/Cargo.toml.liquid
[ 4/11]   Done: crates/datalang/src/lib.rs
[ 5/11]   Done: crates/datalang/src
[ 6/11]   Done: crates/datalang
[ 7/11]   Done: crates/datalang-cli/Cargo.toml.liquid
[ 8/11]   Done: crates/datalang-cli/src/main.rs
[ 9/11]   Done: crates/datalang-cli/src
[10/11]   Done: crates/datalang-cli
[11/11]   Done: crates
     Moving generated files into: `/home/brian/.homes/dev/megaspace/datalang`...
🔧   Initializing a fresh Git repository
✨   Done! New project created /home/brian/.homes/dev/megaspace/datalang
```







## Updating dependencies in lockfile

```
cargo update
```






## Updating dependencies in Cargo.toml

```
cargo upgrade
```

With no extra arguments `cargo upgrade` modifies
`Cargo.toml` files such that the dependencies are
set to their latest compatible versions.
In this way it similar to `cargo update` but for manifests
instead of lockfiles.

todo This command is from the `cargo-edit` package.






## Upgrading dependencies across minor versions in Cargo.toml


```
cargo upgrade --incompatible
```






## topics

- anyhow, thiserror
