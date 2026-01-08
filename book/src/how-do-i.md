# How Do I … in Rust?


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




## Conveniences

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




## Error handling and debugging

### … handle errors simply and correctly?

### … structure errors in a public API?




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
use rand::{Rng, SeedableRng};
let mut rng = rand::rngs::StdRng::seed_from_u64(42);
let x: u32 = rng.gen();
```

### … generate a fast random number from a seed?

Use [`rand::rngs::SmallRng`] for performance-critical code where cryptographic security isn't required. This automatically selects a fast algorithm.

```rust
use rand::{Rng, SeedableRng};
let mut rng = rand::rngs::SmallRng::seed_from_u64(42);
let x: u32 = rng.gen();
```

### … generate a strong random number from a seed with stable algorithm?

Use [`rand_chacha::ChaCha12Rng`] when you need reproducible results across Rust versions and platforms with cryptographic security.

```rust
use rand::{Rng, SeedableRng};
let mut rng = rand_chacha::ChaCha12Rng::seed_from_u64(42);
let x: u32 = rng.gen();
```

### … generate a fast random number from a seed with stable algorithm?

Use [`rand_pcg::Pcg64`] for deterministic, fast random numbers that remain consistent across platforms and Rust versions.

```rust
use rand::{Rng, SeedableRng};
let mut rng = rand_pcg::Pcg64::seed_from_u64(42);
let x: u32 = rng.gen();
```

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

