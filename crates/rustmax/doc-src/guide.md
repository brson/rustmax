A Guide to the Rustmax crate.




## Getting started

Add `rustmax` to your `Cargo.toml` with a profile enabled:

```toml
[dependencies]
rmx.package = "rustmax"
rmx.version = "0.0.8"
rmx.features = [
  "rmx-profile-portable",
]
```

Or if using a workspace, define the dependency in your workspace `Cargo.toml`:

```toml
[workspace.dependencies]
rmx.package = "rustmax"
rmx.version = "0.0.8"
rmx.features = [
  "rmx-profile-portable",
]
```

Then in each crate's `Cargo.toml`:

```toml
[dependencies]
rmx.workspace = true
```

In your own code then you can access
common crates through the `rmx` path.

```rust
todo
```




## The crates of Rustmax


| Category | Crates |
| --- | --- |
| error handling and debugging | [`anyhow`], [`env_logger`], [`log`], [`thiserror`] |
| collections | [`ahash`], [`bitflags`], [`bytes`], [`itertools`] |
| numerics | [`num_bigint`] |
| encoding, serialization, parsing | [`base64`], [`comrak`], [`flate2`], [`hex`], [`json5`], [`memchr`], [`nom`], [`regex`], [`serde`], [`serde_json`], [`toml`], [`zip`] |
| time | [`chrono`], [`jiff`] |
| random numbers | [`rand`], [`rand_chacha`], [`rand_pcg`] |
| cryptography | [`blake3`], [`sha2`] |
| parallelism | [`crossbeam`], [`rayon`] |
| asynchronous I/O | [`futures`], [`tokio`] |
| networking and web | [`axum`], [`http`], [`hyper`], [`mime`], [`reqwest`], [`socket2`], [`tera`], [`tower`], [`url`] |
| text / unicode | [`unicode_segmentation`] |
| convenience macros | [`cfg-if`](cfg_if), [`derive_more`], [`extension-trait`](extension_trait), [`num_enum`], [`powerletters`] |
| terminal / CLI | [`clap`], [`ctrlc`], [`indicatif`], [`termcolor`], [`rustyline`] |
| system / OS | [`glob`], [`ignore`], [`notify`], [`tempfile`], [`walkdir`], [`xshell`] |
| graphics / images | [`image`] |
| testing | [`proptest`] |
| FFI / interop | [`libc`], [`bindgen`], [`cc`], [`cxx`], [`cxx-build`](cxx_build) |
| build scripts | ... |
| deployment and software lifecycle | [`semver`] |
| procedural macros | [`proc-macro2`](proc_macro2), [`quote`], [`syn`] |




## A Guide to the Rustmax crate

