A collection of useful Rust crates.


**🚧
WARNING:
Do not use this project.
It is neither stable nor supported.
🚧**


This crate documents and reexports selected high-quality Rust crates
suitable for many Rust programs.


- [Profiles](#profiles)
  - [🌎 Profile: `rx-profile-no-std`][`rx-profile-no-std`]
  - **[🌎 Profile: `rx-profile-std`][`rx-profile-std`]**
  - [🌎 Profile: `rx-profile-net`][`rx-profile-net`]
  - [🌎 Profile: `rx-profile-cli`][`rx-profile-cli`]
  - [🌎 Profile: `rx-profile-build-script`][`rx-profile-build-script`]
  - [🌎 Profile: `rx-profile-proc-macro`][`rx-profile-proc-macro`]
  - **[🌎 Profile: `rx-profile-full`][`rx-profile-full`]**
  - [🌎 Profile: `rx-profile-max`][`rx-profile-max`]
- - [🌎 Profile: `rx-profile-max-nightly`][`rx-profile-max-nightly`]
- [Ecosystem features](#ecosystem-features)
  - [⛲ Feature: `rx-feature-no-std`][`rx-feature-no-std`]
  - [⛲ Feature: `rx-feature-std`][`rx-feature-std`]
  - [⛲ Feature: `rx-feature-default`][`rx-feature-default`]
  - **[⛲ Feature: `rx-feature-derive`][`rx-feature-derive`]**
  - **[⛲ Feature: `rx-feature-serde`][`rx-feature-serde`]**
  - [⛲ Feature: `rx-feature-backtrace`][`rx-feature-backtrace`]
  - **[⛲ Feature: `rx-feature-tokio`][`rx-feature-tokio`]**
  - [⛲ Feature: `rx-feature-nightly`][`rx-feature-nightly`]
- [Crate Features](#crate-features)
  - [⛲ Feature: `rx-rand-x-small_rng`][`rx-rand-x-small_rng`]
  - [⛲ Feature: `rx-serde-x-rc`][`rx-serde-x-rc`]
- [Rust system libraries](#rust-system-libraries)
  - [📙 Rustlib: `rx-rustlibs-no-std`][`rx-rustlibs-no-std`]
  - [📙 Rustlib: `rx-rustlibs-alloc`][`rx-rustlibs-alloc`]
  - [📙 Rustlib: `rx-rustlibs-std`][`rx-rustlibs-std`]
  - [📙 Rustlib: `rx-rustlibs-proc-macro`][`rx-rustlibs-proc-macro`]
- [Using `rustx` as a library](#using-rustx-as-a-library)
  - [`rustx` and cargo features](#rustx-and-cargo-features)
  - [Crate reexports](#crate-reexports)
  - [Standard library reexports](#standard-library-reexports)
  - [The `rustx` prelude](#the-rustx-prelude)
  - [The `extra` module](#the-extra-module)
  - [Starter examples](#starter-examples)
  - [Starting from a template](#starting-from-a-template)
  - [Known bugs](#known-bugs)




# Profiles

`rustx` organizes crates into _profiles_,
which correspond to common target environments and application types.

By default no profile is enabled and no crates are exported.


## 🌎 Profile: `rx-profile-no-std`

This profile includes crates that do not require Rust `std`.
It allows use of the Rust allocator,
and enables allocator-related features of its crates.
All crates in this profile are also in [`rx-feature-std`].

💡 This profile also enables [`rx-feature-no-std`].\
💡 This profile also enables [`rx-rustlibs-no-std`].\


### Crates in `rx-profile-no-std`

- [`ahash`] - A fast and DOS-resistent hash function, for use in `HashMap`s.
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
- [`futures`]
- [`hex`]
- [`itertools`]
- [`jiff`]
- [`libc`]
- [`log`]
- [`nom`]
- [`num_bigint`]
- [`num_enum`]
- [`once_cell`]
- [`proptest`]
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

- [`clap`]
- [`env_logger`]
- [`json5`]
- [`lazy_static`]
- [`num_cpus`]
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
- [`ctrlc`]
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
💡 This profile also enables [`rx-rustlibs-proc-macro`].\


### Crates in `rx-profile-proc-macro`

- [`proc-macro2`](proc_macro2)
- [`quote`]
- [`syn`]




## 🌎 Profile: `rx-profile-full`

This profile simply enables all previous profiles.

💡 This profile also enables [`rx-profile-std`].\
💡 This profile also enables [`rx-profile-net`].\
💡 This profile also enables [`rx-profile-cli`].\
💡 This profile also enables [`rx-profile-build-script`].\
💡 This profile also enables [`rx-profile-proc-macro`].\




## 🌎 Profile: `rx-profile-max`

`rustx` with all features (that don't require nightly).

💡 This profile also enables [`rx-profile-full`].\
💡 This profile also enables [`rx-feature-derive`].\
💡 This profile also enables [`rx-feature-serde`].\
💡 This profile also enables [`rx-feature-backtrace`].\
💡 This profile also enables [`rx-feature-tokio`].\
💡 This profile also enables [`rx-rand-x-small_rng`].\
💡 This profile also enables [`rx-serde-x-rng`].\




## 🌎 Profile: `rx-profile-max-nightly`

`rustx` with all features (including nightly).

💡 This profile also enables [`rx-profile-max`].\
💡 This profile also enables [`rx-feature-nightly`].\




# Ecosystem features

todo


## ⛲ Feature: `rx-feature-no-std`

This feature is enabled by [`rx-profile-no-std`].
It does not typically need to be set manually.

It enables few features,
particularly enabling allocator support for no-std crates
that can be compiled without.


## ⛲ Feature: `rx-feature-std`

This feature is enabled by [`rx-profile-std`].
It does not typically need to be set manually.

It enables the "std" feature of crates
and other default features that require the standard library.


## ⛲ Feature: `rx-feature-default`

This feature is enabled by [`rx-profile-std`].
It does not typically need to be set manually.

It enables the "default" feature of crates.


## ⛲ Feature: `rx-feature-derive`

Enables derive macros of crates where it is optional,
typically with a feature named "derive".


## ⛲ Feature: `rx-feature-serde`

Enables [`serde`] support for crates where it is optional,
typically with a feature named "serde".


## ⛲ Feature: `rx-feature-backtrace`

Enables backtrace support for crates where it is optional,
typically with a feature named "backtrace".

This feature is necessary for backtrace support in [`anyhow`].

This feature also enables `rx-feature-std`.


## ⛲ Feature: `rx-feature-tokio`

Enables [`tokio`] support for crates where it is optional,
typically with a feature named "tokio".


## ⛲Feature: `rx-feature-nightly`

Enables features that only compile with the Rust [nightly compiler],
typically with a feature named "nightly".




# Crate features

todo


## ⛲ Feature: `rx-rand-x-small_rng`

todo


## ⛲ Feature: `rx-serde-x-rng`

todo




# Rust system libraries

todo


## 📙 Rustlib: `rx-rustlib-no-std`


## 📙 Rustlib: `rx-rustlib-alloc`


## 📙 Rustlib: `rx-rustlib-std`


## 📙 Rustlib: `rx-rustlib-proc-macro`




# Using `rustx` as a library.

The `rustx` crate name is `rstx`,
but it is usually renamed to the shorter `rx` in your Cargo manifest,
since the crate name will be typed often
(if `rx` is too awkward to type then the crate can be renamed `rs`).

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




## `rustx` and cargo features

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




## Crate reexports




## Standard library reexports




## The `rustx` prelude




## The `extra` module




## Known bugs

- serde derive only works if the serde crate is an explicit dependency.




<!-- links -->

[`rx-profile-no-std`]: #-profile-rx-profile-no-std
[`rx-profile-std`]: #-profile-rx-profile-std
[`rx-profile-net`]: #-profile-rx-profile-net
[`rx-profile-cli`]: #-profile-rx-profile-cli
[`rx-profile-build-script`]: #-profile-rx-profile-build-script
[`rx-profile-proc-macro`]: #-profile-rx-profile-proc-macro
[`rx-profile-full`]: #-profile-rx-profile-full
[`rx-feature-no-std`]: #-feature-rx-feature-no-std
[`rx-feature-std`]: #-feature-rx-feature-std
[`rx-feature-default`]: #-feature-rx-feature-default
[`rx-feature-derive`]: #-feature-rx-feature-derive
[`rx-feature-serde`]: #-feature-rx-feature-serde
[`rx-feature-backtrace`]: #-feature-rx-feature-backtrace
[`rx-feature-tokio`]: #-feature-rx-feature-tokio
[`rx-feature-nightly`]: #-feature-rx-feature-nightly
[`rx-rustlibs-no-std`]: #-rustlibs-rx-rustlibs-no-std
[`rx-rustlibs-alloc`]: #-rustlibs-rx-rustlibs-alloc
[`rx-rustlibs-std`]: #-rustlibs-rx-rustlibs-std
[`rx-rustlibs-proc-macro`]: #-rustlibs-rx-rustlibs-proc-macro
