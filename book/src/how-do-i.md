# How Do I … in Rust?

**Discovery:**
[find a crate for a given purpose](#-find-a-crate-for-a-given-purpose) ·
[find the latest version of a crate](#-find-the-latest-version-of-a-crate)

**Project organization and maintenance:**
[init a Rustmax project from template](#-init-a-rustmax-project-from-template) ·
[organize a Rust workspace](#-organize-a-rust-workspace) ·
[fix bugs in a published crates.io dependency](#-fix-bugs-in-a-published-cratesio-dependency)

**Tooling and the command line:**
[find all Rust files](#-find-all-rust-files) ·
[replace text in all Rust files](#-replace-text-in-all-rust-files) ·
[delete trailing whitespace in all Python and Rust files](#-delete-trailing-whitespace-in-all-python-and-rust-files) ·
[free up the disk space used by `target/` directories](#-free-up-the-disk-space-used-by-target-directories) ·
[speed up linking](#-speed-up-linking) ·
[stop `rustfmt` from reformatting some code](#-stop-rustfmt-from-reformatting-some-code) ·
[keep a project's common development commands at hand](#-keep-a-projects-common-development-commands-at-hand)

**Conveniences:**
[import a trait just to call its methods](#-import-a-trait-just-to-call-its-methods) ·
[get an `Either` type](#-get-an-either-type) ·
[define "extension" methods on a type in another crate](#-define-extension-methods-on-a-type-in-another-crate) ·
[guarantee a trait is object-safe](#-guarantee-a-trait-is-object-safe) ·
[make compile-time assertions](#-make-compile-time-assertions) ·
[assert a type is `Send` and `Sync`](#-assert-a-type-is-send-and-sync)

**Error handling and debugging:**
[handle errors simply and correctly](#-handle-errors-simply-and-correctly) ·
[structure errors in a public API](#-structure-errors-in-a-public-api) ·
[use `?` with `Option`](#-use--with-option) ·
[make a block that uses `?` typecheck](#-make-a-block-that-uses--typecheck) ·
[capture a backtrace](#-capture-a-backtrace) ·
[set up basic logging](#-set-up-basic-logging)

**Collections:**
[create a fast `HashMap`](#-create-a-fast-hashmap) ·
[convert from slices to fixed-length arrays](#-convert-from-slices-to-fixed-length-arrays) ·
[Implement an `impl Iterator` with `todo!`](#-implement-an-impl-iterator-with-todo)

**Numerics:**
[convert between numeric types ideomatically](#-convert-between-numeric-types-ideomatically) ·
[perform math ideomatically](#-perform-math-ideomatically) ·
[convert between ints and bytes](#-convert-between-ints-and-bytes) ·
[perform typical byte order conversions](#-perform-typical-byte-order-conversions)

**Encoding, serialization, parsing:**
[serialize to and from JSON](#-serialize-to-and-from-json) ·
[decide what format to use with `serde`](#-decide-what-format-to-use-with-serde)

**Time:**
[parse and render standard time formats](#-parse-and-render-standard-time-formats)

**Random numbers:**
[generate a strong random anything](#-generate-a-strong-random-anything) ·
[generate a strong random number from a seed](#-generate-a-strong-random-number-from-a-seed) ·
[generate a fast random number from a seed](#-generate-a-fast-random-number-from-a-seed) ·
[generate a strong random number from a seed with stable algorithm](#-generate-a-strong-random-number-from-a-seed-with-stable-algorithm) ·
[generate a fast random number from a seed with stable algorithm](#-generate-a-fast-random-number-from-a-seed-with-stable-algorithm) ·
[generate random numbers without the `rand` crate](#-generate-random-numbers-without-the-rand-crate)

**Cryptography:**
[calculate a cryptographic content hash](#-calculate-a-cryptographic-content-hash)

**Parallelism and Concurrency:**
[initialize a global value](#-initialize-a-global-value) ·
[send messages to/from async code](#-send-messages-tofrom-async-code) ·
[use a thread pool](#-use-a-thread-pool) ·
[get the number of CPUs](#-get-the-number-of-cpus)

**Asynchronous I/O:**
[set up the `tokio` event loop](#-set-up-the-tokio-event-loop) ·
[stub an unwritten `async fn`](#-stub-an-unwritten-async-fn) ·
[merge message streams in an event loop](#-merge-message-streams-in-an-event-loop)

**Networking and web:**
[make a synchronous HTTP request](#-make-a-synchronous-http-request) ·
[configure a basic HTTP server](#-configure-a-basic-http-server)

**Terminal / CLI:**
[set up a simple CLI parser with subcommands](#-set-up-a-simple-cli-parser-with-subcommands) ·
[display colors in the terminal](#-display-colors-in-the-terminal) ·
[read line-based input from the terminal](#-read-line-based-input-from-the-terminal) ·
[handle ctrl-](#-handle-ctrl-)

**System / OS:**
[read environment variables](#-read-environment-variables) ·
[work with a temporary file](#-work-with-a-temporary-file) ·
[work with multiple files in a temporary directory](#-work-with-multiple-files-in-a-temporary-directory) ·
[copy a directory recursively](#-copy-a-directory-recursively)

**Testing:**
[create a custom test harness](#-create-a-custom-test-harness) ·
[create a custom table-based test harness](#-create-a-custom-table-based-test-harness)

**Build scripts:**
[write build scripts ideomatically](#-write-build-scripts-ideomatically) ·
[link to a native static library](#-link-to-a-native-static-library) ·
[compile and link a C source file](#-compile-and-link-a-c-source-file)

**FFI / interop:**
[create Rust bindings to a C/C++ program](#-create-rust-bindings-to-a-cc-program)



## Discovery

### … find a crate for a given purpose?

### … find the latest version of a crate?




## Project organization and maintenance

### … init a Rustmax project from template?

```
cargo generate brson/rustmax
```


### … organize a Rust workspace?

todo

- sharing dependencies in the workspace etc




### … fix bugs in a published crates.io dependency?

Use `[patch.crates-io]` in your `Cargo.toml` to override any crates.io dependency with your fixed version.

**Workflow:**

1. Fork/clone the dependency.
2. Check out the commit the corresponds to the revision in your lockfile.
   Sometimes projects will have obvious git tags for each version.
   Otherwise published packages contain a `.cargo_vcs_info.json` with that info.
   It can be [viewed directly on docs.rs](https://docs.rs/crate/rustmax/latest/source/.cargo_vcs_info.json).
2. Make your fix.
3. Patch your workspace's `Cargo.toml` to use the fixed version:

   **Option A: Patch with a local path** (for development)

   ```toml
   [patch.crates-io]
   some-crate = { path = "../some-crate" }
   ```

   **Option B: Patch with a git repo** (for sharing/CI)

   ```toml
   [patch.crates-io]
   some-crate = { git = "https://github.com/you/some-crate", branch = "my-fix" }
   ```
4. Publish the dependency with a new version number
   (or get the owner of the dependency to publish if it's a third-party crate).
5. Update your workspace's dependency to use use the new version number
   and remove the patch.

The `patch.crates-io` section requires a patch's source package has
the same declared version as the published source.
This should be the case by default if you have checked out the commit the corresponds
exactly to the published package.


#### Alternate: just use `path` dependencies temporarily.

The main benefit of using `patch` is that it applies the dependency
change to all instances of that crate in the workspace.

Well-organized workspaces declare shared dependencies in the root `Cargo.toml`:

```toml
# Root Cargo.toml
[workspace.dependencies]
some-crate = "1.0"
# or temporarily:
some-crate = { path = "../some-crate" }
```

```toml
# Crate Cargo.toml
[dependencies]
some-crate.workspace = true
```

With this setup, switching to a path dependency in one place updates all crates.
No `[patch]` needed, and no version-matching constraints.
This tends to be an easier workflow for simple local fixes.




## Tooling and the command line

### … find all Rust files?

```
fd -e rs
```


### … replace text in all Rust files?

```
fd -e rs -x sd "before_regex" "after"
```


### … delete trailing whitespace in all Python and Rust files?

```
fd -e py -e rs -x sd "\s+$" "\n"
```


### … free up the disk space used by `target/` directories?

Everyone finds out eventually: the Rust compiler stores
a large amount of data in your `target/` directories,
and doesn't clean up after itself.

The `cargo clean-all` plugin finds these directories
and helps you delete them, interactively by default.

```
cargo install cargo-clean-all
cargo clean-all
```


### … speed up linking?

Linking is one of the most time-consuming
stages of a Rust build,
and it has to be redone every time you test your program.

On Linux the [`mold`] linker is faster than
the default linker.
Setting it up manually is not difficult,
but just hard enough that I have to look it up
every time.

The Rustmax CLI tool can do it instead:

`rustmax install-tool mold`

[`mold`]: https://github.com/rui314/mold


### … stop `rustfmt` from reformatting some code?

Sometimes exact formatting is important to make code beautiful.
Don't be afraid to annotate it with `#[rustfmt::skip]`.


### … keep a project's common development commands at hand?

Almost every project has a handful of commands the developer(s)
uses frequently. Put these in a `justfile` so the menu of
commands for this project is always obvious, which
can be extra helpful after years away from a project.

```
cargo install just
```

`just` runs commands listed in a file named `justfile`.
The `justfile` lives in your project's root directory,
and is configured with a `make`-like syntax:

```just
default:
    just --list

install-tools:
    cargo install mdbook
    cargo install mdbook-yapp

clean: doc-clean
    cargo clean

doc-clean:
    rm -rf out
```

It's a simple idea, but surprisingly useful. And don't worry that it looks like
a `Makefile` &mdash; it is much more fun and sensible in use than `make`.

When you come back to a project and see there's a justfile you
know to run `just --list` and you'll immediately see what
was on the previous maintainer's mind.

```
$ just --list
Available recipes:
    build
    check
    clean
    default
    doc-book
    doc-build
    doc-clean
    doc-crates
    install-tools
    lint
    maint-audit
    maint-duplicates
    maint-lock-minimum-versions # useful prior to running `cargo audit`
    maint-outdated
    maint-upgrade
    publish
    publish-dry
    replace-version old new
    test
    test-min-version-build
```




## Conveniences

### … import a trait just to call its methods?

Most of the time we need traits to call a method,
and for this we need the trait to be in scope,
but it doesn't need to be nameable.
Import it with `as _`:

```rust
use std::io::{self, Seek as _};

let hash = read_hash(&mut reader)?;
reader.seek(io::SeekFrom::Start(0))?;
```


### … get an `Either` type?

The `Either` type, most known from Haskell, is a useful
reusable abstraction for the common need to create a lightweight type
that is a named union of two other types. It typically looks like:

```
enum Either<A, B> {
  Left(A),
  Right(B),
}
```

Rust doesn't provide this type because it instead provides `Result`,
which is the most common use-case for the pattern, and the remaining
use-cases seem marginal for such a simple type.
Official guidelines encourage writing your own `Either`-like types
for specific uses, with more specific names.

Sometimes you want one though!

There's one hiding in the futures crate: [`futures::future::Either`].

Although `Either` is a `Future` and provided for awaiting pairs of futures,
its definition is exactly as above.

[`futures::future::Either`]: https://docs.rs/futures/latest/futures/future/enum.Either.html

### … define "extension" methods on a type in another crate?

### … guarantee a trait is object-safe?

### … make compile-time assertions?

Use inline const blocks (Rust 1.79+):

```rust
// Assert a condition at compile time
const { assert!(BUFFER_SIZE > 0) }

// Assert types have the same size
const { assert!(std::mem::size_of::<u64>() == std::mem::size_of::<usize>()) }

// Assert a type implements traits
const fn assert_impls<T: Clone + Send>() {}
const { assert_impls::<String>() }
```


### … assert a type is `Send` and `Sync`?

```
struct DbPathGen(());

const _ASSERT_SEND_SYNC: () = assert_send_sync::<DbPathGen>();
const fn assert_send_sync<T: Send + Sync>() { }
```

Also in `rustmax::extras::assert_send_sync`.




## Error handling and debugging

### … handle errors simply and correctly?

### … structure errors in a public API?


### … use `?` with `Option`?

The `?` operator works with `Option` to early-return on `None`,
making option-heavy code much cleaner.

```rust
fn find_user_email(id: u32) -> Option<String> {
    let user = database.find_user(id)?;  // Return None if user not found
    let profile = user.get_profile()?;   // Return None if no profile
    let email = profile.email?;          // Return None if no email
    Some(email)
}
```

An alternative to `match` or `if let` statements, `?` lets you chain
operations that might fail, automatically propagating `None` up the call stack.


### … make a block that uses `?` typecheck?

Sometimes you have a block of code that uses `?`
but otherwise doesn't return a value,
and it won't typecheck without a hint.

Usually the prettiest solution is to
write out the full `Ok` constructor,
using the [turbofish](concept-index.md#turbofish),
in the block's tail expression,
`Ok::<(), WhateverError>(())`.

```
block_on(async {
    let scan_completion = engine.start_scan(config.C(), None).await?;
    scan_completion.await??;
    Ok::<(), AnyError>(())
})?;
```




### … capture a backtrace?

Use [`std::backtrace::Backtrace`], stabilized in Rust 1.65:

```rust
use std::backtrace::Backtrace;

let bt = Backtrace::capture();
println!("{bt}");
```

Set `RUST_BACKTRACE=1` to enable capture.

For error types, `anyhow` automatically captures backtraces when the `backtrace` feature is enabled.

[`std::backtrace::Backtrace`]: https://doc.rust-lang.org/std/backtrace/struct.Backtrace.html




### … set up basic logging?




## Collections

### … create a fast `HashMap`?

### … convert from slices to fixed-length arrays?

### … Implement an `impl Iterator` with `todo!`?

```
  fn merge_create_accounts_results(                      
      accounts: &[tb::Account],
      results: Vec<tb::CreateAccountsResult>,
  ) -> impl Iterator<Item = (u128, Option<tb::Account>)> + use<'_> {
      todo!(); // optional
      std::iter::empty() // satisfies the type checker
  }  
```




## Numerics

### … convert between numeric types ideomatically?

### … perform math ideomatically?

### … convert between ints and bytes?

### … perform typical byte order conversions?

Since Rust 1.32, all integer types have built-in methods for byte order conversion:

```rust
// Convert integers to bytes
let x: u32 = 0x12345678;
let be_bytes = x.to_be_bytes(); // big-endian: [0x12, 0x34, 0x56, 0x78]
let le_bytes = x.to_le_bytes(); // little-endian: [0x78, 0x56, 0x34, 0x12]
let ne_bytes = x.to_ne_bytes(); // native-endian (platform-dependent)

// Convert bytes back to integers
let y = u32::from_be_bytes([0x12, 0x34, 0x56, 0x78]); // 0x12345678
let z = u32::from_le_bytes([0x78, 0x56, 0x34, 0x12]); // 0x12345678
```

These methods work on all integer types: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, and their signed equivalents.




## Encoding, serialization, parsing

### … serialize to and from JSON?

### … decide what format to use with `serde`?




## Time

### … parse and render standard time formats?




## Random numbers

### … generate a strong random anything?

Use [`rand::random`] for convenience when you need cryptographically secure randomness without managing state.

```rust
let x: u32 = rand::random();
let y: f64 = rand::random(); // 0.0..1.0
```

### … generate a strong random number from a seed?

Use [`rand::rngs::StdRng`] when you need reproducible cryptographically secure randomness. This uses the platform's secure RNG algorithm.

```rust
use rand::{RngExt, SeedableRng};
let mut rng = rand::rngs::StdRng::seed_from_u64(42);
let x: u32 = rng.random();
```

### … generate a fast random number from a seed?

Use [`rand::rngs::SmallRng`] for performance-critical code where cryptographic security isn't required. This automatically selects a fast algorithm.

```rust
use rand::{RngExt, SeedableRng};
let mut rng = rand::rngs::SmallRng::seed_from_u64(42);
let x: u32 = rng.random();
```

### … generate a strong random number from a seed with stable algorithm?

Use [`rand_chacha::ChaCha12Rng`] when you need reproducible results across Rust versions and platforms with cryptographic security.

```rust
use rand::{RngExt, SeedableRng};
let mut rng = rand_chacha::ChaCha12Rng::seed_from_u64(42);
let x: u32 = rng.random();
```

### … generate a fast random number from a seed with stable algorithm?

Use [`rand_pcg::Pcg64`] for deterministic, fast random numbers that remain consistent across platforms and Rust versions.

```rust
use rand::{RngExt, SeedableRng};
let mut rng = rand_pcg::Pcg64::seed_from_u64(42);
let x: u32 = rng.random();
```

### … generate random numbers without the `rand` crate?

You should use the [`rand`] crate for random numbers generally.
But if for some reason you need random numbers,
can't use `rand`, but _can_ use `std`,
then you can pull reasonably-strong random `u64`s
from the standard [`Hasher`]:

```rust
fn random_u64() -> u64 {
    std::hash::Hasher::finish(&std::hash::BuildHasher::build_hasher(
        &std::collections::hash_map::RandomState::new(),
    ))
}
```

In practice this RNG is cryptographically strong on most platforms,
but that is not guaranteed nor documented.
On platforms with no random number generator,
like `wasm-unknown-unknown`, these numbers might be fully guessable.

[`rand`]: https://docs.rs/rand
[`Hasher`]: https://doc.rust-lang.org/std/hash/trait.Hasher.html
[`rand::random`]: https://docs.rs/rand/latest/rand/fn.random.html
[`rand::rngs::StdRng`]: https://docs.rs/rand/latest/rand/rngs/struct.StdRng.html
[`rand::rngs::SmallRng`]: https://docs.rs/rand/latest/rand/rngs/struct.SmallRng.html
[`rand_chacha::ChaCha12Rng`]: https://docs.rs/rand_chacha/latest/rand_chacha/struct.ChaCha12Rng.html
[`rand_pcg::Pcg64`]: https://docs.rs/rand_pcg/latest/rand_pcg/type.Pcg64.html



## Cryptography




### … calculate a cryptographic content hash?




## Parallelism and Concurrency

### … initialize a global value?

todo `LazyLock`, `OnceLock`, and `Once`.

### … send messages to/from async code?

todo futures::channels

### … use a thread pool?

Use [`rayon::ThreadPool`].

Although it has additional rayon features,
it can be used as a basic thread pool.

todo example

[`rayon::ThreadPool`]: https://docs.rs/rayon/latest/rayon/struct.ThreadPool.html

### … get the number of CPUs?

```rust
let cpus = std::thread::available_parallelism()?.get();
```


## Asynchronous I/O

### … set up the `tokio` event loop?

### … stub an unwritten `async fn`?

### … merge message streams in an event loop?

An async event loop often needs to receive messages from multiple sources.
Use `futures::stream::select` to merge two streams of the same type into one.

A common pattern: an event loop that handles both external messages
and internally-generated events (like async task completions):

```rust
use futures::channel::mpsc;
use futures::stream::StreamExt;

async fn run_event_loop(
    mut rx_external: mpsc::Receiver<Msg>,
) {
    // Create channel for loop to send messages to itself.
    let (tx_self, rx_self) = mpsc::channel::<Msg>(10);

    // Merge both streams.
    let mut rx_combined = futures::stream::select(rx_external, rx_self);

    while let Some(msg) = rx_combined.next().await {
        match msg {
            Msg::StartTask => {
                // Spawn async task that reports completion.
                let mut tx_self = tx_self.clone();
                tokio::spawn(async move {
                    do_work().await;
                    tx_self.send(Msg::TaskComplete).await.ok();
                });
            }
            Msg::TaskComplete => {
                // Handle completion sent from spawned task.
            }
        }
    }
}
```




## Networking and web




### … make a synchronous HTTP request?

### … configure a basic HTTP server?




## Text / unicode




## Terminal / CLI

### … set up a simple CLI parser with subcommands?

### … display colors in the terminal?

### … read line-based input from the terminal?

### … handle ctrl-?

Either use the [`ctrl`] crate or [`tokio::signal::ctrlc`].

todo say more w/ example



## System / OS

### … read environment variables?

### … work with a temporary file?

### … work with multiple files in a temporary directory?

### … copy a directory recursively?

There's no standard way to recursively copy a dir!

Here take this:

```rust
pub fn copy_dir_recursive(
    src: &std::path::Path,
    dst: &std::path::Path,
) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}
```

It's also at `rustmax::extras::copy_dir_recursive`.

Now doing it in a way that is mindful of the infinite curveballs
different OSes will throw at you via the filesystem …
well that's probably why `std` doesn't try.




## Testing

### … create a custom test harness?

### … create a custom table-based test harness?




## Build scripts

### … write build scripts ideomatically?

### … link to a native static library?

### … compile and link a C source file?




## FFI / interop

### … create Rust bindings to a C/C++ program?

## Procedural macros