- [Cargo features and profiles](#cargo-features-and-profiles)
- [Crate reexports](#crate-reexports)
- [Standard library reexports](#standard-library-reexports)
- [The `rustmax` prelude](#the-rustmax-prelude)
- [The `extras` module](#the-extras-module)
- [Starting from a template](#starting-from-a-template)
- [Known bugs](#known-bugs)
- [Profiles](#profiles).
  `rustmax` organizes crates into _profiles_,
  which correspond to common target environments and application types.
  - [Profile: `rmx-profile-no-std`][`rmx-profile-no-std`]
  - [Profile: `rmx-profile-std`][`rmx-profile-std`]
  - [Profile: `rmx-profile-portable`][`rmx-profile-portable`]
  - [Profile: `rmx-profile-net`][`rmx-profile-net`]
  - [Profile: `rmx-profile-cli`][`rmx-profile-cli`]
  - [Profile: `rmx-profile-build-script`][`rmx-profile-build-script`]
  - [Profile: `rmx-profile-proc-macro`][`rmx-profile-proc-macro`]
  - [Profile: `rmx-profile-full`][`rmx-profile-full`]
  - [Profile: `rmx-profile-max`][`rmx-profile-max`]
  - [Profile: `rmx-profile-max-nightly`][`rmx-profile-max-nightly`]
- [Ecosystem features](#ecosystem-features).
  `rustmax` identifies Cargo features common across many crates.
  - [Feature: `rmx-feature-no-std`][`rmx-feature-no-std`]
  - [Feature: `rmx-feature-std`][`rmx-feature-std`]
  - [Feature: `rmx-feature-default`][`rmx-feature-default`]
  - [Feature: `rmx-feature-more`][`rmx-feature-more`]
  - [Feature: `rmx-feature-derive`][`rmx-feature-derive`]
  - [Feature: `rmx-feature-serde`][`rmx-feature-serde`]
  - [Feature: `rmx-feature-backtrace`][`rmx-feature-backtrace`]
  - [Feature: `rmx-feature-tokio`][`rmx-feature-tokio`]
  - [Feature: `rmx-feature-nightly`][`rmx-feature-nightly`]
- [Rust standard libraries](#rust-standard-libraries).
  `rustmax` re-exports the standard Rust libraries for convenience.
  - [Rustlib: `rmx-rustlib-core`][`rmx-rustlib-core`]
  - [Rustlib: `rmx-rustlib-alloc`][`rmx-rustlib-alloc`]
  - [Rustlib: `rmx-rustlib-std`][`rmx-rustlib-std`]
  - [Rustlib: `rmx-rustlib-proc_macro`][`rmx-rustlib-proc_macro`]
- `rustmax` crate API docs
  - [Re-exports](#reexports)
  - [Modules](#modules)




## Cargo features and profiles

`rustmax` enables no features by default and reexports no crates.
All configuration is done through Cargo features,
primarily through _profile_ features.

Profiles select sets of crates for common use cases:
[`rmx-profile-portable`] is a good default for most applications,
[`rmx-profile-std`] adds crates that require native OS features,
and [`rmx-profile-net`] adds networking and async I/O.
See [Profiles](#profiles) for the full list.

In addition to profiles,
_ecosystem features_ provide cross-cutting control
over individual crate features like `derive`, `serde`, and `tokio` support.
Profiles enable sensible defaults for these,
but they can also be toggled individually.
See [Ecosystem features](#ecosystem-features) for details.




## Crate reexports

All `rustmax` crates are reexported as modules:

```rust,ignore
# use rustmax as rmx;
use rmx::rand::Rng;
```

These modules behave the same as the corresponding crates,
with exceptions noted in [Known bugs](#known-bugs).
Each module has `rustmax`-specific documentation
with a description, example, and links to the original crate docs.

Modules are only defined when their crate is enabled
through a profile feature like [`rmx-profile-portable`].




## Standard library reexports

`rustmax` also reexports the Rust standard libraries as modules,
enabled automatically by profiles.
See [Rust standard libraries](#rust-standard-libraries).




## The `rustmax` prelude




## The `extras` module




## Starting from a template




## Known bugs

- `serde` derive only works if the `serde` crate is an explicit dependency.
- `derive_more` derives only works if the `derive_more` crate is an explicit dependency.




<br>

----

<br>




## Profiles

`rustmax` organizes crates into _profiles_,
which correspond to common target environments and application types.

By default no profile is enabled and no crates are exported.


### Profile: `rmx-profile-no-std`

This profile includes crates that do not require Rust `std`.
It allows use of the Rust allocator,
and enables allocator-related features of its crates.
All crates in this profile are also in [`rmx-profile-std`].

This profile also enables [`rmx-feature-no-std`].\
This profile also enables [`rmx-rustlib-core`] and [`rmx-rustlib-alloc`].


#### Crates in `rmx-profile-no-std`

- [`ahash`] - A fast and DOS-resistent hash function, for use in `HashMap`s.
- [`anyhow`] - Flexible error handling.
- [`base64`] - Base-64 encoding and decoding.
- [`bitflags`] - Types in which the bits are individually addressable.
- [`blake3`] - The BLAKE3 cryptographic hash function.
- [`bytes`] - Abstractions for working with byte buffers: [`Bytes`](bytes::Bytes), [`Buf`](bytes::Buf), and [`BufMut`](bytes::BufMut).
- [`cfg-if`](cfg_if) - A macro for writing conditional compilation as `if` / `else` blocks.
- [`chrono`] - Dates and time (legacy).
- [`crossbeam`] - Concurrency tools to supplement [`std::sync`], including fast channels.
- [`derive_more`] - `derive` for more standard traits.
- [`extension-trait`](extension_trait) - A macro for defining extension methods to external types.
- [`futures`] - Abstractions for asynchronous programming.
- [`hex`] - Encoding and decoding hexadecimal strings.
- [`itertools`] - Additional methods for iterators.
- [`jiff`] - Dates and time.
- [`libc`] - Bindings to the C standard library.
- [`log`] - A simple logging framework.
- [`memchr`] - Fast byte search with SIMD acceleration.
- [`nom`] - An efficient parser combinator.
- [`num_bigint`] - Arbitrary-sized integers.
- [`num_enum`] - Conversions between numbers and enums.
- [`powerletters`] - Superscript and subscript Unicode text conversion.
- [`rand`] - Random number generators.
- [`rand_chacha`] - The ChaCha cryptographically-secure random number generators.
- [`rand_pcg`] - The PCG non-cryptographically-secure random number generators.
- [`semver`] - The software versioning standard used by Rust.
- [`serde`] - The standard Rust serialization framework.
- [`serde_json`] - JSON serialization / deserialization with [`serde`].
- [`sha2`] - The SHA2 cryptographic hash functions.
- [`toml`] - TOML serialization / deserialization with `serde`.




### Profile: `rmx-profile-std`

This profile depends on the Rust standard library,
and includes crates that require the Rust standard library,
in addition to the crates provided by [`rmx-profile-no-std`].

This profile also enables [`rmx-feature-std`].\
This profile also enables [`rmx-feature-default`].\
This profile also enables [`rmx-feature-more`].\
This profile also enables [`rmx-feature-derive`].\
This profile also enables [`rmx-feature-serde`].\
This profile also enables [`rmx-rustlib-core`], [`rmx-rustlib-alloc`], and [`rmx-rustlib-std`].


#### Crates in `rmx-profile-std`

- [`clap`] - Command line parsing.
- [`comrak`] - CommonMark and GitHub Flavored Markdown parser.
- [`env_logger`] - A basic logger to use with the [`log`] crate.
- [`flate2`] - Deflate, gzip, and zlib compression and decompression.
- [`glob`] - Unix shell style pattern matching for paths.
- [`ignore`] - Directory traversal respecting gitignore rules.
- [`image`] - Image processing and manipulation.
- [`indicatif`] - Progress bars and spinners for CLI.
- [`json5`] - JSON5, a superset of JSON with expanded syntax.
- [`notify`] - Cross-platform filesystem notifications.
- [`proptest`] - Testing over generated inputs, ala QuickCheck.
- [`rayon`] - Parallel iterators and other parallel processing tools.
- [`regex`] - Regular expressions.
- [`tempfile`] - Temporary files and directories.
- [`tera`] - A text template engine based on Jinja2.
- [`thiserror`] - Tools for defining custom error types.
- [`unicode-segmentation`](unicode_segmentation) - Splitting strings on grapheme cluster, word, and sentence boundaries.
- [`walkdir`] - Efficient directory traversal.
- [`xshell`] - A Swiss-army knife for writing shell-style scripts in Rust.
- [`zip`] - Read and write ZIP archives.




### Profile: `rmx-profile-portable`

This profile is designed for portable targets including WebAssembly (WASM)
and cross-compiled environments like Linux musl.
It includes all crates from [`rmx-profile-no-std`],
plus additional crates that are compatible with these environments.

This profile uses portable variants of ecosystem features
that exclude features incompatible with WASM or requiring
a C cross-compiler toolchain, such as OS-specific threading APIs,
file system operations that require native OS support,
and C library dependencies like zstd.

This profile also enables [`rmx-rustlib-core`], [`rmx-rustlib-alloc`], and [`rmx-rustlib-std`].\
This profile also enables [`rmx-feature-std-portable`].\
This profile also enables [`rmx-feature-default-portable`].\
This profile also enables [`rmx-feature-more-portable`].\
This profile also enables [`rmx-feature-derive`].\
This profile also enables [`rmx-feature-serde`].


#### Crates in `rmx-profile-portable`

All crates from [`rmx-profile-no-std`], plus:

- [`clap`] - Command line parsing.
- [`comrak`] - CommonMark and GitHub Flavored Markdown parser.
- [`env_logger`] - A basic logger to use with the [`log`] crate.
- [`flate2`] - Deflate, gzip, and zlib compression and decompression.
- [`glob`] - Unix shell style pattern matching for paths.
- [`json5`] - JSON5, a superset of JSON with expanded syntax.
- [`rayon`] - Parallel iterators and other parallel processing tools.
- [`regex`] - Regular expressions.
- [`tempfile`] - Temporary files and directories.
- [`thiserror`] - Tools for defining custom error types.
- [`unicode-segmentation`](unicode_segmentation) - Splitting strings on grapheme cluster, word, and sentence boundaries.
- [`walkdir`] - Efficient directory traversal.
- [`zip`] - Read and write ZIP archives.

Note: Some crates from [`rmx-profile-std`] are not included
because they require native OS features or C dependencies:
[`proptest`], [`tera`], [`xshell`].




### Profile: `rmx-profile-net`

Adds networking crates,
including the [`tokio`] async runtime.

Not that this profile does not enable `tokio` features
for other crates; to enable `tokio` features
apply the [`rmx-feature-tokio`] feature.

This profile also enables [`rmx-profile-std`].


#### Crates in `rmx-profile-net`

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




### Profile: `rmx-profile-cli`

Crates for building commandline interfaces.

This profile also enables [`rmx-profile-std`].


#### Crates in `rmx-profile-cli`

- [`ctrlc`] - Simple handling of Ctrl-C for CLI programs.
- [`termcolor`] - Cross-platform library for writing colored output to the terminal.
- [`rustyline`] - Command-line input reading with history.




### Profile: `rmx-profile-build-script`

Crates for writing [Rust build scripts](todo).

This profile also enables [`rmx-profile-std`].


#### Crates in `rmx-profile-build-script`

- [`bindgen`] - Generate Rust bindings to C and C++ libraries.
- [`cc`] - A basic cross-platform C/C++ compiler driver.
- [`cxx`] - C++ bridge runtime support; paired with [`cxx_build`].
- [`cxx-build`](cxx_build) - C++ bridge generator; paired with [`cxx`].




### Profile: `rmx-profile-proc-macro`

Crates for writing [Rust procedural macros](todo).

This profile also enables [`rmx-profile-std`].\
This profile also enables [`rmx-rustlib-proc_macro`].


#### Crates in `rmx-profile-proc-macro`

- [`proc-macro2`](proc_macro2) - A preferred wrapper around the standard [`proc_macro`] crate.
- [`quote`] - The `quote!` macro for turning code blocks into source tokens.
- [`syn`] - A Rust parser used by procedural macros.




### Profile: `rmx-profile-full`

This profile simply enables all previous profiles.

This profile also enables [`rmx-profile-std`].\
This profile also enables [`rmx-profile-net`].\
This profile also enables [`rmx-profile-cli`].\
This profile also enables [`rmx-profile-build-script`].\
This profile also enables [`rmx-profile-proc-macro`].




### Profile: `rmx-profile-max`

`rustmax` with all features (that don't require nightly).

This profile also enables [`rmx-profile-full`].\
This profile also enables [`rmx-feature-derive`].\
This profile also enables [`rmx-feature-serde`].\
This profile also enables [`rmx-feature-backtrace`].\
This profile also enables [`rmx-feature-tokio`].




### Profile: `rmx-profile-max-nightly`

`rustmax` with all features (including nightly).

This profile also enables [`rmx-profile-max`].\
This profile also enables [`rmx-feature-nightly`].




## Ecosystem features

`rustmax` identifies Cargo features common across many crates.


### Feature: `rmx-feature-no-std`

This feature is enabled by [`rmx-profile-no-std`].
It does not typically need to be set manually.

It enables few features,
particularly enabling allocator support for no-std crates
that can be compiled without.


### Feature: `rmx-feature-std`

This feature is enabled by [`rmx-profile-std`].
It does not typically need to be set manually.

It enables the "std" feature of crates
and other default features that require the standard library.


### Feature: `rmx-feature-std-portable`

This feature is enabled by [`rmx-profile-portable`].
It does not typically need to be set manually.

Similar to [`rmx-feature-std`], but excludes features
that are incompatible with portable targets (WASM, musl cross-compilation),
such as those requiring threading or OS-specific APIs.


### Feature: `rmx-feature-default`

This feature is enabled by [`rmx-profile-std`].
It does not typically need to be set manually.

It enables the "default" feature of crates.


### Feature: `rmx-feature-default-portable`

This feature is enabled by [`rmx-profile-portable`].
It does not typically need to be set manually.

Similar to [`rmx-feature-default`], but uses portable
default features where necessary (e.g., excludes zstd from zip).


### Feature: `rmx-feature-more`

This feature is enabled by [`rmx-profile-std`].
It does not typically need to be set manually.

This activates extra crate features for convenience
that the crates themselves do not typically activate by default.


### Feature: `rmx-feature-more-portable`

This feature is enabled by [`rmx-profile-portable`].
It does not typically need to be set manually.

Similar to [`rmx-feature-more`], but excludes features
that are incompatible with portable targets,
such as blocking I/O and threading.


### Feature: `rmx-feature-derive`

Enables derive macros of crates where it is optional,
typically with a feature named "derive".


### Feature: `rmx-feature-serde`

Enables [`serde`] support for crates where it is optional,
typically with a feature named "serde".


### Feature: `rmx-feature-backtrace`

Enables backtrace support for crates where it is optional,
typically with a feature named "backtrace".

This feature is necessary for backtrace support in [`anyhow`].

This feature also enables `rmx-feature-std`.


### Feature: `rmx-feature-tokio`

Enables [`tokio`] support for crates where it is optional,
typically with a feature named "tokio".


### Feature: `rmx-feature-nightly`

Enables features that only compile with the Rust [nightly compiler],
typically with a feature named "nightly".




## Rust standard libraries

`rustmax` re-exports the standard Rust libraries for convenience.
These features enable reexports of the corresponding standard library crates
as modules within `rustmax`.


### Rustlib: `rmx-rustlib-core`

Reexports the [`core`] library.
Enabled by [`rmx-profile-no-std`] and all profiles that include it.


### Rustlib: `rmx-rustlib-alloc`

Reexports the [`alloc`] library.
Enabled by [`rmx-profile-no-std`] and all profiles that include it.


### Rustlib: `rmx-rustlib-std`

Reexports the [`std`] library.
Enabled by [`rmx-profile-std`] and all profiles that include it.


### Rustlib: `rmx-rustlib-proc_macro`

Reexports the [`proc_macro`] library.
Enabled by [`rmx-profile-proc-macro`].




<!-- links -->

[`rmx-profile-no-std`]: #profile-rmx-profile-no-std
[`rmx-profile-std`]: #profile-rmx-profile-std
[`rmx-profile-portable`]: #profile-rmx-profile-portable
[`rmx-profile-net`]: #profile-rmx-profile-net
[`rmx-profile-cli`]: #profile-rmx-profile-cli
[`rmx-profile-build-script`]: #profile-rmx-profile-build-script
[`rmx-profile-proc-macro`]: #profile-rmx-profile-proc-macro
[`rmx-profile-full`]: #profile-rmx-profile-full
[`rmx-profile-max`]: #profile-rmx-profile-max
[`rmx-profile-max-nightly`]: #profile-rmx-profile-max-nightly
[`rmx-feature-no-std`]: #feature-rmx-feature-no-std
[`rmx-feature-std`]: #feature-rmx-feature-std
[`rmx-feature-std-portable`]: #feature-rmx-feature-std-portable
[`rmx-feature-default`]: #feature-rmx-feature-default
[`rmx-feature-default-portable`]: #feature-rmx-feature-default-portable
[`rmx-feature-more`]: #feature-rmx-feature-more
[`rmx-feature-more-portable`]: #feature-rmx-feature-more-portable
[`rmx-feature-derive`]: #feature-rmx-feature-derive
[`rmx-feature-serde`]: #feature-rmx-feature-serde
[`rmx-feature-backtrace`]: #feature-rmx-feature-backtrace
[`rmx-feature-tokio`]: #feature-rmx-feature-tokio
[`rmx-feature-nightly`]: #feature-rmx-feature-nightly
[`rmx-rustlib-core`]: #rustlib-rmx-rustlib-core
[`rmx-rustlib-alloc`]: #rustlib-rmx-rustlib-alloc
[`rmx-rustlib-std`]: #rustlib-rmx-rustlib-std
[`rmx-rustlib-proc_macro`]: #rustlib-rmx-rustlib-proc_macro
