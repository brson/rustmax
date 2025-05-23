A collection of Rust crates.

**🚧
WARNING:
Do not use this project.
It is neither stable nor supported.
🚧**


This crate documents and reexports selected high-quality Rust crates
suitable for many Rust programs.
Through the organization of its [Cargo features]
into _profiles_ and _ecosystem features_,
with consistent descriptions of individual crates,
it is a guide to the Rust crate ecosystem.
It can be read as reference documentation or
imported through a Cargo dependency
as a "batteries included" supercrate.


<br>

----

<br>


| Category | Crates |
| --- | --- |
| error handling and debugging | [`anyhow`], [`backtrace`], [`env_logger`], [`log`], [`thiserror`] |
| collections | [`ahash`], [`bitflags`], [`bytes`], [`itertools`] |
| numerics | [`num_bigint`] |
| encoding, serialization, parsing | [`base64`], [`byteorder`], [`hex`], [`json5`], [`nom`], [`regex`], [`serde`], [`serde_json`], [`toml`] |
| time | [`chrono`], [`jiff`] |
| random numbers | [`rand`], [`rand_chacha`], [`rand_pcg`] |
| cryptography | [`blake3`], [`sha2`] |
| parallelism | [`crossbeam`], [`rayon`] |
| asyncronous I/O | [`futures`], [`tokio`] |
| networking and web | [`axum`], [`http`], [`hyper`], [`mime`], [`reqwest`], [`socket2`], [`tera`], [`tower`], [`url`] |
| text / unicode | [`unicode_segmentation`] |
| convenience macros | [`cfg-if`](cfg_if), [`derive_more`], [`extension-trait`](extension_trait), [`num_enum`] |
| terminal / CLI | [`clap`], [`ctrlc`], [`termcolor`], [`rustyline`] |
| system / OS | [`num_cpus`], [`tempfile`], [`walkdir`], [`xshell`] |
| testing | [`proptest`], [`static_assertions`] |
| FFI / interop | [`libc`], [`bindgen`], [`cc`], [`cxx`], [`cxx-build`](cxx_build) |
| build scripts | … |
| deployment and software lifecycle | [`semver`] |
| procedural macros | [`proc-macro2`](proc_macro2), [`quote`], [`syn`] |


<br>

----

<br>


