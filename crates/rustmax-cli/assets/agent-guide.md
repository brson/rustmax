# Rustmax for coding agents

This guide is for coding agents working in a Rust project that uses
`rustmax`. It describes rustmax 0.0.13. The same text is printed by
`rustmax agents` and published at <https://rustmax.net/agent-guide.md>.

`rustmax` is a single crate that re-exports a curated set of
crates.io crates (serde, clap, anyhow, rand, rayon, tokio and about
seventy others) at versions chosen to work together. A project
depends on `rustmax` instead of on those crates individually, and
reaches them through it: `rmx::serde`, `rmx::clap`, `rmx::rand`.

Most mistakes in rustmax projects come from writing code as if the
re-exported crates were direct dependencies, or from using APIs of
older versions of those crates than the ones rustmax provides.


## Recognizing a rustmax project

The workspace `Cargo.toml` declares it, usually renamed to `rmx`:

```toml
[workspace.dependencies]
rmx.package = "rustmax"
rmx.version = "0.0.13"
rmx.features = [
  "rmx-profile-portable",
]
```

and each member crate takes it with `rmx.workspace = true`.

Read the `rmx.features` list before writing any code. It decides
which crates exist under `rmx::` (see "Profiles" below).


## Rules

1. Start every module with `use rmx::prelude::*;`.
2. Reach third-party crates through `rmx::<crate>`, as in
   `rmx::serde_json::to_string`. Do not add a direct dependency on a
   crate that rustmax already provides. Before adding any new
   dependency, check whether rustmax re-exports it.
3. Derive through `#[rmx::derive(..)]`, not `#[derive(..)]`, whenever a
   derive comes from a re-exported crate (serde, clap, derive_more,
   num_enum). Plain `#[derive(..)]` is fine for built-in derives.
4. Return `AnyResult<T>` from fallible application code, and use
   `bail!`, `anyhow!` and `.context(..)` from the prelude.
