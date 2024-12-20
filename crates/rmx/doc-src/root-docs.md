**🚧
WARNING:
Do not use this project.
It is neither stable nor supported.
🚧**


This crate documents and reexports selected high-quality Rust crates
suitable for many Rust programs.
Through the organization of its [Cargo features]
into _profiles_ and _ecosystem features_
this crate is a guide to the Rust crate ecosystem.
It can be read as reference documentation or
imported through a Cargo dependency
as a "batteries included" extendend Rust standard library.

<br>

----

<br>

| Category | Crates |
| --- | --- |
| error handling and debugging | [`anyhow`], [`backtrace`], [`env_logger`], [`log`], [`thiserror`] |
| collections | [`ahash`], [`bitflags`], [`bytes`], [`itertools`], [`lazy_static`] |
| numerics | [`num_bigint`] |
| encoding, serialization, parsing | [`base64`], [`byteorder`], [`hex`], [`json5`], [`nom`], [`regex`], [`serde`], [`serde_json`], [`tera`], [`toml`] |
| time | [`chrono`], [`jiff`] |
| random numbers | [`rand`], [`rand_chacha`], [`rand_pcg`] |
| cryptography | [`blake3`] |
| parallelism | [`crossbeam`], [`once_cell`], [`rayon`] |
| asyncronous I/O | [`futures`], [`tokio`] |
| networking and web | [`http`], [`hyper`], [`mime`], [`reqwest`], [`socket2`], [`url`] |
|  text / unicode | [`unicode_segmentation`] |
| convenience macros | [`cfg-if`](cfg_if), [`derive_more`], [`extension-trait`](extension_trait), [`num_enum`] |
| terminal / CLI | [`clap`], [`console`], [`ctrlc`], [`dialoguer`], [`indicatif`], [`termcolor`], [`rustyline`] |
| system / OS | [`num_cpus`], [`tempfile`], [`walkdir`], [`xshell`] |
| testing | [`proptest`], [`static_assertions`] |
| foreign / native interop | [`libc`], [`cc`], [`cxx`], [`cxx-build`](cxx_build) |
| procedural macros | [`proc-macro2`](proc_macro2), [`quote`], [`syn`] |

<br>

----

<br>

