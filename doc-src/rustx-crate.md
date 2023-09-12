A collection of useful Rust crates.


**⚠️
WARNING:
Do not use this project.
⚠️**


This crate documents and reexports selected high-quality Rust crates
suitable for many Rust programs.


- [Profiles](#profiles)
  - [Profile: `rx-profile-no-std`](#profile-rx-profile-no-std)
  - [Profile: `rx-profile-std`](#profile-rx-profile-std)
  - [Profile: `rx-profile-net`](#profile-rx-profile-net)
  - [Profile: `rx-profile-cli`](#profile-rx-profile-cli)
  - [Profile: `rx-profile-build-script`](#profile-rx-profile-build-script)
  - [Profile: `rx-profile-proc-macro`](#profile-rx-proc-macro)
  - [Profile: `rx-profile-full`](#profile-rx-profile-full)
- [Using `rustx` as a library](#using-rustx-as-a-library)
- [Crate reexports](#crate-reexports)
- [Standard library reexports](#standard-library-reepxorts)
- [The `rustx` prelude](#the-rustx-prelude)
- [The `extra` module](#the-extra-module)
- [`rustx` and cargo features](#rustx-and-cargo-features)
- [Ecosystem features](#ecosystem-features)
  - [Feature: `rx-feature-no-std`](#feature-rx-feature-no-std)
  - [Feature: `rx-feature-std`](#feature-rx-feature-std)
  - [Feature: `rx-feature-default`](#feature-rx-feature-default)
  - [Feature: `rx-feature-derive`](#feature-rx-feature-derive)
  - [Feature: `rx-feature-serde`](#feature-rx-feature-serde)
  - [Feature: `rx-feature-backtrece`](#feature-rx-feature-backtrace)
  - [Feature: `rx-feature-tokio`](#feature-rx-feature-tokio)
  - [Feature: `rx-feature-nightly`](#feature-rx-feature-nightly)
- [Crate features](#crate-features)




## Profiles

todo

By default no profile is enabled and no crates are exported.




## 🌎 Profile: `rx-profile-no-std`

This profile includes crates that do not require Rust `std`.
It allows use of the Rust allocator,
and enables allocator-related features of its crates.

💡 This profile also enables [`rx-feature-no-std`].\
💡 This profile also enables [`rx-rustlibs-no-std`].\


### Crates in `rx-profile-no-std`

- [`ahash`]
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




## 🌎 Profile: `rx-profile-std`

This profile depends on the Rust standard library,
and includes crates that require the Rust standard library,
in addition to the crates provided by [`rx-profile-no-std`].

💡 This profile also enables [`rx-feature-std`].\
💡 This profile also enables [`rx-feature-default`].\
💡 This profile also enables [`rx-rustlibs-std`].\


### Crates in `rx-profile-std`

- [`big_s`]
- [`clap`]
- [`env_logger`]
- [`json5`]
- [`lazy_static`]
- [`num_cpus`]
- [`og_fmt`]
- [`rayon`]
- [`regex`]
- [`tempfile`]
- [`tera`]
- [`thiserror`]
- [`toml`]
- [`unicode-segmentation`](unicode_segmentation)
- [`walkdir`]
- [`xshell`]




## 🌎 Profile: `rx-profile-net`

Adds networking crates,
including the [`tokio`] async runtime.

Not that this profile does not enable `tokio` features
for other crates; to enable `tokio` features
apply the [`rx-feature-tokio`] feature.

💡 This profile also enables [`rx-profile-std`].\


### Crates in `rx-profile-net`

- [`http`]
- [`hyper`]
- [`mime`]
- [`reqwest`]
- [`socket2`]
- [`tokio`]
- [`url`]




## 🌎 Profile: `rx-profile-cli`

Crates for building commandline interfaces.

💡 This profile also enables [`rx-profile-std`].\


### Crates in `rx-profile-cli`

- [`console`]
- [`dialoguer`]
- [`indicatif`]
- [`termcolor`]
- [`rustyline`]




## 🌎 Profile: `rx-profile-build-script`

Crates for writing [Rust build scripts](todo).

💡 This profile also enables [`rx-profile-std`].\


### Crates in `rx-profile-build-script`

- [`cc`]
- [`cxx`]
- [`cxx-build`](cxx_build)




## 🌎 Profile: `rx-profile-proc-macro`

Crates for writing [Rust procedural macros](todo).

💡 This profile also enables [`rx-profile-std`].\


### Crates in `rx-profile-proc-macro`

- [`proc-macro2`](proc_macro2)
- [`quote`]
- [`syn`]




## 🌎 Profile: `rx-profile-full`

This profile simply enables all other profiles.




# Using `rustx` as a library.

The `rustx` crate name is `rstx`,
but it is usually renamed `rx` in your Cargo manifest,
since the crate name will be typed often.

In your manifest `Cargo.toml`:

```toml
[dependencies]
rx.package = "rstx"
rx.version = "0.1.0"
rx.features = [
  "rx-profile-std",
]
```

Or if using a workspace, in your workspace `Cargo.toml`

```toml
[dependencies]
rx.package = "rstx"
rx.version = "0.1.0"
rx.features = [
  "rx-profile-std",
]
```

And in your manifest `Cargo.toml`

```toml
[dependencies]
rx.workspace = true
```




# `rustx` and cargo features

todo

The main way of configuring the `rustx` crates is by enabling
the appropriate _profile_ cargo features.

`rustx` enables no features by default,
and reexports no crates;
but for most uses people will want to enable [`rx-profile-std`].
This feature augments the Rust `std` library with crates
that are widely used with a variety of Rust programs,
as well as minor helpers missing from the standard library.

```toml
[dependencies]
rx.package = "rstx"
rx.version = "0.1.0"
rx.features = ["rx-profile-std"]
```




# Crate reexports




# Standard library reexports




# The `rustx` prelude




## The `extra` module




## `rustx` and Cargo features




# Exosystem features

`rustx` organizes its crates features

todo


## Feature: `rx-feature-no-std`

This feature is enabled by [`rx-profile-no-std`].
It does not typically need to be set manually.

It enables few features,
particularly enabling allocator support for no-std crates
that can be compiled without.


## Feature: `rx-feature-std`

This feature is enabled by [`rx-profile-std`].
It does not typically need to be set manually.

It enables the "std" feature of crates
and other default features that require the standard library.


## Feature: `rx-feature-default`

This feature is enabled by [`rx-profile-std`].
It does not typically need to be set manually.

It enables the "default" feature of crates.


## Feature: `rx-feature-derive`

Enables derive macros of crates where it is optional,
typically with a feature named "derive".


## Feature: `rx-feature-serde`

Enables [`serde`] support for crates where it is optional,
typically with a feature named "serde".


## Feature: `rx-feature-backtrace`

Enables backtrace support for crates where it is optional,
typically with a feature named "backtrace".

This feature is necessary for backtrace support in [`anyhow`].

This feature also enables `rx-feature-std`.


## Feature: `rx-feature-tokio`

Enables [`tokio`] support for crates where it is optional,
typically with a feature named "tokio".


## Feature: `rx-feature-nightly`

Enables features that only compile with the Rust [nightly compiler],
typically with a feature named "nightly".


