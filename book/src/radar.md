# The Rustmax Radar

This is where we collect crates and tools of interest,
but that are not yet part of Rustmax.


## Crates

### Collections and algorithms

- [`bitvec`](https://docs.rs/bitvec).
  Operate directly on bits.
- [`dashmap`](https://docs.rs/dashmap).
  Concurrent hash map.
- [`ena`](https://docs.rs/ena).
  The union find algorithm.
- [`hashbrown`](https://docs.rs/hashbrown).
  The `std` hash map, usable in no-std, with a lower-level API.
- [`indexmap`](https://docs.rs/indexmap).
  Hash maps and sets that preserve insertion order.
- [`ndarray`](https://docs.rs/ndarray).
  N-dimensional arrays.
- [`petgraph`](https://docs.rs/petgraph).
  Graph data structures and algorithms.
- [`rangetools`](https://docs.rs/rangetools).
  Set operations on ranges.
- [`smallvec`](https://docs.rs/smallvec).
  The "small vector" optimization.
  There may be better / newer options.

### Hashing

- [`rustc-hash`](https://docs.rs/rustc-hash).
  The fast, non-cryptographic hasher used by `rustc`.
- [`fnv`](https://docs.rs/fnv).
  Another fast hasher, for small keys.

### Numbers

- [`num`](https://docs.rs/num).
  Rationals, complex numbers, and the rest of the `num` family.
- [`num-traits`](https://docs.rs/num-traits).
  Traits for writing code generic over numeric types.
- [`ordered-float`](https://docs.rs/ordered-float).
  Floats that implement `Ord` and `Hash`.

### Serialization

- [`borsh`](https://docs.rs/borsh).
  Fast and stable binary serialization.
- [`postcard`](https://docs.rs/postcard).
  Compact binary serde format, no-std friendly.
- [`ron`](https://docs.rs/ron).
  Rusty Object Notation, a serde format for config and data files.

### Errors

- [`eyre`](https://docs.rs/eyre).
  Sophisticated error handling.

### Language extensions

- [`bytemuck`](https://docs.rs/bytemuck).
  Safe casts between plain-old-data types and byte slices.
- [`derive-new`](https://docs.rs/derive-new).
  Derive the `new` function.
- [`enum-iterator`](https://docs.rs/enum-iterator).
  Iterate over variants.
- [`pin-project-lite`](https://docs.rs/pin-project-lite).
  Safe pin projection for hand-written futures.
- [`rustversion`](https://docs.rs/rustversion).
  Conditional compilation based on compiler version.
- [`scopeguard`](https://docs.rs/scopeguard).
  Like `defer`.

### Concurrency and runtime

- [`parking_lot`](https://docs.rs/parking_lot).
  Non-poisoning mutexes, etc.
- [`pollster`](https://docs.rs/pollster).
  Minimal `block_on` for running a future without an async runtime.
- [`stacker`](https://docs.rs/stacker).
  Manually-growable call stacks.
- [`backtrace-on-stack-overflow`](https://docs.rs/backtrace-on-stack-overflow).
  Nice missing feature, but code looks underdeveloped.

### Networking and databases

- [`tungstenite`](https://docs.rs/tungstenite).
  WebSockets.
- [`rustls`](https://docs.rs/rustls).
  TLS.
- [`boringtun`](https://docs.rs/boringtun).
  WireGuard.
- [`sqlx`](https://docs.rs/sqlx).
  Async SQL with compile-time checked queries.

### Cryptography and secrets

- [`sha3`](https://docs.rs/sha3).
  SHA-3 hashes.
- [`k256`](https://docs.rs/k256).
  secp256k1 elliptic curve.
- [`zeroize`](https://docs.rs/zeroize).
  Secret-zeroing cells.
- [`rpassword`](https://docs.rs/rpassword).
  Password input.

### Text

- [`encoding`](https://docs.rs/encoding),
  [`charset`](https://docs.rs/charset),
  [`codepage`](https://docs.rs/codepage),
  [`oem_cp`](https://docs.rs/oem_cp),
  [`icu_normalizer`](https://docs.rs/icu_normalizer),
  [`detone`](https://docs.rs/detone).
  Text encoding.
- [`unicode-xid`](https://docs.rs/unicode-xid).
  Unicode identifier character classes.
- [`html5ever`](https://docs.rs/html5ever),
  [`scraper`](https://docs.rs/scraper), and
  [`selectors`](https://docs.rs/selectors).
  HTML parsing and CSS selectors. html5ever is low-level;
  scraper provides a convenient DOM + selector API.

### Time

- [`time`](https://docs.rs/time).
  Another time crate.

### Logging and tracing

- [`tracing`](https://docs.rs/tracing).
  Structured, span-based diagnostics.
- [`tracing-subscriber`](https://docs.rs/tracing-subscriber).
  Collecting and formatting `tracing` output.
- [`tracing-tracy`](https://docs.rs/tracing-tracy).
  `tracing` integration with the Tracy profiler.

### Testing and benchmarking

- [`criterion`](https://docs.rs/criterion).
  Advanced benchmarking.
- [`divan`](https://docs.rs/divan).
  Simple, fast benchmarking with an attribute-based API.
- [`datatest-stable`](https://docs.rs/datatest-stable).
  Data-driven tests.

### Terminal

- [`console`](https://docs.rs/console) and
  [`dialoguer`](https://docs.rs/dialoguer).
  Pretty terminals.
- [`crossterm`](https://docs.rs/crossterm).
  Cross-platform terminal control and input events.
- [`ratatui`](https://docs.rs/ratatui).
  Seriously cool CLIs.

### Graphics and audio

- [`wgpu`](https://docs.rs/wgpu).
  Cross-platform GPU API based on WebGPU.
- [`raw-window-handle`](https://docs.rs/raw-window-handle).
  Interop between windowing and graphics crates.
- [`cpal`](https://docs.rs/cpal).
  Cross-platform audio I/O.
- [`rodio`](https://docs.rs/rodio).
  Cross-platform audio playback.

### System and platform

- [`nix`](https://docs.rs/nix).
  Safe bindings to Unix APIs.
- [`libloading`](https://docs.rs/libloading).
  Load dynamic libraries at runtime.
- [`getrandom`](https://docs.rs/getrandom).
  Random bytes from the operating system.
- [`memmap2`](https://docs.rs/memmap2).
  Memory-mapped files.
  Maintained fork of `memmap`.
- [`home`](https://docs.rs/home).
  Find the user's home directory.
- [`xdg`](https://docs.rs/xdg).
  XDG base directories.
- [`dotenvy`](https://docs.rs/dotenvy).
  Load environment variables from `.env` files.
  Maintained fork of `dotenv`.

### Embedding resources

- [`include_dir`](https://docs.rs/include_dir).
  Embed a directory tree in a binary.
- [`rust-embed`](https://docs.rs/rust-embed).
  Embedding of file system resources into binaries,
  with hot-reloading during development.


## Tools

- [`cargo-cache`](https://crates.io/crates/cargo-cache).
  Inspect and clean the `~/.cargo` registry caches and git checkouts.
- [`cargo-hack`](https://crates.io/crates/cargo-hack).
  Build and test with combinations of cargo features.
- [`cargo-llvm-cov`](https://crates.io/crates/cargo-llvm-cov).
  Code coverage.
- [`delta`](https://crates.io/crates/git-delta).
  Improved `git diff` plugin.
- [`flamegraph`](https://crates.io/crates/flamegraph)
  and [`inferno`](https://crates.io/crates/inferno).
  Flame graph profiling.
- [`hyperfine`](https://github.com/sharkdp/hyperfine).
  Command-line benchmarking.
- [`rust-analyzer`](https://rust-analyzer.github.io/).
  The Rust language server.
- [`skim`](https://crates.io/crates/skim).
  Fuzzy finder (`sk`), like fzf but in Rust.
- [`wasmtime-cli`](https://crates.io/crates/wasmtime-cli).
  WebAssembly runtime.


## Wanted

- gRPC
- wasm crates and tools, wasm-bindgen
- threadpool
- parser generator (pest?)
- alternative to bitflags
- gui stuff
  - winit, morphorm, css, iced vs egui
- i18n
- QUIC - either quinn or quiche
- HTTP3
- csv
- small string, smartstring
- rational numbers
- fixed-point, decimal numbers, rust-decimal


## Graveyard

These projects were once useful or notable,
but are now deprecated by others.

- [`lazy_static`](https://docs.rs/lazy_static).
  Use [`std::sync::LazyLock`](https://doc.rust-lang.org/std/sync/struct.LazyLock.html).
- [`static_assertions`](https://docs.rs/static_assertions).
  Use [inline const blocks](https://doc.rust-lang.org/reference/expressions/block-expr.html#const-blocks).
- [`num_cpus`](https://docs.rs/num_cpus).
  Use [`std::thread::available_parallelism`](https://doc.rust-lang.org/std/thread/fn.available_parallelism.html).
- [`once_cell`](https://docs.rs/once_cell).
  Use [`std::sync::OnceLock`](https://doc.rust-lang.org/std/sync/struct.OnceLock.html).
- [`memoffset`](https://docs.rs/memoffset).
  Use [`std::mem::offset_of!`](https://doc.rust-lang.org/std/mem/macro.offset_of.html).
- [`memmap`](https://docs.rs/memmap).
  Use [`memmap2`](https://docs.rs/memmap2).
- [`dotenv`](https://docs.rs/dotenv).
  Use [`dotenvy`](https://docs.rs/dotenvy).