- [Profiles](#profiles).
  `rmx` organizes crates into _profiles_,
  which correspond to common target environments and application types.
  - [🌎 Profile: `rmx-profile-no-std`][`rmx-profile-no-std`]
  - **[🌎 Profile: `rmx-profile-std`][`rmx-profile-std`]**
  - [🌎 Profile: `rmx-profile-net`][`rmx-profile-net`]
  - [🌎 Profile: `rmx-profile-cli`][`rmx-profile-cli`]
  - [🌎 Profile: `rmx-profile-build-script`][`rmx-profile-build-script`]
  - [🌎 Profile: `rmx-profile-proc-macro`][`rmx-profile-proc-macro`]
  - **[🌎 Profile: `rmx-profile-full`][`rmx-profile-full`]**
  - **[🌎 Profile: `rmx-profile-max`][`rmx-profile-max`]**
  - [🌎 Profile: `rmx-profile-max-nightly`][`rmx-profile-max-nightly`]
- [Ecosystem features](#ecosystem-features).
  `rmx` identifies Cargo features common across many crates.
  - [⛲ Feature: `rmx-feature-no-std`][`rmx-feature-no-std`]
  - [⛲ Feature: `rmx-feature-std`][`rmx-feature-std`]
  - [⛲ Feature: `rmx-feature-default`][`rmx-feature-default`]
  - **[⛲ Feature: `rmx-feature-derive`][`rmx-feature-derive`]**
  - **[⛲ Feature: `rmx-feature-serde`][`rmx-feature-serde`]**
  - [⛲ Feature: `rmx-feature-backtrace`][`rmx-feature-backtrace`]
  - **[⛲ Feature: `rmx-feature-tokio`][`rmx-feature-tokio`]**
  - [⛲ Feature: `rmx-feature-nightly`][`rmx-feature-nightly`]
- [Crate features](#crate-features).
  Some Crate-specific features are re-exported by `rmx`.
  - [⛲ Feature: `rmx-rand-x-small_rng`][`rmx-rand-x-small_rng`]
  - [⛲ Feature: `rmx-serde-x-rc`][`rmx-serde-x-rc`]
- [Rust standard libraries](#rust-standard-libraries).
  `rmx` re-exports the standard Rust libraries for convenience.
  - [📙 Rustlib: `rmx-rustlibs-no-std`][`rmx-rustlibs-no-std`]
  - [📙 Rustlib: `rmx-rustlibs-alloc`][`rmx-rustlibs-alloc`]
  - [📙 Rustlib: `rmx-rustlibs-std`][`rmx-rustlibs-std`]
  - [📙 Rustlib: `rmx-rustlibs-proc-macro`][`rmx-rustlibs-proc-macro`]
- [Using `rmx` as a library](#using-rmx-as-a-library)
  - [`rmx` and cargo features](#rmx-and-cargo-features)
  - [Crate reexports](#crate-reexports)
  - [Standard library reexports](#standard-library-reexports)
  - [The `rmx` prelude](#the-rmx-prelude)
  - [The `extra` module](#the-extra-module)
  - [Starter examples](#starter-examples)
  - [Starting from a template](#starting-from-a-template)
  - [Known bugs](#known-bugs)
- `rmx` crate API docs
  - [Re-exports](#reexports)
  - [Modules](#modules)

<br>

----
----

<br>




# Profiles

`rmx` organizes crates into _profiles_,
which correspond to common target environments and application types.

By default no profile is enabled and no crates are exported.


## 🌎 Profile: `rmx-profile-no-std`

This profile includes crates that do not require Rust `std`.
It allows use of the Rust allocator,
and enables allocator-related features of its crates.
All crates in this profile are also in [`rmx-profile-std`].

💡 This profile also enables [`rmx-feature-no-std`].\
💡 This profile also enables [`rmx-rustlibs-no-std`].


### Crates in `rmx-profile-no-std`

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




## 🌎 Profile: `rmx-profile-std`

This profile depends on the Rust standard library,
and includes crates that require the Rust standard library,
in addition to the crates provided by [`rmx-profile-no-std`].

💡 This profile also enables [`rmx-feature-std`].\
💡 This profile also enables [`rmx-feature-default`].\
💡 This profile also enables [`rmx-rustlibs-std`].


### Crates in `rmx-profile-std`

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
- [`unicode-segmentation`](unicode_segmentation)
- [`walkdir`]
- [`xshell`]




## 🌎 Profile: `rmx-profile-net`

Adds networking crates,
including the [`tokio`] async runtime.

Not that this profile does not enable `tokio` features
for other crates; to enable `tokio` features
apply the [`rmx-feature-tokio`] feature.

💡 This profile also enables [`rmx-profile-std`].


### Crates in `rmx-profile-net`

- [`http`]
- [`hyper`]
- [`mime`]
- [`reqwest`]
- [`socket2`]
- [`tokio`]
- [`url`]




## 🌎 Profile: `rmx-profile-cli`

Crates for building commandline interfaces.

💡 This profile also enables [`rmx-profile-std`].


### Crates in `rmx-profile-cli`

- [`console`]
- [`ctrlc`]
- [`dialoguer`]
- [`indicatif`]
- [`termcolor`]
- [`rustyline`]




## 🌎 Profile: `rmx-profile-build-script`

Crates for writing [Rust build scripts](todo).

💡 This profile also enables [`rmx-profile-std`].


### Crates in `rmx-profile-build-script`

- [`cc`]
- [`cxx`]
- [`cxx-build`](cxx_build)




## 🌎 Profile: `rmx-profile-proc-macro`

Crates for writing [Rust procedural macros](todo).

💡 This profile also enables [`rmx-profile-std`].\
💡 This profile also enables [`rmx-rustlibs-proc-macro`].


### Crates in `rmx-profile-proc-macro`

- [`proc-macro2`](proc_macro2)
- [`quote`]
- [`syn`]




## 🌎 Profile: `rmx-profile-full`

This profile simply enables all previous profiles.

💡 This profile also enables [`rmx-profile-std`].\
💡 This profile also enables [`rmx-profile-net`].\
💡 This profile also enables [`rmx-profile-cli`].\
💡 This profile also enables [`rmx-profile-build-script`].\
💡 This profile also enables [`rmx-profile-proc-macro`].




## 🌎 Profile: `rmx-profile-max`

`rmx` with all features (that don't require nightly).

💡 This profile also enables [`rmx-profile-full`].\
💡 This profile also enables [`rmx-feature-derive`].\
💡 This profile also enables [`rmx-feature-serde`].\
💡 This profile also enables [`rmx-feature-backtrace`].\
💡 This profile also enables [`rmx-feature-tokio`].\
💡 This profile also enables [`rmx-rand-x-small_rng`].\
💡 This profile also enables [`rmx-serde-x-rc`].




## 🌎 Profile: `rmx-profile-max-nightly`

`rmx` with all features (including nightly).

💡 This profile also enables [`rmx-profile-max`].\
💡 This profile also enables [`rmx-feature-nightly`].




# Ecosystem features

`rmx` identifies Cargo features common across many crates.


## ⛲ Feature: `rmx-feature-no-std`

This feature is enabled by [`rmx-profile-no-std`].
It does not typically need to be set manually.

It enables few features,
particularly enabling allocator support for no-std crates
that can be compiled without.


## ⛲ Feature: `rmx-feature-std`

This feature is enabled by [`rmx-profile-std`].
It does not typically need to be set manually.

It enables the "std" feature of crates
and other default features that require the standard library.


## ⛲ Feature: `rmx-feature-default`

This feature is enabled by [`rmx-profile-std`].
It does not typically need to be set manually.

It enables the "default" feature of crates.


## ⛲ Feature: `rmx-feature-derive`

Enables derive macros of crates where it is optional,
typically with a feature named "derive".


## ⛲ Feature: `rmx-feature-serde`

Enables [`serde`] support for crates where it is optional,
typically with a feature named "serde".


## ⛲ Feature: `rmx-feature-backtrace`

Enables backtrace support for crates where it is optional,
typically with a feature named "backtrace".

This feature is necessary for backtrace support in [`anyhow`].

This feature also enables `rmx-feature-std`.


## ⛲ Feature: `rmx-feature-tokio`

Enables [`tokio`] support for crates where it is optional,
typically with a feature named "tokio".


## ⛲Feature: `rmx-feature-nightly`

Enables features that only compile with the Rust [nightly compiler],
typically with a feature named "nightly".




# Crate features

Some Crate-specific features are re-exported by `rmx`.


## ⛲ Feature: `rmx-rand-x-small_rng`

todo


## ⛲ Feature: `rmx-serde-x-rc`

todo




# Rust standard libraries

`rmx` re-exports the standard Rust libraries for convenience.


## 📙 Rustlib: `rmx-rustlib-no-std`


## 📙 Rustlib: `rmx-rustlib-alloc`


## 📙 Rustlib: `rmx-rustlib-std`


## 📙 Rustlib: `rmx-rustlib-proc-macro`




# Using `rmx` as a library.

In your manifest `Cargo.toml`:

```toml
[dependencies]
rmx.version = "0.2.0"
rmx.features = [
  "rmx-profile-std",
]
```

Or if using a workspace, in your workspace `Cargo.toml`

```toml
[dependencies]
rmx.version = "0.2.0"
rmx.features = [
  "rmx-profile-std",
]
```

And in your crate's `Cargo.toml`

```toml
[dependencies]
rmx.workspace = true
```




## `rmx` and cargo features

todo

The main way of configuring the `rmx` crates is by enabling
the appropriate _profile_ cargo features.

`rmx` enables no features by default,
and reexports no crates;
but for most uses people will want to enable [`rmx-profile-std`].
This feature augments the Rust `std` library with crates
that are widely used with a variety of Rust programs,
as well as minor helpers missing from the standard library.

```toml
[dependencies]
rmx.version = "0.2.0"
rmx.features = [
  "rmx-profile-std",
]
```




## Crate reexports




## Standard library reexports




## The `rmx` prelude




## The `extra` module




## Known bugs

- serde derive only works if the serde crate is an explicit dependency.




<!-- links -->

[`rmx-profile-no-std`]: #-profile-rmx-profile-no-std
[`rmx-profile-std`]: #-profile-rmx-profile-std
[`rmx-profile-net`]: #-profile-rmx-profile-net
[`rmx-profile-cli`]: #-profile-rmx-profile-cli
[`rmx-profile-build-script`]: #-profile-rmx-profile-build-script
[`rmx-profile-proc-macro`]: #-profile-rmx-profile-proc-macro
[`rmx-profile-full`]: #-profile-rmx-profile-full
[`rmx-profile-max`]: #-profile-rmx-profile-max
[`rmx-profile-max-nightly`]: #-profile-rmx-profile-max-nightly
[`rmx-feature-no-std`]: #-feature-rmx-feature-no-std
[`rmx-feature-std`]: #-feature-rmx-feature-std
[`rmx-feature-default`]: #-feature-rmx-feature-default
[`rmx-feature-derive`]: #-feature-rmx-feature-derive
[`rmx-feature-serde`]: #-feature-rmx-feature-serde
[`rmx-feature-backtrace`]: #-feature-rmx-feature-backtrace
[`rmx-feature-tokio`]: #-feature-rmx-feature-tokio
[`rmx-feature-nightly`]: #-feature-rmx-feature-nightly
[`rmx-rand-x-small_rng`]: #-feature-rmx-rand-x-small_rng
[`rmx-serde-x-rc`]: #-feature-rmx-serde-x-rc
[`rmx-rustlibs-no-std`]: #-rustlibs-rmx-rustlibs-no-std
[`rmx-rustlibs-alloc`]: #-rustlibs-rmx-rustlibs-alloc
[`rmx-rustlibs-std`]: #-rustlibs-rmx-rustlibs-std
[`rmx-rustlibs-proc-macro`]: #-rustlibs-rmx-rustlibs-proc-macro

[Cargo features]: todo