- [Profiles](#profiles).
  `rustmax` organizes crates into _profiles_,
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
  `rustmax` identifies Cargo features common across many crates.
  - [⛲ Feature: `rmx-feature-no-std`][`rmx-feature-no-std`]
  - [⛲ Feature: `rmx-feature-std`][`rmx-feature-std`]
  - [⛲ Feature: `rmx-feature-default`][`rmx-feature-default`]
  - [⛲ Feature: `rmx-feature-more`][`rmx-feature-more`]
  - **[⛲ Feature: `rmx-feature-derive`][`rmx-feature-derive`]**
  - **[⛲ Feature: `rmx-feature-serde`][`rmx-feature-serde`]**
  - [⛲ Feature: `rmx-feature-backtrace`][`rmx-feature-backtrace`]
  - **[⛲ Feature: `rmx-feature-tokio`][`rmx-feature-tokio`]**
  - [⛲ Feature: `rmx-feature-nightly`][`rmx-feature-nightly`]
- [Rust standard libraries](#rust-standard-libraries).
  `rustmax` re-exports the standard Rust libraries for convenience.
  - [📙 Rustlib: `rmx-rustlibs-no-std`][`rmx-rustlibs-no-std`]
  - [📙 Rustlib: `rmx-rustlibs-alloc`][`rmx-rustlibs-alloc`]
  - [📙 Rustlib: `rmx-rustlibs-std`][`rmx-rustlibs-std`]
  - [📙 Rustlib: `rmx-rustlibs-proc-macro`][`rmx-rustlibs-proc-macro`]
- [Using `rustmax` as a library](#using-rustmax-as-a-library)
  - [`rustmax` and cargo features](#rustmax-and-cargo-features)
  - [Crate reexports](#crate-reexports)
  - [Standard library reexports](#standard-library-reexports)
  - [The `rustmax` prelude](#the-rustmax-prelude)
  - [The `extras` module](#the-extras-module)
  - [Starter examples](#starter-examples)
  - [Starting from a template](#starting-from-a-template)
  - [Known bugs](#known-bugs)
- `rustmax` crate API docs
  - [Re-exports](#reexports)
  - [Modules](#modules)


<br>

----

<br>




# Profiles

`rustmax` organizes crates into _profiles_,
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
- [`anyhow`] - Easy error handling.
- [`backtrace`] - Callstack backtraces on demand.
- [`base64`] - Base-64 encoding and decoding.
- [`bitflags`] - Types in which the bits are individually addressable.
- [`blake3`] - The BLAKE3 cryptographic hash function.
- [`byteorder`] - Big-endian and little-endian encoding.
- [`bytes`] - Abstractions for working with byte buffers: [`Bytes`](bytes::Bytes), [`Buf`](bytes::Buf), and [`BufMut`](bytes::BufMut).
- [`cfg-if`](cfg_if) - A macro for writing conditional compilation as `if` / `else` blocks.
- [`chrono`] - Dates and time (legacy).
- [`crossbeam`] - Concurrency tools to supplement [`std::sync`], including fast channels.
- [`derive_more`] - `derive` for more standard traits.
- [`extension-trait`](extension_trait) - A macro for defining extension methods to external types.
- [`futures`] - Abstractions for asynchronous programming.
- [`hex`] - Encoding and decoding hexidecimal strings.
- [`itertools`] - Additional methods for iterators.
- [`jiff`] - Dates and time.
- [`libc`] - Bindings to the C standard library.
- [`log`] - A simple logging framework.
- [`nom`] - An efficient parser combinator.
- [`num_bigint`] - Arbitrary-sized integers.
- [`num_enum`] - Conversions between numbers and enums.
- [`proptest`] - Testing over generated inputs, ala QuickCheck.
- [`rand`] - Random number generators.
- [`rand_chacha`] - The ChaCha cryptographically-secure random number generators.
- [`rand_pcg`] - The PCG non-cryptographically-secure random number generators.
- [`semver`] - The software versioning standard used by Rust.
- [`serde`] - The standard Rust serialization framework.
- [`serde_json`] - JSON serialization / deserialization with [`serde`].
- [`sha2`] - The SHA2 cryptographic hash functions.
- [`static_assertions`] - Compile-time assertions about constants, types, etc.
- [`toml`] - TOML serialization / deserialization with `serde`.




## 🌎 Profile: `rmx-profile-std`

This profile depends on the Rust standard library,
and includes crates that require the Rust standard library,
in addition to the crates provided by [`rmx-profile-no-std`].

💡 This profile also enables [`rmx-feature-std`].\
💡 This profile also enables [`rmx-feature-default`].\
💡 This profile also enables [`rmx-feature-more`].\
💡 This profile also enables [`rmx-rustlibs-std`].


### Crates in `rmx-profile-std`

- [`clap`] - Command line parsing.
- [`env_logger`] - A basic logger to use with the [`log`] crate.
- [`json5`] - JSON5, a superset of JSON with expanded syntax.
- [`num_cpus`] - Get the number of CPUS on a machine.
- [`rayon`] - Parallel iterators and other parallel processing tools.
- [`regex`] - Regular expressions.
- [`tempfile`] - Temporary files and directories.
- [`tera`] - A text template engine based on Jinja2.
- [`thiserror`] - Tools for defining custom error types.
- [`unicode-segmentation`](unicode_segmentation) - Splitting strings on grapheme cluster, word, and sentence boundaries.
- [`walkdir`] - Efficient directory traversal.
- [`xshell`] - A Swiss-army knife for writing shell-style scripts in Rust.




## 🌎 Profile: `rmx-profile-net`

Adds networking crates,
including the [`tokio`] async runtime.

Not that this profile does not enable `tokio` features
for other crates; to enable `tokio` features
apply the [`rmx-feature-tokio`] feature.

💡 This profile also enables [`rmx-profile-std`].


### Crates in `rmx-profile-net`

- [`axum`] - Web application framework based on [`tokio`].
- [`http`] - Shared definitions related to the HTTP protocol.
- [`hyper`] - HTTP, versions 1 and 2.
- [`mime`] - MIME media types.
- [`reqwest`] - Simple HTTP requests, synchronous and asynchronous.
- [`socket2`] - Low-level network socket programming beyond [`std::net`].
- [`tokio`] - An async task runtime and I/O library.
- [`tower`] - Service request/response abstraction (HTTP middleware)
              for [`tokio`] and [`axum`].
- [`url`] - URL parsing and data structures.




## 🌎 Profile: `rmx-profile-cli`

Crates for building commandline interfaces.

💡 This profile also enables [`rmx-profile-std`].


### Crates in `rmx-profile-cli`

- [`ctrlc`] - Simple handling of Ctrl-C for CLI programs.
- [`termcolor`] - Cross-platform library for writing colored output to the terminal.
- [`rustyline`] - Command-line input reading with history.




## 🌎 Profile: `rmx-profile-build-script`

Crates for writing [Rust build scripts](todo).

💡 This profile also enables [`rmx-profile-std`].


### Crates in `rmx-profile-build-script`

- [`bindgen`] - Generate Rust bindings to C and C++ libraries.
- [`cc`] - A basic cross-platform C/C++ compiler driver.
- [`cxx`] - C++ bridge runtime support; paired with [`cxx_build`].
- [`cxx-build`](cxx_build) - C++ bridge generator; paired with [`cxx`].




## 🌎 Profile: `rmx-profile-proc-macro`

Crates for writing [Rust procedural macros](todo).

💡 This profile also enables [`rmx-profile-std`].\
💡 This profile also enables [`rmx-rustlibs-proc-macro`].


### Crates in `rmx-profile-proc-macro`

- [`proc-macro2`](proc_macro2) - A preferred wrapper around the standard [`proc_macro`] crate.
- [`quote`] - The `quote!` macro for turning code blocks into source tokens.
- [`syn`] - A Rust parser used by procedural macros.




## 🌎 Profile: `rmx-profile-full`

This profile simply enables all previous profiles.

💡 This profile also enables [`rmx-profile-std`].\
💡 This profile also enables [`rmx-profile-net`].\
💡 This profile also enables [`rmx-profile-cli`].\
💡 This profile also enables [`rmx-profile-build-script`].\
💡 This profile also enables [`rmx-profile-proc-macro`].




## 🌎 Profile: `rmx-profile-max`

`rustmax` with all features (that don't require nightly).

💡 This profile also enables [`rmx-profile-full`].\
💡 This profile also enables [`rmx-feature-derive`].\
💡 This profile also enables [`rmx-feature-serde`].\
💡 This profile also enables [`rmx-feature-backtrace`].\
💡 This profile also enables [`rmx-feature-tokio`].




## 🌎 Profile: `rmx-profile-max-nightly`

`rustmax` with all features (including nightly).

💡 This profile also enables [`rmx-profile-max`].\
💡 This profile also enables [`rmx-feature-nightly`].




# Ecosystem features

`rustmax` identifies Cargo features common across many crates.


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


## ⛲ Feature: `rmx-feature-more`

This feature is enabled by [`rmx-profile-std`].
It does not typically need to be set manually.

This activates extra crate features for convenience
that the crates themselves do not typically activate by default.


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




# Rust standard libraries

`rustmax` re-exports the standard Rust libraries for convenience.


## 📙 Rustlib: `rmx-rustlibs-no-std`


## 📙 Rustlib: `rmx-rustlibs-alloc`


## 📙 Rustlib: `rmx-rustlibs-std`


## 📙 Rustlib: `rmx-rustlibs-proc-macro`




# Using `rustmax` as a library.

In your manifest `Cargo.toml`:

```toml
[dependencies]
rmx.package = "rustmax"
rmx.version = "0.0.4"
rmx.features = [
  "rmx-profile-max",
]
```

Or if using a workspace, in your workspace `Cargo.toml`

```toml
[dependencies]
rmx.version = "0.0.4"
rmx.features = [
  "rmx-profile-max",
]
```

And in your crate's `Cargo.toml`

```toml
[dependencies]
rmx.workspace = true
```




## `rustmax` and cargo features

todo

The main way of configuring the `rustmax` crates is by enabling
the appropriate _profile_ cargo features.

`rustmax` enables no features by default,
and reexports no crates;
but for most uses people will want to enable [`rmx-profile-std`].
This feature augments the Rust `std` library with crates
that are widely used with a variety of Rust programs,
as well as minor helpers missing from the standard library.

```toml
[dependencies]
rmx.package = "rustmax"
rmx.version = "0.0.4"
rmx.features = [
  "rmx-profile-max",
]
```




## Crate reexports

The crates of `rustmax` are all reexported as modules from the `rustmax` crate.

```rust,ignore
# use rustmax as rmx;
use rmx::rand::Rng;
```

These modules should generally behave the same as the corresponding crates,
with exceptions noted in ["Known Bugs"](#known-bugs).
These modules have have `rustmax`-specific module-level documentation,
with a short description and motivation, an example,
and relevant links, including to the original crate-level documentation.

These modules are only defined when their crate is configured
through cargo features like `rmx-profile-std`.




## The `rustmax` prelude




## The `extras` module




## Known bugs

- `serde` derive only works if the `serde` crate is an explicit dependency.
- `derive_more` derives only works if the `derive_more` crate is an explicit dependency.




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
[`rmx-feature-more`]: #-feature-rmx-feature-more
[`rmx-feature-derive`]: #-feature-rmx-feature-derive
[`rmx-feature-serde`]: #-feature-rmx-feature-serde
[`rmx-feature-backtrace`]: #-feature-rmx-feature-backtrace
[`rmx-feature-tokio`]: #-feature-rmx-feature-tokio
[`rmx-feature-nightly`]: #-feature-rmx-feature-nightly
[`rmx-rustlibs-no-std`]: #-rustlibs-rmx-rustlibs-no-std
[`rmx-rustlibs-alloc`]: #-rustlibs-rmx-rustlibs-alloc
[`rmx-rustlibs-std`]: #-rustlibs-rmx-rustlibs-std
[`rmx-rustlibs-proc-macro`]: #-rustlibs-rmx-rustlibs-proc-macro

[Cargo features]: todo
