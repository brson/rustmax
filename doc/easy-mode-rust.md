# Easy Mode Rust

## Starting a project

## Starting a project with `cargo-generate`

I start a new project with `cargo-generate`:

```
cargo generate https://github.com/brson/rustx/master/template
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
