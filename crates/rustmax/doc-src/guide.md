A Guide to the Rustmax crate.

- [Should you use `rustmax` as a crate?](#should-you-use-rustmax-as-a-crate)
- [Getting started](#getting-started)
- [The crates of Rustmax](#the-crates-of-rustmax)
- [How `rustmax` is organized](#how-rustmax-is-organized)
- [Profiles](#profiles)
- [Ecosystem features](#ecosystem-features)
- [Rust standard libraries](#rust-standard-libraries)
- [Macros](#macros)




## Should you use `rustmax` as a crate?

Definitely not.

Firstly, it's completely unstable and unsupported and always has breaking changes.

I dogfood the `rustmax` crate in many projects,
and I have found the benefits of using it directly minimal,
though I do prefer using it to starting from scratch every project.
The primary benefit is simply having the crate names for common solutions directly available
without re-searching crates.io.
I also _usually_ keep its crates upgraded and resolve their security advisories.
I do find it convenient to start a new project by initializing
the rustmax template with the portable profile and going from there.
All my crates are in one place, and I know they will at least build on WASM,
a decent proxy for "portable".

But there are significant downsides still:

1. Build and link time. They aren't unusual for a large Rust project,
   but you do pay to build a lot of code you definitely aren't using.
   Even after the initial build you continue to pay the linkage cost.
   The obvious solution, the experimental [`mostly-unused`] hint
   that is applied to the rustmax crate (on nightly), so far
   hasn't produced significant speedups.
2. Re-exported proc macros need help. Most are handled by writing
   `#[rmx::derive(..)]` instead of `#[derive(..)]`, but `thiserror` and
   `cxx` still require a direct dependency. See [Macros](#macros).
3. Disk usage. Lots of deps to download and upgrade and build and rebuild
   even for tiny projects. I run `cargo clean` and `cargo clean-all` a lot.
   I also find `cargo sweep` useful.

[`mostly-unused`]: https://doc.rust-lang.org/beta/unstable-book/compiler-flags/hint-mostly-unused.html




## Getting started

The quickest start is `rustmax new-project`,
which generates a workspace from the `rustmax` template
using [`cargo-generate`](https://crates.io/crates/cargo-generate),
with [`rmx-profile-portable`] enabled.

To add `rustmax` to an existing project,
add it to your `Cargo.toml` with a profile enabled:

```toml
[dependencies]
rmx.package = "rustmax"
rmx.version = "0.0.10"
rmx.features = [
  "rmx-profile-portable",
]
```

Or if using a workspace, define the dependency in your workspace `Cargo.toml`:

```toml
[workspace.dependencies]
rmx.package = "rustmax"
rmx.version = "0.0.10"
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
# use rustmax as rmx;
use rmx::prelude::*;

let config: rmx::serde_json::Value =
    rmx::serde_json::from_str(r#"{ "port": 8080 }"#)?;
let port = config["port"].as_u64().ok_or(A!("expected a port"))?;

assert_eq!(port, 8080);
# Ok::<(), AnyError>(())
```




## The crates of Rustmax

| Category | Crates |
| --- | --- |
| errors and logging | [`anyhow`], [`env_logger`], [`log`], [`thiserror`] |
| collections | [`ahash`], [`bitflags`], [`bytes`], [`itertools`] |
| numerics and random numbers | [`libm`], [`num_bigint`], [`rand`], [`rand_chacha`], [`rand_pcg`] |
| serialization | [`json5`], [`serde`], [`serde_json`], [`toml`] |
| encoding and compression | [`base64`], [`flate2`], [`hex`], [`tar`], [`zip`] |
| parsing and text | [`comrak`], [`memchr`], [`nom`], [`regex`], [`semver`], [`tera`], [`unicode-segmentation`](unicode_segmentation) |
| time | [`chrono`], [`jiff`] |
| cryptography | [`blake3`], [`sha2`] |
| concurrency and async | [`crossbeam`], [`futures`], [`rayon`], [`tokio`] |
| networking and web | [`http`], [`hyper`], [`mime`], [`reqwest`], [`socket2`], [`tower`], [`url`] |
| convenience macros | [`cfg-if`](cfg_if), [`derive_more`], [`extension-trait`](extension_trait), [`num_enum`], [`powerletters`] |
| terminal / CLI | [`clap`], [`ctrlc`], [`indicatif`], [`rustyline`], [`termcolor`] |
| system / OS | [`glob`], [`ignore`], [`notify`], [`tempfile`], [`walkdir`], [`xshell`] |
| graphics / images | [`image`] |
| testing | [`proptest`] |
| FFI and build scripts | [`bindgen`], [`cc`], [`cxx`], [`cxx-build`](cxx_build), [`libc`] |
| procedural macros | [`proc-macro2`](proc_macro2), [`quote`], [`syn`] |




## How `rustmax` is organized

`rustmax` enables no features by default and reexports no crates.
All configuration is done through Cargo features,
primarily through _profile_ features.

[Profiles](#profiles) select sets of crates for common use cases:
[`rmx-profile-portable`] is a good default for most applications,
[`rmx-profile-std`] adds crates that require native OS features,
and [`rmx-profile-net`] adds networking and async I/O.

[Ecosystem features](#ecosystem-features) provide cross-cutting control
over individual crate features like `derive` and `serde` support.
Profiles enable sensible defaults for these,
but they can also be toggled individually.

Every enabled crate is reexported as a module of the same name:

```rust,ignore
# use rustmax as rmx;
use rmx::rand::Rng;
```

These modules behave the same as the corresponding crates,
with exceptions noted in [Macros](#macros).
Each module has `rustmax`-specific documentation
with a description, example, and links to the original crate docs.

The Rust standard libraries are reexported the same way,
as [`core`], [`alloc`], [`std`] and [`proc_macro`] modules.
See [Rust standard libraries](#rust-standard-libraries).

The [`prelude`](crate::prelude) module combines the standard prelude
with commonly-used items from the enabled crates,
like `anyhow`'s `bail!` and `Context`,
`log`'s logging macros,
and extension traits like `Itertools` and `Rng`.

The [`extras`](crate::extras) module holds small additions of `rustmax`'s own,
like the [`bug!`](crate::bug) macro,
the `AnyError` and `AnyResult` aliases,
and [`extras::init`](crate::extras::init) for setting up logging.




## Profiles

`rustmax` organizes crates into _profiles_,
which correspond to common target environments and application types.
By default no profile is enabled and no crates are exported.

The three base profiles build on each other:
[`rmx-profile-portable`] provides every crate in [`rmx-profile-no-std`],
and [`rmx-profile-std`] provides every crate in [`rmx-profile-portable`].
The remaining profiles each add a group of crates on top of [`rmx-profile-std`].


### Profile: `rmx-profile-no-std`

Crates that do not require Rust `std`.
It allows use of the Rust allocator,
and enables allocator-related features of its crates.

Also enables
[`rmx-feature-no-std`],
[`rmx-rustlib-core`] and [`rmx-rustlib-alloc`].

- [`ahash`] - A fast and DoS-resistant hash function, for use in `HashMap`s.
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
- [`libm`] - Floating-point math functions, which `core` does not provide.
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


### Profile: `rmx-profile-portable`

For portable targets including WebAssembly (WASM)
and cross-compiled environments like Linux musl.
A good default for most applications.

It uses portable variants of ecosystem features
that exclude features incompatible with WASM or requiring
a C cross-compiler toolchain, such as OS-specific threading APIs,
file system operations that require native OS support,
and C library dependencies like zstd.

Also enables
[`rmx-feature-std-portable`], [`rmx-feature-default-portable`], [`rmx-feature-more-portable`],
[`rmx-feature-derive`], [`rmx-feature-serde`],
[`rmx-rustlib-core`], [`rmx-rustlib-alloc`] and [`rmx-rustlib-std`].

All crates from [`rmx-profile-no-std`], plus:

- [`clap`] - Command line parsing.
- [`comrak`] - CommonMark and GitHub Flavored Markdown parser.
- [`env_logger`] - A basic logger to use with the [`log`] crate.
- [`flate2`] - Deflate, gzip, and zlib compression and decompression.
- [`glob`] - Unix shell style pattern matching for paths.
- [`http`] - Shared definitions related to the HTTP protocol.
- [`json5`] - JSON5, a superset of JSON with expanded syntax.
- [`mime`] - MIME media types.
- [`rayon`] - Parallel iterators and other parallel processing tools.
- [`regex`] - Regular expressions.
- [`tar`] - Read and write tar archives; pair with [`flate2`] for `.tar.gz`.
- [`tempfile`] - Temporary files and directories.
- [`thiserror`] - Tools for defining custom error types.
- [`unicode-segmentation`](unicode_segmentation) - Splitting strings on grapheme cluster, word, and sentence boundaries.
- [`url`] - URL parsing and data structures.
- [`walkdir`] - Efficient directory traversal.
- [`zip`] - Read and write ZIP archives.


### Profile: `rmx-profile-std`

For native targets with the full Rust standard library.

Unlike [`rmx-profile-portable`] it uses the non-portable variants
of the ecosystem features.

Also enables
[`rmx-feature-std`], [`rmx-feature-default`], [`rmx-feature-more`],
[`rmx-feature-derive`], [`rmx-feature-serde`],
[`rmx-rustlib-core`], [`rmx-rustlib-alloc`] and [`rmx-rustlib-std`].

All crates from [`rmx-profile-portable`], plus:

- [`ctrlc`] - Simple handling of Ctrl-C for CLI programs.
- [`ignore`] - Directory traversal respecting gitignore rules.
- [`image`] - Image processing and manipulation.
- [`indicatif`] - Progress bars and spinners for CLI.
- [`notify`] - Cross-platform filesystem notifications.
- [`proptest`] - Testing over generated inputs, ala QuickCheck.
- [`rustyline`] - Command-line input reading with history.
- [`socket2`] - Low-level network socket programming beyond [`std::net`].
- [`tera`] - A text template engine based on Jinja2.
- [`termcolor`] - Cross-platform library for writing colored output to the terminal.
- [`xshell`] - A Swiss-army knife for writing shell-style scripts in Rust.


### Profile: `rmx-profile-net`

Networking crates, including the [`tokio`] async runtime.
Builds on [`rmx-profile-std`].

- [`hyper`] - HTTP, versions 1 and 2.
- [`reqwest`] - Simple HTTP requests, synchronous and asynchronous.
- [`tokio`] - An async task runtime and I/O library.
- [`tower`] - Service request/response abstraction (HTTP middleware)
              for [`tokio`] and [`hyper`].


### Profile: `rmx-profile-build-script`

Crates for writing [Cargo build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html).
Builds on [`rmx-profile-std`].

- [`bindgen`] - Generate Rust bindings to C and C++ libraries.
- [`cc`] - A basic cross-platform C/C++ compiler driver.
- [`cxx`] - C++ bridge runtime support; paired with [`cxx_build`].
- [`cxx-build`](cxx_build) - C++ bridge generator; paired with [`cxx`].


### Profile: `rmx-profile-proc-macro`

Crates for writing [procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html).
Builds on [`rmx-profile-std`],
and also enables [`rmx-rustlib-proc_macro`].

- [`proc-macro2`](proc_macro2) - A preferred wrapper around the standard [`proc_macro`] crate.
- [`quote`] - The `quote!` macro for turning code blocks into source tokens.
- [`syn`] - A Rust parser used by procedural macros.


### Profile: `rmx-profile-max`

All of the above profiles:
[`rmx-profile-net`], [`rmx-profile-build-script`]
and [`rmx-profile-proc-macro`].


### Profile: `rmx-profile-max-nightly`

[`rmx-profile-max`] plus [`rmx-feature-nightly`].




## Ecosystem features

`rustmax` identifies Cargo features common across many crates,
and enables them across all of its crates at once.

The first group is enabled by the base profiles
and does not typically need to be set manually.
[`rmx-profile-portable`] uses the portable variants,
which exclude features incompatible with portable targets
(WASM, musl cross-compilation),
such as threading, blocking I/O, OS-specific APIs,
and C dependencies like zstd.

| Feature | Portable variant | Enables |
| --- | --- | --- |
| `rmx-feature-no-std` | | Allocator support for no-std crates that can be compiled without it |
| `rmx-feature-std` | `rmx-feature-std-portable` | The "std" feature of crates, and other features that require the standard library |
| `rmx-feature-default` | `rmx-feature-default-portable` | The "default" feature of crates |
| `rmx-feature-more` | `rmx-feature-more-portable` | Extra convenience features that crates do not enable by default |

The second group is opt-in,
though the base profiles other than [`rmx-profile-no-std`]
enable `derive` and `serde`.

| Feature | Enables |
| --- | --- |
| `rmx-feature-derive` | Derive macros, typically a feature named "derive" |
| `rmx-feature-serde` | [`serde`] support, typically a feature named "serde" |
| `rmx-feature-nightly` | Features that only compile with the Rust nightly compiler, typically a feature named "nightly" |

Backtraces need no feature:
[`anyhow`] captures backtraces through [`std::backtrace`],
controlled by the `RUST_BACKTRACE` and `RUST_LIB_BACKTRACE`
environment variables.




## Rust standard libraries

These features reexport the standard Rust libraries
as modules within `rustmax`.
Profiles enable them as appropriate,
via the group features
`rmx-rustlibs-no-std`, `rmx-rustlibs-std` and `rmx-rustlibs-proc-macro`.

| Feature | Reexports | Enabled by |
| --- | --- | --- |
| `rmx-rustlib-core` | [`core`] | all profiles |
| `rmx-rustlib-alloc` | [`alloc`] | all profiles |
| `rmx-rustlib-std` | [`std`] | all profiles except [`rmx-profile-no-std`] |
| `rmx-rustlib-proc_macro` | [`proc_macro`] | [`rmx-profile-proc-macro`] |




## Macros

Macros are the one part of `rustmax` that does not
simply work the way the original crate's documentation says it does.
This section explains why, and what to write instead.

The short version: use [`rmx::derive`](crate::derive)
in place of `#[derive(..)]`.

```rust
# use rustmax as rmx;
#[rmx::derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct Config {
    some_field: u32,
}
```


### Why re-exported macros are different

A derive macro has to generate code that refers back to its own crate.
`serde`'s derive generates calls into `serde`,
`clap`'s derive generates calls into `clap`.
The macro cannot know what the user calls that crate,
so it hardcodes the crate's real name into its output.

That name is resolved in the crate the macro was used in,
where it usually resolves against the _extern prelude_ --
the set of direct Cargo dependencies.
A crate that depends on `rustmax` has `rustmax` in its extern prelude,
not `serde`, so the generated code does not compile.

Nothing about this affects ordinary re-exported items.
Types, traits, functions and `macro_rules!` macros
all work through `rustmax` exactly as they do directly,
because `macro_rules!` macros refer to their own crate
through `$crate`, which survives re-export.
Only proc macros -- derive macros and attribute macros -- are affected.

The affected crates behave in three distinct ways,
which is why there is no single workaround:

| Crate | What its generated code names | Reachable through `rustmax`? |
| --- | --- | --- |
| [`serde`] | `extern crate serde` | yes, via `#[serde(crate = "..")]` |
| [`num_enum`] | `::num_enum` | yes, via `#[num_enum(crate = ..)]` |
| [`clap`] | `clap`, relative to the use site | yes, by binding the name in scope |
| [`derive_more`] | `derive_more`, relative to the use site | yes, by binding the name in scope |
| [`thiserror`] | `::thiserror` | no |
| [`cxx`] | `::cxx` | no |

A leading `::` resolves only against the extern prelude,
so for `thiserror` and `cxx` no re-export, import, or macro
can make the generated code resolve.
Those two need a direct dependency.


### `#[rmx::derive(..)]`

[`rmx::derive`](crate::derive) is an attribute macro
that stands in for `#[derive(..)]`.
It rewrites each derive to its path within `rustmax`
and adds whatever that particular derive crate needs
in order to find its runtime.

```rust
# use rustmax as rmx;
#[rmx::derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Config {
    some_field: u32,
    #[serde(default)]
    other: String,
}

let json = rmx::serde_json::to_string(&Config {
    some_field: 7,
    other: "hi".to_owned(),
})?;
assert_eq!(json, r#"{"someField":7,"other":"hi"}"#);
# Ok::<(), rmx::extras::AnyError>(())
```

Container and field attributes work unchanged,
and names it does not recognize are passed straight through,
so built-in derives can be listed alongside re-exported ones.
It knows the derives of
[`clap`], [`derive_more`], [`num_enum`], [`serde`] and [`thiserror`].

Where a derive name is also a built-in derive --
`Debug`, `Eq` and `PartialEq` all exist in `derive_more` --
the built-in one wins.
Qualify the name to ask for the other:

```rust
# use rustmax as rmx;
#[rmx::derive(derive_more::Debug)]
struct Secret {
    #[debug("redacted")]
    token: String,
}

assert_eq!(
    format!("{:?}", Secret { token: "hunter2".to_owned() }),
    "Secret { token: redacted }",
);
```

`Error` is the reverse case:
both `thiserror` and `derive_more` provide it,
and a bare `Error` means `thiserror`'s,
because that is what it usually means elsewhere.
Since `thiserror` needs a direct dependency,
`#[rmx::derive(Error)]` without one is an error
that says so and points at the alternative.

Adding a direct dependency on one of these crates is always allowed,
and `#[rmx::derive]` notices and uses it,
generating exactly what the crate's own documentation describes.
Mixing the two styles in one crate is fine.


### Importing macros without `#[rmx::derive]`

`#[rmx::derive]` is not the only way,
and for `clap`, `derive_more` and `tokio`
the plain import is barely different from upstream.
These crates name themselves relative to the use site,
so binding the name is enough:

```rust
# use rustmax as rmx;
use rmx::clap::{self, Parser};

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    count: u32,
}

assert_eq!(Cli::parse_from(["prog", "--count", "4"]).count, 4);
```

The `self` in that import is the whole trick:
it binds `clap` as well as `Parser`,
and the generated code needs the former.
Importing only `rmx::clap::Parser` fails to compile.

The same applies to attribute macros,
which `#[rmx::derive]` does not cover:

```rust,ignore
# use rustmax as rmx;
use rmx::tokio;

#[tokio::main]
async fn main() {
    // ...
}
```

For `serde`, the equivalent is its `crate` attribute,
which takes the path to use in generated code:

```rust
# use rustmax as rmx;
use rmx::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "::rustmax::serde")]
struct Point {
    x: f64,
    y: f64,
}
```

Note that the path there is the real crate name.
If the dependency is renamed, as the template renames it to `rmx`,
the attribute has to say `"::rmx::serde"` instead.
`#[rmx::derive]` exists partly to avoid having to know that:
it discovers the name from the depending crate's manifest.


### Adding a direct dependency

For `thiserror` and `cxx` this is the only option,
and it is a reasonable one for any of the affected crates.
Add the crate alongside `rustmax`:

```toml
[dependencies]
rmx.package = "rustmax"
rmx.version = "0.0.10"
rmx.features = ["rmx-profile-portable"]

thiserror = "2"
```

This costs less than it looks like it does.
Cargo unifies the two references to a single build of the crate,
so nothing is compiled twice,
and `rmx::serde::Serialize` and `serde::Serialize`
are the same trait, not two versions of it.
What the extra line buys is a name in the extern prelude.

The cost is that the version is now declared in two places,
which is exactly what `rustmax` exists to avoid.
Keep the version ranges loose enough
that Cargo can unify them with whatever `rustmax` selects.


### Known limitations

`#[rmx::derive]` reads the depending crate's manifest
to decide whether a crate is a direct dependency.
The manifest does not distinguish `[dependencies]` from `[dev-dependencies]`,
but the extern prelude of a given build target contains only one of them.
A crate listed under `[dependencies]` but not `[dev-dependencies]`
is treated as available inside integration tests, where it is not.
Adding it to both sections resolves this.

Writing `#[serde(crate = "..")]` on a type
that also has `#[rmx::derive(Serialize)]`
produces a duplicate-attribute error from `serde`.
Use one or the other.

Proc macro functions cannot be called from other proc macros,
so `rustmax` cannot provide drop-in replacements
for the derive macros themselves,
only an attribute that rewrites what is handed to them.
Cargo has no mechanism for a library crate
to add entries to its dependents' extern prelude;
the unstable `public-dependency` feature does not do this.
That is why `thiserror` and `cxx` have no workaround.




<!-- links -->

[`rmx-profile-no-std`]: #profile-rmx-profile-no-std
[`rmx-profile-std`]: #profile-rmx-profile-std
[`rmx-profile-portable`]: #profile-rmx-profile-portable
[`rmx-profile-net`]: #profile-rmx-profile-net
[`rmx-profile-build-script`]: #profile-rmx-profile-build-script
[`rmx-profile-proc-macro`]: #profile-rmx-profile-proc-macro
[`rmx-profile-max`]: #profile-rmx-profile-max
[`rmx-feature-no-std`]: #ecosystem-features
[`rmx-feature-std`]: #ecosystem-features
[`rmx-feature-std-portable`]: #ecosystem-features
[`rmx-feature-default`]: #ecosystem-features
[`rmx-feature-default-portable`]: #ecosystem-features
[`rmx-feature-more`]: #ecosystem-features
[`rmx-feature-more-portable`]: #ecosystem-features
[`rmx-feature-derive`]: #ecosystem-features
[`rmx-feature-serde`]: #ecosystem-features
[`rmx-feature-nightly`]: #ecosystem-features
[`rmx-rustlib-core`]: #rust-standard-libraries
[`rmx-rustlib-alloc`]: #rust-standard-libraries
[`rmx-rustlib-std`]: #rust-standard-libraries
[`rmx-rustlib-proc_macro`]: #rust-standard-libraries