5. The re-exported crates are recent versions. If you remember an API
   from an older version, check it before using it (see "Looking up
   APIs").
6. Follow the conventions already in the codebase. If the surrounding
   code uses `.S()` and `.C()` (see "Extras"), so should yours.


## The prelude

`use rmx::prelude::*` brings in the standard prelude plus, when the
corresponding crate is enabled:

- From anyhow: `AnyResult`, `AnyError`, `bail!`, `anyhow!`, and the
  `Context` trait (so `.context(..)` and `.with_context(..)` work).
- From log: `trace!`, `debug!`, `info!`, `warn!`, `error!`.
- Extension traits, imported anonymously so their methods work:
  `Itertools`, rand's `Rng`, `RngExt`, `SeedableRng` and `Fill`.
- From futures: `futures::prelude::*`, `block_on` and `Either`.
- `cfg_if!` and `#[extension_trait]`.
- `core::cmp::Ordering`.
- From rustmax itself: `default()`, `bug!`, and the powerletters
  methods `.C()`, `.O()`, `.S()`, `.I()` and `.X()`.

Because the prelude is a glob import, a local item with the same
name as one of these (an enum variant called `Either`, a function
called `default`) silently takes precedence. That is usually what
you want, but be aware of it when an import seems to be ignored.


## Using a re-exported crate

Every enabled crate is a module of `rmx` with the same name as the
crate's library, using underscores: `rmx::serde_json`,
`rmx::rustc_hash`, `rmx::unicode_segmentation`. These modules behave
exactly like the original crates, apart from derive and attribute
macros.

```rust
use rmx::prelude::*;
use rmx::rayon::prelude::*;
use rmx::rustc_hash::FxHashMap;

fn count(words: &[String]) -> FxHashMap<String, usize> {
    let mut map: FxHashMap<String, usize> = default();
    for w in words {
        *map.entry(w.C()).or_default() += 1;
    }
    map
}

fn total(xs: &[u64]) -> u64 {
    xs.par_iter().sum()
}
```

Common imports seen in rustmax codebases:

```rust
use rmx::clap::{self, Parser as _};
use rmx::futures::channel::{mpsc, oneshot};
use rmx::futures::executor::block_on;
use rmx::proptest::prelude::*;
use rmx::rayon::prelude::*;
use rmx::serde::{Deserialize, Serialize};
use rmx::serde_json::json;
```

The standard libraries are re-exported too, as `rmx::std`,
`rmx::core` and `rmx::alloc`. `rmx::std::path::PathBuf` and
`std::path::PathBuf` are the same item. Plain `std::` is the more
common style. Do not rewrite one form into the other.

In tests use `rmx::tempfile` for temporary files and directories:

```rust
use rmx::prelude::*;

#[test]
fn writes_output() -> AnyResult<()> {
    let dir = rmx::tempfile::tempdir()?;
    std::fs::write(dir.path().join("out.txt"), "hi")?;
    Ok(())
}
```


## Derive and attribute macros

Proc macros generate code that names their own crate, such as
`serde` or `clap`. In a rustmax project that crate is not a direct
dependency, so the generated code does not compile:

```rust
use rmx::serde::Serialize;

#[derive(Serialize)] // error[E0463]: can't find crate for `serde`
struct Point { x: f64, y: f64 }
```

Use `#[rmx::derive(..)]` instead. It takes the same list as
`#[derive(..)]`, accepts built-in derives alongside re-exported ones,
and leaves helper attributes like `#[serde(..)]` working as usual:

```rust
use rmx::prelude::*;

#[rmx::derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Config {
    some_field: u32,
    #[serde(default)]
    other: String,
}
```

It makes the derives of serde, clap, derive_more and num_enum work
through rustmax. Where derive_more and the standard library share a
name (`Debug`, `PartialEq`), the built-in one wins. Write
`derive_more::Debug` to get the other.

thiserror is the exception. A bare `Error` in `#[rmx::derive]` means
thiserror's `Error`, which only compiles when `thiserror` is a direct
dependency; without one it is a compile error. Use
`derive_more::Error` to stay within rustmax (see below).

`#[rmx::derive]` was added in rustmax 0.0.11. Older projects use the
workarounds below, and they keep working:

- serde: `#[derive(Serialize)]` plus `#[serde(crate = "rmx::serde")]`
  on every derived type. Do not combine this attribute with
  `#[rmx::derive]`. That is a duplicate-attribute error.
- clap: import the crate name as well as the trait,
  `use rmx::clap::{self, Parser as _};`, then
  `#[derive(clap::Parser)]`. Without the `self`, the derive fails with
  "cannot find module or crate `clap`".

Specific crates:

- **thiserror** cannot work through a re-export at all, because its
  generated code names `::thiserror`. Either add `thiserror = "2"` as
  a direct dependency (this is the one allowed exception to rule 2),
  or use derive_more instead:
  `#[rmx::derive(Debug, Display, derive_more::Error)]`.
  Hand-written `impl std::error::Error` is also common in rustmax
  codebases.
- **tokio** attribute macros are written `#[rmx::tokio::main]` and
  `#[rmx::tokio::test]`. They accept the usual arguments, such as
  `flavor = "current_thread"`.
- Function-like macros (`tokio::select!`, `futures::join!`,
  `serde_json::json!`) work through the re-export without help.


## Profiles and enabling crates

Rustmax enables nothing by default. A profile feature selects a set
of crates:

- `rmx-profile-no-std`: no_std crates using `alloc`, including anyhow,
  serde, serde_json, toml, rand, itertools, log, parsing (nom),
  hashing (blake3, sha2, ahash, rustc-hash) and dates (jiff, chrono).
- `rmx-profile-portable`: everything above plus clap, rayon, regex,
  tempfile, thiserror, env_logger, url, walkdir, flate2, tar,
  zip and others. Compiles for WASM and musl. The default choice for
  new projects.
- `rmx-profile-std`: everything above plus native-only crates: ignore,
  image, indicatif, notify, proptest, rustyline, tera, termcolor,
  xshell, ctrlc, socket2.
- `rmx-profile-tokio`: everything above plus tokio, hyper, reqwest and
  tower.
- `rmx-profile-build-script` (cc, bindgen) and
  `rmx-profile-proc-macro` (syn, quote, proc-macro2).
- `rmx-profile-max`: all of them.

If `rmx::foo` does not resolve ("cannot find `foo` in `rmx`"), the
crate is not enabled. Do not add it as a direct dependency. Enable it
in the workspace `rmx.features` by its package name, which is the
crates.io name, with hyphens:

```toml
rmx.features = [
  "rmx-profile-portable",
  "tera",
  "rustc-hash",
]
```

A crate that should only be available to one member, or only in
tests, can be enabled there instead:

```toml
[dev-dependencies]
rmx = { workspace = true, features = ["proptest"] }
```

Changing the profile, or enabling crates outside the current one,
changes the project's dependency footprint and target support. Make
that change only when the task requires it, and say so in your
summary.

### Enabling a feature of a re-exported crate

Profiles turn on a curated set of each crate's own features. When you
need one that is not on, add a direct dependency that exists only to
enable it, with a version range compatible with the one rustmax
uses. Cargo unifies the two into one build, so `rmx::serde_json`
gains the feature and nothing is duplicated:

```toml
[dependencies]
rmx.workspace = true
# Feature-only: keeps serde_json object keys in insertion order.
serde_json = { version = "1", default-features = false, features = ["preserve_order"] }
```

Keep using `rmx::serde_json::..` paths in code. Check the version
rustmax uses with `cargo tree -i serde_json` before writing the range.


## Versions and outdated APIs

Rustmax tracks recent releases, so the versions are likely newer than
the ones in your training data. Wrong guesses compile-fail at best and
silently pull in a second, older copy of the crate at worst, if you
add a direct dependency to make old code work. Do not do that.

Known renames that agents get wrong:

- rand 0.9 and later: `thread_rng()` is `rng()`, `gen()` is
  `random()`, `gen_range(..)` is `random_range(..)`,
  `rand::distributions` is `rand::distr`.
- base64: the free functions `encode` and `decode` are gone. Use an
  engine: `use rmx::base64::prelude::*;` then
  `BASE64_STANDARD.encode(..)`.

### Looking up APIs

Find the exact version in use, then read that version, not memory:

```sh
cargo tree -i rand --depth 0      # The version rustmax resolved.
cargo doc -p rand --no-deps       # HTML docs under target/doc/.
```

The source of that version is under
`~/.cargo/registry/src/*/rand-<version>/`, which is often the fastest
thing to search.

If the `ferritin` tool is installed, it prints docs as text and
follows rustmax re-exports. It needs the real package name rather
than the `rmx` alias:

```sh
ferritin -m Cargo.toml get --format agent rustmax::rand::rngs::StdRng
ferritin -m Cargo.toml search --format agent rand "random value in range"
```

Each re-exported module also has rustmax-specific documentation with
an example (`cargo doc -p rustmax`, then open the module, such as
`rustmax::rand`).


## Error handling

Application code uses anyhow through the prelude:

```rust
use rmx::prelude::*;

fn load(path: &std::path::Path) -> AnyResult<Config> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("reading {}", path.display()))?;
    let config: Config = rmx::serde_json::from_str(&text)
        .context("parsing config")?;
    if config.some_field == 0 {
        bail!("some_field must be nonzero");
    }
    Ok(config)
}
```

`AnyResult<T>` is `anyhow::Result<T>` and `AnyError` is
`anyhow::Error`. For typed errors in libraries, see the thiserror
notes under "Derive and attribute macros".

Use `bug!("..")` for states that indicate a bug in the program. It
panics with a recognizable message and accepts format arguments.


## Extras

`rmx::extras` and the prelude add a few small conveniences:

- `default()`: `Default::default()` with the type inferred, as in
  `let map: FxHashMap<_, _> = default();`.
- `bug!(..)`: an unconditional panic for impossible states.
- `rmx::extras::init_crate_name(env!("CARGO_CRATE_NAME"))`: set up
  env_logger at `info` for the calling crate, overridable with
  `RUST_LOG`. The usual first line of `main`. `rmx::extras::init()`
  is the same for all crates.
- Powerletters, single-letter methods on any value:
  `.C()` is `.clone()`, `.O()` is `.to_owned()`, `.S()` is
  `.to_string()`, `.I()` discards a `Result`, and `.X()` is `.expect()`
  with a generic message ("impossible `Err` result").

Powerletters are a house style. Some rustmax codebases use them
everywhere and others not at all. Match the file you are editing, and
do not convert existing code unless asked.


## A typical binary

```rust
use rmx::prelude::*;
use rmx::clap::{self, Parser as _};
use std::path::PathBuf;

#[derive(clap::Parser)]
struct Cli {
    #[arg(long, default_value = "config.json")]
    config: PathBuf,
}

fn main() -> AnyResult<()> {
    rmx::extras::init_crate_name(env!("CARGO_CRATE_NAME"));
    let cli = Cli::parse();
    info!("loading {}", cli.config.display());
    Ok(())
}
```


## When not to use rmx

Rustmax is one dependency but not a small one. Some crates in a
rustmax workspace deliberately do not depend on it, and you should
not add it to them:

- **no_std crates.** Cargo unifies features across the workspace, so
  a no_std crate that takes the workspace's `rmx` with a std profile
  is no longer no_std in its dependency graph. Use `core` and `alloc`
  directly, or a direct dependency with `default-features = false`.
- **staticlib and cdylib crates.** Everything rustmax enables can end
  up in the artifact. One project cut a runtime staticlib from 156 MB
  to 41 MB by dropping rmx from it.
- **Small reusable libraries** where the project has said it wants a
  modest dependency footprint.

Crates that rustmax does not re-export (salsa, wgpu, winit,
cranelift, and so on) are ordinary direct dependencies.

If a crate's manifest has no `rmx` dependency while its neighbors
do, assume that is intentional, and look for the reason in the
project's docs before adding one.


## WebAssembly

`rmx-profile-portable` compiles for `wasm32-unknown-unknown`, and in
doing so enables `getrandom`'s `js` backend and chrono's `wasmbind`.
Those import wasm-bindgen functions, so a module built for a
non-browser runtime (wasmi, wasmtime) needs those imports satisfied,
even if nothing calls them.


## Error messages and their fixes

- `can't find crate for 'serde'` (E0463) on a derive: use
  `#[rmx::derive(..)]`, or on rustmax 0.0.11 and older add
  `#[serde(crate = "rmx::serde")]`.
- `cannot find module or crate 'clap'` on a derive: import with
  `use rmx::clap::{self, ..}`, or use `#[rmx::derive(Parser)]`.
- "`thiserror` derives only work when `thiserror` is a direct
  dependency": add `thiserror = "2"`, or switch to
  `derive_more::Error`.
- `cannot find 'foo' in 'rmx'` (E0433): the crate is not enabled. Add it to
  `rmx.features` (see "Profiles").
- `cannot find function 'rng' in module 'rmx::rand'`: the project is
  on rustmax 0.0.12 or older with `rmx-profile-portable`, which lacks
  the thread-local generator. Use a seeded `StdRng`, or enable
  `rand/thread_rng` with a feature-only dependency (see "Enabling a
  feature of a re-exported crate").
- A method from an extension trait is missing (`.sorted()`,
  `.random_range()`, `.context()`): add `use rmx::prelude::*;`.


## Upgrading rustmax

1. Bump `rmx.version` in the workspace manifest.
2. `cargo update -p rustmax`, then build and test everything,
   including any wasm or no_std checks the project has.
3. Fix breakage from upgraded crates by reading their changelogs
   (`~/.cargo/registry/src/*/<crate>-<version>/CHANGELOG.md`).
4. Look for direct dependencies that the new version now provides,
   and move them to `rmx::` paths, unless they exist only to enable a
   feature.
