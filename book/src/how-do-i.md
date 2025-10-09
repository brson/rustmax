# How Do I … in Rust?


## Discovery

### … find a crate for a given purpose?

### … find the latest version of a crate?




## Project organization

### … init a Rustmax project from template?

```
cargo generate brson/rustmax
```


### … organize a Rust workspace?




## Conveniences

### … define "extension" methods on a type in another crate?

### … guarantee a trait is object-safe?

```
static_assertions::assert_obj_safe!(MyTrait);
```




## Error handling and debugging

### … handle errors simply and correctly?

### … structure errors in a public API?

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

