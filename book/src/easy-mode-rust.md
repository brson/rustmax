# Easy Mode Rust



## Install Rust with `rustup`

Rust is installed with the [`rustup`](https://rust-lang.github.io/rustup) tool.

On Linux, Mac OS, and Unixes,
run the following in your shell then follow the onscreen instructions:

```
curl -sSf https://sh.rustup.rs | sh
```

For Windows and other install see [https://rustup.rs].




## Rust workspace considerations




## Start a new `rustmax` project with `cargo-generate`

```
todo
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
