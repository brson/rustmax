A collection of useful Rust crates.


**⚠️
WARNING:
Do not use this project.
⚠️**


This crate documents and reexports selected high-quality Rust crates
suitable for typical Rust programs.


- [Profiles](#profiles)
  - [Profile: `rx-profile-no-std`](#profile-rx-profile-no-std)
  - [Profile: `rx-profile-std`](#profile-rx-profile-std)
  - [Profile: `rx-profile-net`](#profile-rx-profile-net)
  - [Profile: `rx-profile-cli`](#profile-rx-profile-cli)
  - [Profile: `rx-profile-build-script`](#profile-rx-profile-build-script)
  - [Profile: `rx-profile-proc-macro`](#profile-rx-proc-macro)
  - [Profile: `rx-profile-full`](#profile-rx-profile-full)
- [Using `rustx` as a library](#using-rustx-as-a-library)
- [The `rustx` prelude(#the-rustx-prelude)
- [The `extra` module](#the-extra-module)
- [`rustx` and cargo features](#rustx-and-cargo-features)
- [Features](#feature-selection)
  - [Crate features](#crate-features)
  - [Feature: `rx-feature-no-std`](#feature-rx-feature-no-std)




## Profiles

todo

By default no profile is enabled and no crates are exported.




## Profile: `rx-profile-no-std`

This profile includes crates that do not require Rust `std`,
and provide features used by many Rust programs.
It allows use of the Rust allocator,
and enables allocator-related features of its crates.


### Crates in `rx-profile-no-std`

- [`anyhow`]
- [`backtrace`]
- [`base64`]
- [`bitflags`]
- [`blake3`]
- [`byteorder`]
- [`bytes`]
- [`cfg-if`](cfg_if)
- [`chrono`]
- [`crossbeam`]
- [`derive_more`]
- [`extension-trait`](extension_trait)
- [`fnv`]
- [`futures`]
- [`hex`]
- [`itertools`]
- [`libc`]
- [`log`]
- [`nom`]
- [`num_enum`]
- [`once_cell`]
- [`rand`]
- [`rand_chacha`]
- [`rand_pcg`]
- [`serde`]
- [`serde_json`]
- [`static_assertions`]
- [`toml`]


### Features enabled by `rx-profile-no-std`

- [`rx-feature-no-std`](#feature-rx-feature-no-std)




## Profile: `rx-profile-std`

This profile depends on the Rust standard library,
and includes crates that require the Rust standard library.
and provide features used by many Rust programs.
It automatically activates Cargo features of each crate
to enable standard library features, usually named "std".


### Crates in `rx-profile-std`

- [`big_s`]
- [`clap`],
- [`env_logger`],
- [`json5`],
- [`lazy_static`],
- [`num_cpus`],
- [`og_fmt`],
- [`rayon`],
- [`regex`],
- [`tempfile`],
- [`tera`],
- [`thiserror`],
- [`toml`],
- [`unicode-segmentation`](unicode_segmentation),
- [`walkdir`],
- [`xshell`],


### Features enabled by `rx-profile-std`

- [`rx-feature-std`](#feature-rx-feature-std),
- [`rx-feature-default`](#feature-rx-feature-default),



## Profile: `rx-profile-net`


## Profile: `rx-profile-cli`


## Profile: `rx-profile-build-script`


## Profile: `rx-profile-proc-macro`


## Profile: `rx-profile-full`




# Using `rustx` as a library.

In your manifest `Cargo.toml`:

```toml
[dependencies]
rustx = "0.1.0"
```

Or if using a workspace, in your workspace `Cargo.toml`

```toml
[workspace.dependencies]
rustx = "0.1.0"
```

And in your manifest `Cargo.toml`

```toml
[dependencies]
rustx.workspace = true
```


# `rustx` and cargo features

todo

The main way of configuring the `rustx` crates is by enabling
the appropriate _profile_ cargo features.

The default profile feature is `rx-profile-std`.
This feature augments the Rust `std` library with crates
that are widely used with a variety of Rust programs,
as well as minor helpers missing from the standard library.

If the default features are disabled:

```toml
[dependencies]
rustx.version = "0.1.0"
rustx.default-features = false
```

Then `rustx` reexports no crates.
Profiles can then be added by adding cargo features:

```toml
[dependencies]
rustx.version = "0.1.0"
rustx.default-features = false
rustx.features = ["rx-profile-no-std"]
```


# Features

`rustx` organizes its crates features

todo


## Crate features

For every included crate `rustx` exposes a feature with the same name.

Example: todo.


## Feature: `rx-feature-no-std`

This feature is enabled by `rx-profile-no-std`.
It does not typically need to be set manually.

todo



# TODO

- document rustlibs
- update big_s and og_fmt to be no_std
- update fmt to `use format as fmt`
